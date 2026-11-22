#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14149fd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141492e20; const P1: u64 = 0x1401fb320; const P2: u64 = 0x14307e970; const P3: u64 = 0x141833c0f;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0xfffffffffffffffu64) as u64; rbp = rcx; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rbx = 0x1; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = rdx.wrapping_sub(0x1); rax = (((rax) | (rbx))) as u64; r8 = r14; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = rbp.wrapping_add(0x18); rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); rax = rbx.wrapping_sub(0x1); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { rbx = rdi; bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r9 = rax; r9 = (((r9) & (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))))) as u64; r9 = ((r9).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r10.wrapping_add(r9.wrapping_mul(8)), rbx); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64), rbx); bb = 12; continue; }
            6 => { r8 = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r11 = (crate::simd_ops::ru64(r8)) as u64; fl_l = r11; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r11); rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 5; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r10.wrapping_add(r9.wrapping_mul(8)), rbx); bb = 12; continue; }
            12 => { rbx = rdi; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 12; continue; }
            15 => { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14149ff00, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14149ff48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1401fb320; const P2: u64 = 0x18; const P3: u64 = 0x30; const P4: u64 = 0x8; const P5: u64 = 0x1414a0058;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = rax; rdx = (((rdx) & (crate::simd_ops::ru64(rdi.wrapping_add(P3))))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P4))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P4), rax); bb = 7; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), rax); bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14149ffb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rsi = rdx; r14 = rcx; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(P0), rax); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r15 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = 0x28; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(1)) as u64; rsi = r15; fl_l = r15; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; fl_r = rbp; crate::simd_ops::wu64(r14.wrapping_add(P0), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0058(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; crate::simd_ops::lk_sub_u64(rcx.wrapping_add(P0), (1) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax, rsi); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a00d4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14149cae0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x1414a07a4; const P3: u64 = 0x141833c49; const P4: u64 = 0x14149e918; const P5: u64 = 0x14149d52c; const P6: u64 = 0x1418348a3; const P7: u64 = 0x1414a021f;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a00d4, funcinfo_rva: 0x36b843c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); r8 = ((rax.wrapping_add(0x8u64)) as u64) as u64; rdx = ((rax.wrapping_add(0x10u64)) as u64) as u64; rcx = ((rax.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0101); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rdi.wrapping_add(0xc0u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0111); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(0x190u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a012a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0132); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 2; continue; }
            2 => { fl_l = ((((rbp as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0191); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a01a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a01b0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a01ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a01e1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 17; continue; }
            17 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a01f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a021a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14149cb50; const P1: u64 = 0x141833c2d; const P2: u64 = 0x1414a07a4; const P3: u64 = 0x141833c49; const P4: u64 = 0x14149c840; const P5: u64 = 0x142646e00; const P6: u64 = 0x14149d5a8; const P7: u64 = 0x1418348a3; const P8: u64 = 0x1414a032c; const P9: u64 = 0x1414a0334;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0220, funcinfo_rva: 0x36b8408, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); r8 = ((rax.wrapping_add(0x8u64)) as u64) as u64; rdx = ((rax.wrapping_add(0x10u64)) as u64) as u64; rcx = ((rax.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0249); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rsi.wrapping_add(0xc0u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0259); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(0x150u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0272); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a027a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rsi.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0286); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r8 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rcx = ((rsi.wrapping_add(0x110u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a02b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x118u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x118u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a02e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { rcx = ((rsi.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a02fe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0309); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax & !0xffu64) | ((((rbp as u8)) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0327); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a032f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0338(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14149ca70; const P1: u64 = 0x141834d64; const P2: u64 = 0x141833c2d; const P3: u64 = 0x1414a07a4; const P4: u64 = 0x14149c840; const P5: u64 = 0x14149b9cc; const P6: u64 = 0x14149ff48; const P7: u64 = 0x141833c49; const P8: u64 = 0x142646e30; const P9: u64 = 0x142646e08; const P10: u64 = 0x142646e40; const P11: u64 = 0x14149e918; const P12: u64 = 0x14149bce4; const P13: u64 = 0x141834d30; const P14: u64 = 0x14149d48c; const P15: u64 = 0x1418348a3; const P16: u64 = 0x1414a0540;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0338, funcinfo_rva: 0x36b847c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x88u64)) as u64; r15 = (rcx) as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), ((rsi as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((xmm0 as u32)) as u32); r14 = ((rcx.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (r14) as u64); bb = 1; continue; }
            1 => { r8 = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x77u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a037a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x41u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a038f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rax) as u64); rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a03a2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 2; continue; }
            2 => { rcx = ((r15.wrapping_add(0x1d0u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a03ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((r15.wrapping_add(0x210u64)) as u64) as u64; rax = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rcx) as u64); bb = 4; continue; }
            4 => { rdi = ((r15.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rdi) as u64); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a03f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 5; continue; }
            5 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rbx = (crate::simd_ops::ru64(r12)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { r13 = ((r15.wrapping_add(0x110u64)) as u64) as u64; bb = 7; continue; }
            7 => { r8 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0424); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x118u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0442); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rsi) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a044d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rsi = (rsi & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u8 as u64)); bb = 11; continue; }
            11 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0464); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a046d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0487); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0495); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a049f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a04c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 22; continue; }
            21 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (rsi & !0xffu64) | ((((rbx as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), ((rbx as u8)) as u8); bb = 22; continue; }
            22 => { rdx = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x41u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a04f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x20u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0503); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a050d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } bb = 23; continue; }
            23 => { rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0533); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a053b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0544(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8; const P1: u64 = 0x14183acbc; const P2: u64 = 0x14149bd94; const P3: u64 = 0x1413b6610;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rbx = r9; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdi = rcx; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); fl_l = r9; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 4; continue; }
            4 => { r15 = ((rcx.wrapping_add(rdx)) as u64) as u64; rax = ((rsi.wrapping_add(rbx)) as u64) as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((rcx.wrapping_add(r8)) as u64) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = r15; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            8 => { r14 = r15; fl_l = r14; fl_r = rsi; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 10; continue; }
            9 => { r14 = rbx; bb = 10; continue; }
            10 => { r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = ((r15.wrapping_add(rbx)) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = r14; rdx = rsi; rcx = r15; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = ((r14.wrapping_add(rsi)) as u64) as u64; r8 = ((r8).wrapping_sub(r14)) as u64; rcx = ((r14.wrapping_add(r15)) as u64) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 12; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64)); r9 = rdx; rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a0626, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0628(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x141833c2d; const P2: u64 = 0x14149c840; const P3: u64 = 0x14149b924; const P4: u64 = 0x141833c49; const P5: u64 = 0x14149e918; const P6: u64 = 0x1414a0338; const P7: u64 = 0x142646e28; const P8: u64 = 0x142646df8; const P9: u64 = 0x14149bce4; const P10: u64 = 0x141834d30; const P11: u64 = 0x1418348a3; const P12: u64 = 0x1414a07a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0628, funcinfo_rva: 0x36b84d0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a064d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rbp = ((r14.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbp) as u64); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a066a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rsi = ((r14.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rsi) as u64); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0686); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x118u64))) as u64; rdi = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x158u64))) as u64; r8 = ((rdi.wrapping_add(0x10u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rcx = ((r14.wrapping_add(0x150u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a06bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rdi.wrapping_add(0x18u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a06d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a06e7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0700); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0707); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a071a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0723); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0730); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0736); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x58u64), (rax) as u64); rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0744); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a074d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a075d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x20u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a076c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a078b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0793); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a079b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a07a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P2), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P5), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx); rcx = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P6), rax); crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx); bb = 2; continue; }
            2 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a082c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x58; const P2: u64 = 0x142646e18; const P3: u64 = 0x8; const P4: u64 = 0x141833c49; const P5: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a086f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1401de2f0; const P2: u64 = 0x14183af0c; const P3: u64 = 0x1412dd770; const P4: u64 = 0x142669d50; const P5: u64 = 0x1411f4730; const P6: u64 = 0x142645c20; const P7: u64 = 0x142669d38; const P8: u64 = 0x1402034f0; const P9: u64 = 0x14148aef4; const P10: u64 = 0x142669c90; const P11: u64 = 0x1426467f0; const P12: u64 = 0x1414a09bd; const P13: u64 = 0x141834d30; const P14: u64 = 0x1434278a0; const P15: u64 = 0x140200450; const P16: u64 = 0x1412dd710; const P17: u64 = 0x142645ac0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 57>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0870, funcinfo_rva: 0x36b9110, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x1c8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((rsp.wrapping_sub(0xb0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), (rax) as u64); rdi = (rdx) as u64; rbx = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P4) as u64) as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08eb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a08fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a090a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0931); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0x80u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0942); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0x80u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x80u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsi.wrapping_add(0x53u64)) as u32) as u32 as u64; r8 = ((P10) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a097d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a09b7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a09be); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P14)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), ((rsi as u8)) as u8); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a09df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a09e9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a09f2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0a07); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e0u64))) as u64; rsp = ((rsp).wrapping_add(0x1b0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0a34(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669f80; const P1: u64 = 0x1414a17b4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a0a4b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0a4c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x142669da8; const P3: u64 = 0x10; const P4: u64 = 0x1418348b5; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x1414a0f10;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0a4c, funcinfo_rva: 0x36b9014, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r8) as u64; rsi = (rcx) as u64; rax = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rbx = ((rcx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::lk_and_u32(rbx, (0x0u32) as u32); rcx = ((rbx.wrapping_add(P0)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0a86); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), (rax) as u64); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P3), (0x0u64) as u64); r8 = ((rdi.wrapping_add(P0)) as u64) as u64; r9 = ((r8.wrapping_add(P0)) as u64) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0ab7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a3a00; const P1: u64 = 0x1414a0f10; const P2: u64 = 0x1414a0e80; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1414a0c55; const P5: u64 = 0x141834d30; const P6: u64 = 0x1414a0ca1; const P7: u64 = 0x14183492d; const P8: u64 = 0x141834d64; const P9: u64 = 0x142669fb8; const P10: u64 = 0x142669fd0; const P11: u64 = 0x1414a486c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0ad0, funcinfo_rva: 0x36b9200, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x50u64), (0x0u32) as u32); crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); rsi = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (rsi) as u64); rdx = (rsi) as u64; r13 = (0x2u32) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r13 } else { rdx }) as u64; r8 = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0b50); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0b6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { xmm0 = 0u128; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64); r8 = ((r15.wrapping_add(0x40u64)) as u64) as u64; r9 = ((r8.wrapping_add(0x8u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0ba4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rdx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm0) as u128); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rcx) as u64); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((r13 as u32)) as u32); rdx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x140u64), (rcx) as u64); rcx = ((rax.wrapping_add(0x148u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0c03); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { r15 = (0x1000u32) as u32 as u64; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rsi = ((((rsi as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { r14 = ((r14).wrapping_sub(rcx)) as u64; r14 = ((((r14) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((r14.wrapping_mul(8)) as u64) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0c4f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0c56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0c9b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0ca2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0xcu64), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 25; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(0x160u64)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0cc5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rdi)) as u64; rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0cd2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::lk_and_u64(rdx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdx.wrapping_add(0x10u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), (rbx) as u64); bb = 31; continue; }
            30 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0d2b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0d48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a16a0; const P1: u64 = 0x142669ea0; const P2: u64 = 0x141489d2c; const P3: u64 = 0x142669f80; const P4: u64 = 0x142669f10; const P5: u64 = 0x1414a1d54; const P6: u64 = 0x1414a1570;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0d48, funcinfo_rva: 0x36b9178, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0d64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); r8 = ((rdi.wrapping_add(0xd0u64)) as u64) as u64; crate::simd_ops::lk_and_u64(r8.wrapping_add(0x38u64), (0x0u64) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0d82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P3) as u64) as u64; crate::simd_ops::wu64(r8, (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x38u64), (r8) as u64); bb = 2; continue; }
            2 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rsi = ((rdi.wrapping_add(0x110u64)) as u64) as u64; crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rsi.wrapping_add(0x8u64), (0x0u64) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0dc1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0dd1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = ((((rbp as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u64(rax, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rax.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rsi, (rcx) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0e0f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0e22); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0e40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0e53); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdi.wrapping_add(0xc2u64), (0x1u8) as u8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0e74(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0e7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401eae50; const P1: u64 = 0x141834d64; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1402080d0; const P5: u64 = 0x1414a1fc0; const P6: u64 = 0x1414937f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0x1fffffffffffffffu64) as u64; rdi = r9; r15 = r8; r14 = rcx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rbx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rbx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = rax; bb = 8; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            7 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r14, rsi); rax = ((rbx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(P2), rsi); r8 = rsi; crate::simd_ops::wu64(r14.wrapping_add(P3), rax); rbx = (crate::simd_ops::ru64(r15)) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x40); rdi = ((rdi).wrapping_sub(rbx)) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((rsi.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(P2), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 9; continue; }
            9 => { return rax; }
            10 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a0fd2, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a0fd4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414a16a0; const P2: u64 = 0x142669ea0; const P3: u64 = 0xd0; const P4: u64 = 0x38; const P5: u64 = 0x141489d2c; const P6: u64 = 0x142669f80; const P7: u64 = 0x8; const P8: u64 = 0x142669ed8; const P9: u64 = 0x1414a0d48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a0fd4, funcinfo_rva: 0x36b906c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (0x110u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a0ff5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a100e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rdx = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), (rbx) as u64); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a102b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rdx.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(P4), (rdx) as u64); bb = 5; continue; }
            5 => { rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); bb = 7; continue; }
            6 => { rdi = (rbx) as u64; bb = 7; continue; }
            7 => { rax = (rdi) as u64; bb = 11; continue; }
            8 => { rcx = (0x120u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1063); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (rsi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a107a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; bb = 10; continue; }
            10 => { rax = (rbx) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1098(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x142669e68; const P2: u64 = 0x8; const P3: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a1098, funcinfo_rva: 0x36b90c4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (0x48u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a10af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdi = (rax) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rdi, (rax) as u64);
    rsi = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64);
    crate::simd_ops::lk_and_u64(rsi.wrapping_add(P3), (0x0u64) as u64);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a10e3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsi.wrapping_add(P3), (rax) as u64); }
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1134(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414a0870; const P2: u64 = 0x1414a6354;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = r11.wrapping_sub(0x18);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax);
    rcx = r8;
    rax = r11.wrapping_sub(0x28);
    crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a117c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x8; const P2: u64 = 0x1414a10fc; const P3: u64 = 0x1426467d0; const P4: u64 = 0x141833c1b; const P5: u64 = 0x1414a1201;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a117c, funcinfo_rva: 0x36b903c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; rcx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1199); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r9, (rax) as u64); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            2 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r9) as u64); rbx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r8 = ((P2) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a11d6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi, (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::lk_and_u32(rbx, (0x0u32) as u32); rcx = (0x6u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a11fc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1204(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x141833c2d; const P2: u64 = 0x1; const P3: u64 = 0x18; const P4: u64 = 0x1402049c0; const P5: u64 = 0x70; const P6: u64 = 0x1414a12a8; const P7: u64 = 0x141833c49; const P8: u64 = 0x1418348a3; const P9: u64 = 0x1414a12a4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a1204, funcinfo_rva: 0x36b909c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (rdx) as u64; rsi = (rcx) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1227); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdi = ((rax.wrapping_add(P2)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a123e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (0x2u32) as u32 as u64; bb = 5; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rsi) as u64); rdx = (rbp) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1259); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a126f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1289); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a129f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a12a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141833bfd; const P2: u64 = 0x141833c03; const P3: u64 = 0x14148bec8; const P4: u64 = 0x141834885; const P5: u64 = 0x141834897; const P6: u64 = 0x141833c1b; const P7: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = (((rax) ^ (rsp))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r9)) as u64; rbx = r9; r14 = r8; rsi = rdx; rbp = rcx; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x989680u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 5; continue; }
            4 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = rax; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x28); rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 1; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0x0; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 15; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            14 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a13b4, rcx, rsp); return rax; }
            17 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a13bc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a13c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a0fd4; const P1: u64 = 0x8; const P2: u64 = 0xc8; const P3: u64 = 0x14149b224; const P4: u64 = 0x1414a146f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a13c0, funcinfo_rva: 0x36b8fcc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((rdx as u32)) as u32 as u64; rbx = (rcx) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a13dc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P1), ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx, (rcx) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); bb = 5; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P1), ((rdx as u8)) as u8); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = (rcx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1452); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1459); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a146a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x140226ba0; const P5: u64 = 0x1401de2f0; const P6: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = rsp;
    crate::simd_ops::wu64(rax.wrapping_add(P0), rbx);
    crate::simd_ops::wu64(rax.wrapping_add(P1), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((r8 as u32)) as u32);
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P3), ((rdx as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x40);
    rdi = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rax = ((rdi as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rbx as u32);
    let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rcx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; }
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    if !((((fl_l as u32)) == ((fl_r as u32)))) { r8 = 0x4; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a14e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140226ba0; const P1: u64 = 0x1401de2f0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32);
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rdi = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rdi as u32)) as u32);
    rdi = (tmp_xchg) as u32 as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1518(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645468; const P1: u64 = 0x1426467e8; const P2: u64 = 0x1414a3f40; const P3: u64 = 0x14392bdd8; const P4: u64 = 0x142645460; const P5: u64 = 0x1414e5810;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a153b, rcx, rsp); return rax; }
            2 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P3, rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a156c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x40; const P3: u64 = 0x1414a0e80; const P4: u64 = 0x1414a0ad0; const P5: u64 = 0x1414a2350;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a1570, funcinfo_rva: 0x36b91ac, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (r8) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::lk_and_u64(r11.wrapping_sub(0x18u64), (0x0u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(r8.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); rcx = ((r8.wrapping_add(P2)) as u64) as u64; rax = ((r11.wrapping_sub(0x28u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a15bf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { r8 = (rdi) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a15ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a15d7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a15f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x142669dd0; const P5: u64 = 0x1414a17c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rsi = r8; rcx = 0x1c8; r14 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = 0x1; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P3), ((rdx as u32)) as u32); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P2), ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = rsp.wrapping_add(0x28); rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rbx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rdi, rax); rax = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1714(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669ae0; const P1: u64 = 0x8; const P2: u64 = 0x142669af8; const P3: u64 = 0x10; const P4: u64 = 0x14183488b; const P5: u64 = 0x60; const P6: u64 = 0x141833c33; const P7: u64 = 0x142669b88; const P8: u64 = 0xb0; const P9: u64 = 0xb8; const P10: u64 = 0xc0; const P11: u64 = 0xc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rdi = rdx; crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x3u32) as u32); rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rdx = 0x2; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; rax = P7; crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rcx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(P9), rcx); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P10), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P11), rcx); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a17b4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a185c; const P1: u64 = 0x1426467c8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a17c3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a17c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1414a1b50; const P3: u64 = 0x142669ba8; const P4: u64 = 0x1b0; const P5: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rbx = r8; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; r8 = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P4), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a185c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = rcx;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1868(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414a633c; const P2: u64 = 0x142669a68; const P3: u64 = 0x14183488b; const P4: u64 = 0x141833c33; const P5: u64 = 0x14183af0c; const P6: u64 = 0x1412dd770; const P7: u64 = 0x142669bf0; const P8: u64 = 0x1411f4730; const P9: u64 = 0x1402034f0; const P10: u64 = 0x14148aef4; const P11: u64 = 0x142669c90; const P12: u64 = 0x1426467f0; const P13: u64 = 0x1414a1a1e; const P14: u64 = 0x141834d30; const P15: u64 = 0x1434278a0; const P16: u64 = 0x140200450; const P17: u64 = 0x1412dd710; const P18: u64 = 0x142645ac0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a1868, funcinfo_rva: 0x36b88a0, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x1d8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0xc0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1c0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (rax) as u64); rbx = (r8) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a18a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::lk_and_u64(r14, (0x0u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsi.wrapping_add(0x30u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsi.wrapping_add(0x40u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsi.wrapping_add(0x42u64), ((rax as u8)) as u8); rcx = ((rsi.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a18e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsi.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a18f3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsi.wrapping_add(0xd8u64)) as u64) as u64; rbx = (0x2u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1907); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsi.wrapping_add(0x128u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1916); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a192c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a193b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1956); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1967); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a198e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a199f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0x90u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x21u32) as u32 as u64; r8 = ((P11) as u64) as u64; rdx = ((r9.wrapping_sub(0x20u64)) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a19de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a18); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a1f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(P15)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a4a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a53); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), (0xea60u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), (0x7530u64) as u64); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a1a79); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x1c0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1aa8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183488b; const P1: u64 = 0x48; const P2: u64 = 0x141833c33; const P3: u64 = 0x98; const P4: u64 = 0xa0; const P5: u64 = 0x8; const P6: u64 = 0x10; const P7: u64 = 0xa8; const P8: u64 = 0xb0; const P9: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = 0x2; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P3), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P4), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P9), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669b70; const P1: u64 = 0x141833c33; const P2: u64 = 0x1414a1aa8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = rdx; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rdi = rcx; crate::simd_ops::wu16(rcx.wrapping_add(0xcu64), ((rbp as u16)) as u16); rsi = r8; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); rdx = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rbp); rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbp); rcx = rdi.wrapping_add(0x88); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x148u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x150u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x158u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x140u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x160u64), rdi); crate::simd_ops::wu16(rdi.wrapping_add(0x168u64), ((rbp as u16)) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbx); fl_l = rbx; fl_r = (0x2u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rsi = ((((rsi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1c5c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426680a0; const P1: u64 = 0x8; const P2: u64 = 0x14183ac92; const P3: u64 = 0x142669a98;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = rdx;
    rcx = P0;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rbx, rcx);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1c98(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426680a0; const P1: u64 = 0x8; const P2: u64 = 0x14183ac92; const P3: u64 = 0x142669a98;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8);
    rax = rdx;
    rcx = P0;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rbx, rcx);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(P0), rcx); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx); crate::simd_ops::wu64(rdx, rcx); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1d54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14392bdd0; const P1: u64 = 0x1414a1518; const P2: u64 = 0x14392bdd8; const P3: u64 = 0x8; const P4: u64 = 0x1414a1ce0; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x1414a4b70; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rdx = rsp.wrapping_add(0x48); rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P3), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P5), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P8), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P9), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P10), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P11), (0x0u64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::lk_and_u16(rbx.wrapping_add(0x58u64), (0x0u16) as u16); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1ddc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669fb8; const P1: u64 = 0x8; const P2: u64 = 0x141834939; const P3: u64 = 0x10; const P4: u64 = 0xc; const P5: u64 = 0x142669b18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_add(0x160u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = P5; crate::simd_ops::wu64(rdi, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1e4c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x38; const P2: u64 = 0x20; const P3: u64 = 0xc1; const P4: u64 = 0x142669bc0; const P5: u64 = 0xbc; const P6: u64 = 0x1418348df; const P7: u64 = 0x70; const P8: u64 = 0x141834891; const P9: u64 = 0x141833c27; const P10: u64 = 0x10; const P11: u64 = 0x1418348af;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; }
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = P4;
    crate::simd_ops::wu64(rbx, rax);
    if (((fl_l as u8)) == ((fl_r as u8))) { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P10)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1ec4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669f10; const P1: u64 = 0x110; const P2: u64 = 0x1414a48dc; const P3: u64 = 0x118; const P4: u64 = 0x8; const P5: u64 = 0xc; const P6: u64 = 0x1414a1e4c; const P7: u64 = 0x1414a4194;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            6 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a1f3a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1f3c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669ba8; const P1: u64 = 0x80; const P2: u64 = 0x78; const P3: u64 = 0x1414a3c54; const P4: u64 = 0x8; const P5: u64 = 0x178; const P6: u64 = 0x38; const P7: u64 = 0x20; const P8: u64 = 0x1414a21cc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rdi = ((rbx.wrapping_add(P5)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u64(rdi.wrapping_add(P6), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 6; continue; }
            6 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a1fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P3), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a2021, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418348f7; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0x28; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a207c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0; const P1: u64 = 0x141834921; const P2: u64 = 0x1418348fd; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x141834d30; const P7: u64 = 0x28; const P8: u64 = 0x8; const P9: u64 = 0x1418348af; const P10: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a2099, rcx, rsp); return rax; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P5)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = r8; bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P7), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a20fc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x48; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a211c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x20; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l; }
    rdx = 0x40;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2158(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x141834d30; const P4: u64 = 0x10; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a21b6, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a21cc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669b70; const P1: u64 = 0x141834d30; const P2: u64 = 0x141833c27; const P3: u64 = 0x141834891; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rbp = ((((rbp as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = rcx; fl_r = (0x2u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 6; continue; }
            6 => { rcx = r8; bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x148u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x150u64), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x158u64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rsi = rdi.wrapping_add(0x88); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a2307, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2308(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x50; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x141834d30; const P4: u64 = 0x48; const P5: u64 = 0x20; const P6: u64 = 0x14183490f; const P7: u64 = 0x18; const P8: u64 = 0x8; const P9: u64 = 0xc; const P10: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P7), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P9), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a2416, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2418(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x1414a0e74; const P3: u64 = 0x10; const P4: u64 = 0xc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; bb = 3; continue; }
            2 => { rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdx.wrapping_add(P0), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a24dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183491b; const P1: u64 = 0x14183490f; const P2: u64 = 0x1414a486c; const P3: u64 = 0x141489d2c; const P4: u64 = 0x142669df8; const P5: u64 = 0x1414a345c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = rcx; crate::simd_ops::lk_and_u32(rax.wrapping_add(0x8u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0xc8u64), (0x0u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), (0x1u32) as u32); rcx = rsp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = rax; rbx = (rbx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = rsp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((((r8 as u32)) | (0xffffffffu32))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x90u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rax = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = rsp.wrapping_add(0x58); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a26cc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x142669af8; const P4: u64 = 0x60; const P5: u64 = 0x141833c27; const P6: u64 = 0x18; const P7: u64 = 0x141834891; const P8: u64 = 0x142669ae0; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = P3; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = P8; crate::simd_ops::wu64(rbx, rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0xd0; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2778(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669fd0; const P1: u64 = 0x1414a1ddc; const P2: u64 = 0x141834d30;
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
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a27ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669dd0; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x1c8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a28ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c27; const P1: u64 = 0xd8; const P2: u64 = 0x90; const P3: u64 = 0x141834891; const P4: u64 = 0x48; const P5: u64 = 0x38; const P6: u64 = 0x1426467c8; const P7: u64 = 0x18; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x128u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a28f4, rcx, rsp); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x178; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a292c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669af8; const P1: u64 = 0x141833c27; const P2: u64 = 0x18; const P3: u64 = 0x141834891; const P4: u64 = 0x142669ae0; const P5: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x60u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = P4;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc0; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2984(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418348f7; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2a14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a21cc; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x170; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2a48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426680a0; const P1: u64 = 0x14183ac8c; const P2: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x18; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2a8c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a13c0; const P1: u64 = 0x1414a1204; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x1401e3410; const P4: u64 = 0x1418348e5; const P5: u64 = 0x14149b224; const P6: u64 = 0x1414a2bb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a2a8c, funcinfo_rva: 0x36b89c4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rsi = (rcx) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rcx) as u64); rax = ((rbp.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); r8 = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2ab8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2aed); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 1; continue; }
            7 => { crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), (0x1u8) as u8); rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b05); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b11); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b20); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = ((rsi.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax, ((rdi as u8)) as u8); rdi = (rdi & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rsi.wrapping_add(0x42u64)) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), (0x1u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b41); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = ((rbp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b4d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rcx = ((rsi.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b5c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0xc8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b89); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r9)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2b9b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((r8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x58u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rcx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2baf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2bb8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x141833bfd; const P3: u64 = 0x141833c03; const P4: u64 = 0x1414a2a8c; const P5: u64 = 0x14183af0c; const P6: u64 = 0x1412dd770; const P7: u64 = 0x142669cf0; const P8: u64 = 0x1411f4730; const P9: u64 = 0x1402034f0; const P10: u64 = 0x14148aef4; const P11: u64 = 0x142669c90; const P12: u64 = 0x141834d30; const P13: u64 = 0x140200450; const P14: u64 = 0x1412dd710; const P15: u64 = 0x142645ac0; const P16: u64 = 0x1402049c0; const P17: u64 = 0x1401de2f0; const P18: u64 = 0x1414a6644; const P19: u64 = 0x14148bec8; const P20: u64 = 0x141834885; const P21: u64 = 0x141834897; const P22: u64 = 0x1401e3410; const P23: u64 = 0x141833c49; const P24: u64 = 0x1426467f0; const P25: u64 = 0x1418348a3; const P26: u64 = 0x1414a2f6a; const P27: u64 = 0x141833c1b; const P28: u64 = 0x1414a2f7d;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a2bb8, funcinfo_rva: 0x36b897c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x228, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x128u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), (rax) as u64); r14 = (rcx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r13 as u32)) as u32); rcx = ((rcx).wrapping_add(0xd8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((r13 as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c0b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c1d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c25); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rbx) as u64 as u64; fl_r = (0x989680u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 5; continue; }
            4 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rax) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rbx = ((rax.wrapping_add(rcx)) as u64) as u64; bb = 5; continue; }
            5 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c58); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c65); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c74); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2c8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2ca0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x44u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x44u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2cc3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2cd4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r13 = ((((r13 as u32)) | (0x1u32))) as u32 as u64; rax = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x3au32) as u32 as u64; r8 = ((P11) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d15); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((rdi) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 35; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d53); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::lk_and_u64(rbp.wrapping_add(0xd0u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), (0xfu64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xc0u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d77); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rbp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d81); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rbp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d8a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P15)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d90); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2d98); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (0x989680u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 16; continue; }
            15 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rax) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } fl_l = ((rcx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = (0x431bde82d7b634dbu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(18u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2dfb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r15 = (rax) as u64; rsi = ((r14.wrapping_add(0x41u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e0a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; bb = 17; continue; }
            17 => { rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rdi)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 18; continue; }
            18 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e20); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e2d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e35); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rbx) as u64 as u64; fl_r = (0x989680u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 22; continue; }
            21 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rax) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 23; continue; }
            23 => { if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 33; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rcx) as u64); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xe0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e8a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2e96); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { r8 = ((rbp.wrapping_add(0xe0u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((r14.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2eb3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 27; continue; }
            27 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 28; continue; }
            28 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdi)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2edf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; bb = 32; continue; }
            30 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2eee); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 33; continue; }
            31 => { crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2efd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; bb = 32; continue; }
            32 => { rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f0a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); bb = 33; continue; }
            33 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f15); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f27); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x200u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f5c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 36; continue; }
            36 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f65); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P26, rcx, rsp); return rax; }
            37 => { rcx = (0x4u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f70); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P27) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 38; continue; }
            38 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a2f78); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P28, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a2f8c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x1401e3410; const P4: u64 = 0x14148b150; const P5: u64 = 0x141833bfd; const P6: u64 = 0x141833c03; const P7: u64 = 0x1402049c0; const P8: u64 = 0x1414a6644; const P9: u64 = 0x1418348e5; const P10: u64 = 0x14148bec8; const P11: u64 = 0x141834885; const P12: u64 = 0x141834897; const P13: u64 = 0x141833c15; const P14: u64 = 0x141833c49; const P15: u64 = 0x141833c1b; const P16: u64 = 0x1418348a3;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rsi = rdx; r15 = rcx; r13 = rcx.wrapping_add(0x128); rcx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 48; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x1u8) as u8); r12 = r15.wrapping_add(0x40); rcx = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); rcx = rbp.wrapping_sub(0x31); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rbp.wrapping_sub(0x19); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (((crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; rax = (0x7fffffffffffffffu64) as u64; fl_l = rbx; fl_r = rbx; rbx = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { rax } else { rbx }) as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 3; continue; }
            3 => { if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x989680u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 7; continue; }
            6 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = rax; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rdi) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rdi = rax; rax = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdx = r15.wrapping_add(0x20); rcx = rbp.wrapping_add(0xf); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); bb = 9; continue; }
            8 => { r12 = r15.wrapping_add(0x40); bb = 9; continue; }
            9 => { rcx = r15; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 10; continue; }
            10 => { rcx = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r12 = rax; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x1u8) as u8); rcx = rbp.wrapping_sub(0x29); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r15.wrapping_add(0x41); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((r14 as u8)) as u8); rcx = r15.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = r15.wrapping_add(0x42); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = r14; fl_r = (0x989680u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 16; continue; }
            15 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r14) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = rax; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r14) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 17; continue; }
            17 => { if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 26; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); rdx = rbp.wrapping_sub(0x9); rcx = rbp.wrapping_add(0x17); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r13; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 47; continue; } bb = 19; continue; }
            19 => { r8 = rbp.wrapping_add(0x17); rdx = r13; rcx = r15.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 21; continue; }
            21 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 22; continue; }
            22 => { rcx = r15; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r12)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x1u8) as u8); rcx = rbp.wrapping_sub(0x21); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r15.wrapping_add(0x41); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((r14 as u8)) as u8); rcx = r15.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; r14 = (r14 & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); bb = 28; continue; }
            26 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 28; continue; }
            27 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            28 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r12 = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = r12; fl_r = (0x989680u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 31; continue; }
            30 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r12) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = rax; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r12) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { rdi = ((rdi).wrapping_sub(rax)) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { r12 = r15.wrapping_add(0x40); bb = 2; continue; }
            33 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x39u64), (0x0u8) as u8); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 34; continue; }
            34 => { rdi = 0; fl_l = 0; fl_r = 0; rcx = r15.wrapping_add(0x42); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r12 = rax; rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rax)) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; bb = 38; continue; }
            35 => { rcx = ((rdi as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7fu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r12)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; bb = 38; continue; }
            38 => { if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x39u64), (0x0u8) as u8); rcx = rbp.wrapping_sub(0x39); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r15.wrapping_add(0x42); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((rdi as u8)) as u8); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 46; continue; } bb = 40; continue; }
            40 => { rax = (((rsi) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            42 => { fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 46; continue; }
            44 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (0x20c49ba5e353f7cfu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rbx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); bb = 46; continue; }
            46 => { rcx = r13; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | ((((r14 as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            47 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3326, rcx, rsp); return rax; }
            48 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a332e, rcx, rsp); return rax; }
            49 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3336, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3338(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a2bb8; const P1: u64 = 0x1414a117c; const P2: u64 = 0x38; const P3: u64 = 0x30; const P4: u64 = 0x1426467c8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rax = P0; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rax); r8 = r11.wrapping_add(0x8); rdx = r11.wrapping_add(0x10); rcx = r11.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm1) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a337f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1418348e5; const P2: u64 = 0x1418344f3; const P3: u64 = 0x30; const P4: u64 = 0x1418344f9; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x141833c1b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x48u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(P3), (xmm0) as u128); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            6 => { rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a33f3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a33fe, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a3409, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a340c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x141834d30; const P4: u64 = 0x141833c09;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            3 => { rdx = (0x40u32) as u32 as u64; rcx = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a345c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x141834909; const P2: u64 = 0x20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rdx; rdi = rcx; rax = r11.wrapping_sub(0x48); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rax); crate::simd_ops::lk_and_u64(r11.wrapping_sub(0x10u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x48); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a34cc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14149b068; const P1: u64 = 0x141833c09;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a34cc, funcinfo_rva: 0x36b9138, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x38, conts: &[0x1414a3504] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x108u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a34e8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a34e8, ({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1414a3504 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x1414a34cc) } }; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a34fe); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a34fe, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1414a3504 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x1414a34cc) } }; bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a350a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a350a, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1414a3504 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x1414a34cc) } }; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3514(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x1418348f7; const P3: u64 = 0x141834d30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x28;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3548(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x14149dd2c; const P3: u64 = 0x141834933; const P4: u64 = 0x141833c49; const P5: u64 = 0x14183489d; const P6: u64 = 0x141489d2c; const P7: u64 = 0x142669f48; const P8: u64 = 0x1414a4788; const P9: u64 = 0x1418348a3; const P10: u64 = 0x1414a370f; const P11: u64 = 0x1414a3717;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a3548, funcinfo_rva: 0x36b8f9c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); rsi = (rsi & !0xffu64) | ((((r8 as u8)) as u8 as u64)); rbx = (rbx & !0xffu64) | ((((rdx as u8)) as u8 as u64)); rdi = (rcx) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3588); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { r15 = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; rdx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a35b1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x4u32) as u32); rbx = (0x1u32) as u32 as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { (r15 as u32) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); rcx = ((rdi.wrapping_add(0x160u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3606); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3610); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1b0u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3636); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 23; continue; }
            15 => { rsi = ((rdi.wrapping_add(0x88u64)) as u64) as u64; rcx = ((rsi.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3649); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rsi.wrapping_add(0xb0u64), ((r15 as u32)) as u32); bb = 18; continue; }
            18 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3669); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3672); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x18u64), (0x0u64) as u64); rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3687); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rdi) as u64); rax = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); bb = 21; continue; }
            21 => { rdx = (0x10u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a36b0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a36cc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 23; continue; }
            23 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 25; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a36d8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a370a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            27 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3712); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3718(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a0f10; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414a0a4c; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1414a3831; const P5: u64 = 0x141834d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a3718, funcinfo_rva: 0x36b8d20, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm0) as u128); crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x8u64), (0x0u64) as u64); r8 = ((rcx.wrapping_add(0x148u64)) as u64) as u64; r9 = ((r8.wrapping_add(0x8u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a376d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (0x48u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3778); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3790); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; bb = 3; continue; }
            2 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rcx) as u64); rax = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((r8 as u8)) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a37bd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rsi) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rsi = ((((rsi as u32)) | (0xffffffffu32))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a37df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a37f2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a382b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3832); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | ((((rdi as u8)) as u8 as u64)); r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a39e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1414a0a34;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a39ff, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a15f0; const P1: u64 = 0x1414a4390;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a3a00, funcinfo_rva: 0x36b8f44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (r14) as u64); r8 = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3a65); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((((r15 as u32)) | (0xffffffffu32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3a8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3aa0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3ace); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3ae3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { fl_l = (r14) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3b10); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3b32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a3b47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x38; const P2: u64 = 0x20; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rsi = (rsi & !0xffu64) | ((((rdx as u8)) as u8 as u64));
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rdi; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; }
    fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3c3c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x18; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3c54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x141834d30; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x140226ba0; const P4: u64 = 0x141833c49; const P5: u64 = 0x1414a1470; const P6: u64 = 0x141834903; const P7: u64 = 0x1414a14e4; const P8: u64 = 0x1418348d3; const P9: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = rcx.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = rcx; rcx = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 12; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rdx); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rax); bb = 11; continue; }
            11 => { rdx = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rsi = rdi.wrapping_add(0x10); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x2u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 16; continue; }
            15 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = rdi.wrapping_add(0x10); bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 17; continue; }
            17 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 18; continue; }
            18 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 19; continue; }
            19 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 20; continue; }
            20 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 22; continue; }
            22 => { rdx = 0x2; rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 23; continue; }
            23 => { rbx = rdi.wrapping_add(0x60); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 24; continue; }
            24 => { bb = 26; continue; }
            25 => { rdx = rbx; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3da3, rcx, rsp); return rax; }
            30 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3dab, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3dac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3dc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3de8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x141833c09;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3e07, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3e14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1414a486c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((((r8 as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l;
    r8 = ((fl_l as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3e28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x70; const P2: u64 = 0x1414a44ac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a2418;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3e7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x170; const P2: u64 = 0x141833c2d; const P3: u64 = 0x8; const P4: u64 = 0x141833c49; const P5: u64 = 0x88; const P6: u64 = 0x48; const P7: u64 = 0xb0; const P8: u64 = 0x14183489d; const P9: u64 = 0x70; const P10: u64 = 0x1414a44ac; const P11: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8); rdi = rcx; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = rbx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            3 => { crate::simd_ops::wu32(rdi.wrapping_add(P3), (0x3u32) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = ((rdi.wrapping_add(P5)) as u64) as u64; rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P7))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsi.wrapping_add(P7), (0x2u32) as u32); bb = 6; continue; }
            6 => { rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; crate::simd_ops::lk_and_u64(rdi.wrapping_add(P9), (0x0u64) as u64); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3f37, rcx, rsp); return rax; }
            10 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3f3f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3fa4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x1414a48dc; const P2: u64 = 0x14149ad24; const P3: u64 = 0x1414a4194;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = (rdi & !0xffu64) | ((((rdx as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (rdx & !0xffu64) | ((((rdi as u8)) as u8 as u64)); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a3fdf, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a3ff8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834903; const P1: u64 = 0x10; const P2: u64 = 0x1414a1470; const P3: u64 = 0x60; const P4: u64 = 0x141833c2d; const P5: u64 = 0xb0; const P6: u64 = 0x141833c49; const P7: u64 = 0x18; const P8: u64 = 0x14183489d; const P9: u64 = 0x8; const P10: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = ((rax as u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = ((rbx as u32)) as u32 as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = 0x3; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P9), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a4095, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4098(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x38; const P2: u64 = 0x141489d2c; const P3: u64 = 0x1414a1098; const P4: u64 = 0x20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4098, funcinfo_rva: 0x36b8f04, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rdi = (rcx) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), (rcx) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (rbx) as u64); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r11.wrapping_sub(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a40d3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a40e7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a40f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi.wrapping_add(P1), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4119); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4194(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142669b28; const P1: u64 = 0x1414a1c98; const P2: u64 = 0x1437fce80; const P3: u64 = 0x14183acce;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = P0; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = P2; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a41ba, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a41bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141489d2c; const P1: u64 = 0x142669fe8; const P2: u64 = 0x1414a4098; const P3: u64 = 0x141834945; const P4: u64 = 0x14183494b; const P5: u64 = 0x1414a3e7c; const P6: u64 = 0x141833c09;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a41bc, funcinfo_rva: 0x36b92f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = (rcx) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x1u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a41e0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rax) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rax) as u64); bb = 2; continue; }
            2 => { rax = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4213); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a422f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x1u64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rax = ((rbp.wrapping_add(0x7u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_add(0x160u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4251); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3fu64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4263); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a426c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3fu64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbp.wrapping_add(0x7u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4288); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x3fu64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rdx = (rdx & !0xffu64) | ((((rdi as u8)) as u8 as u64)); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4297); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a42af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a42b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0; const P1: u64 = 0x140226ba0; const P2: u64 = 0x141833c2d; const P3: u64 = 0x141834d64; const P4: u64 = 0x141833c49; const P5: u64 = 0x1414a3ff8; const P6: u64 = 0x1418348a3; const P7: u64 = 0x1414a438c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a42b8, funcinfo_rva: 0x36b8a58, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rcx = ((rdx.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a42d4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a42e1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rbx = (0x1u32) as u32 as u64; crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xb8u64), (rdi) as u64); rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a42fc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4314); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a432a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, (rsi) as u64); fl_l = (crate::simd_ops::lk_and_u64(rax.wrapping_add(0x8u64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 6; continue; }
            5 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), (rcx) as u64); bb = 9; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), (rcx) as u64); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4366); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4372); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4387); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x141834d64; const P3: u64 = 0x1414a1714; const P4: u64 = 0x80; const P5: u64 = 0x78; const P6: u64 = 0x1414a42b8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4390, funcinfo_rva: 0x36b8d58, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rax) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (r8) as u64); fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (0xd0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a43cd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a43e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (rax) as u64); rdx = (rax) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a43f7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a441d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4438); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4448(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412dfdc0; const P1: u64 = 0x1; const P2: u64 = 0x1414a14e4; const P3: u64 = 0x8; const P4: u64 = 0x141498528; const P5: u64 = 0x14149b40c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a447b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a447c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a44ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a46b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a44ac, funcinfo_rva: 0x36b8df0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a44c7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = (r8 & !0xffu64) | ((((r9 as u8)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((r9 as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a44f9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4513); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((r10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4521); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            6 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a452d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rbx = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4551); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4564); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4574(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14392bdd0; const P1: u64 = 0x1414a1518; const P2: u64 = 0x14392bdd8; const P3: u64 = 0x8; const P4: u64 = 0x142669ab0; const P5: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdx; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            2 => { rdx = rsp.wrapping_add(0x50); rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P3), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); r8 = rdi; rdx = rsi; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(P4)) as u64; rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a463c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141833c49; const P2: u64 = 0x1414a34cc; const P3: u64 = 0x141833c2d; const P4: u64 = 0x1412dfa60; const P5: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rcx = 0x1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a469b, rcx, rsp); return rax; }
            7 => { rcx = 0x24; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a46a6, rcx, rsp); return rax; }
            8 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a46ae, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a46b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183492d; const P2: u64 = 0x24; const P3: u64 = 0x10; const P4: u64 = 0x141489d2c; const P5: u64 = 0x142669e30; const P6: u64 = 0x1414a4788; const P7: u64 = 0x20; const P8: u64 = 0x1414a486c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a46b0, funcinfo_rva: 0x36b8e24, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rdi = (rdx) as u64; rsi = (rcx) as u64; rcx = ((rcx).wrapping_add(0x160u64)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a46e0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (0x10u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbx as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4707); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); bb = 5; continue; }
            5 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4732); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4750); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            7 => { r8 = ((rbx as u32)) as u32 as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a475e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4788(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x38; const P2: u64 = 0x1414a340c; const P3: u64 = 0x1414a4574;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4788, funcinfo_rva: 0x36b8b50, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = (rcx) as u64; rcx = (0x40u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a47a1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::lk_and_u64(rax.wrapping_add(P1), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a47cc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { bb = 5; continue; }
            4 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { r8 = ((rsi as u32)) as u32 as u64; rdx = (rbx) as u64; rcx = ((P2) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a47f8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xa8; const P2: u64 = 0x1414a447c; const P3: u64 = 0x142669ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x1; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rbx; rdx = P2; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            5 => { rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a486c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a47f8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rcx = ((rcx).wrapping_add(0x88u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            2 => { bb = 1; continue; }
            3 => { bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a48bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x1414a48dc; const P2: u64 = 0x1414a4194;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a48da, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a48dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x1418348d3; const P2: u64 = 0x141833c49; const P3: u64 = 0x1418348a3; const P4: u64 = 0x1414a0e7c; const P5: u64 = 0x1414a4448;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = rcx.wrapping_add(0x88); r14 = rsi.wrapping_add(0x48); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { bb = 4; continue; }
            3 => { rdx = r14; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = r14; bb = 18; continue; }
            6 => { rsi = rcx.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); r14 = rsi.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rdx = r14; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 10; continue; }
            10 => { rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xdu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { bb = 16; continue; }
            15 => { rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 17; continue; }
            17 => { rcx = rsi; bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rcx = rdi.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x4; rbx = (rbx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a49f2, rcx, rsp); return rax; }
            24 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a4a00, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4a04(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418348a9; const P1: u64 = 0x1418348eb;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4a24(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x141833c49; const P2: u64 = 0x1414a41bc; const P3: u64 = 0x14183493f; const P4: u64 = 0x1418348a3; const P5: u64 = 0x1414a4ad2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4a24, funcinfo_rva: 0x36b9250, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[0x1414a4aa3, 0x1414a4ac7, 0x1414a4ac9] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rax = ((rcx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rax)) as u64; rsi = ((rbx.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4a4e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0x1u32) as u32); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4a6f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4a94); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            6 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4a9d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a4a9d, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x1414a4aa3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 7; continue; } 0x1414a4ac7 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 9; continue; } 0x1414a4ac9 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x1414a4a24) } }; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rcx).wrapping_add(0x160u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4ab2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { bb = 7; continue; }
            10 => { bb = 7; continue; }
            11 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4acd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4ad4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414a3514; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x1418348f1; const P7: u64 = 0x1414a2984; const P8: u64 = 0x142669ab8; const P9: u64 = 0x140fd8920; const P10: u64 = 0x1437febd0; const P11: u64 = 0x14183acce;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rcx = 0x28; rdi = r8; rsi = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_and_u64(rax, (0x0u64) as u64)) as u64; fl_r = fl_l; rax = P1; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rbx); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); bb = 3; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = P8; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = P10; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a4b6f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834927; const P1: u64 = 0x141834915; const P2: u64 = 0x14183491b;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x1u64) as u64;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4bb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140207ce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    r9 = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rdx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4be4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x58; const P3: u64 = 0x141834d64; const P4: u64 = 0x18; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x20; const P8: u64 = 0x140203a40; const P9: u64 = 0x1414a50e4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4be4, funcinfo_rva: 0x36b949c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rsi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P1), (rsi) as u64); rcx = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4c0f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P0), (rax) as u64); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rcx, (rsi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P0), (rsi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P1), (rsi) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), (rax) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(P4)))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4c4e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rbx.wrapping_add(P1)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4c6b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x2u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(r8, rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x4u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4cc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x140205190; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    xmm0 = 0u128;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64);
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_r = (0x10u64) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0x5u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140207d80; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = ((rdx.wrapping_add(P0)) as u64) as u64;
    rbx = rdx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = rcx;
    r8 = rbx;
    rcx = rdi;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    rdx = (crate::simd_ops::ru64(r9)) as u64;
    rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P4), (0x6u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8, rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x3u8) as u8); return rax; }
            1 => { crate::lift_ops::trap(0x80000003, 0x1414a4d71, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, 0x1414a4d72, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, 0x1414a4d73, rcx, 0); return rax; }
            4 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx)) as u8 as u64)); r8 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(r8, ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x0u8) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4d74(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx)) as u8 as u64));
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(r8, ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4d84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffa1; const P4: u64 = 0x1414a4be4; const P5: u64 = 0x48; const P6: u64 = 0x140205190; const P7: u64 = 0x1414a6f14;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a4d84, funcinfo_rva: 0x36b94d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rax.wrapping_add(P3)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; rax = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); r8 = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4dbb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm0) as u128); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rbx) as u64); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { rcx = ((rbp.wrapping_add(0x37u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4de8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rax) as u64); r8 = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a4e03); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4e24(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu64(rcx.wrapping_add(P0), r8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx; r8 = rbx; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a4ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a4d00; const P1: u64 = 0x1414a4cc4; const P2: u64 = 0x1414a4cb0; const P3: u64 = 0x1414a4d60; const P4: u64 = 0x1414a4ca0; const P5: u64 = 0x1414a4c90; const P6: u64 = 0x1414a4d74; const P7: u64 = 0x140207d80; const P8: u64 = 0x1414a4bb4; const P9: u64 = 0x140205010; const P10: u64 = 0x140208e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; fl_l = rcx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 3; continue; }
            3 => { rdx = r8; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            11 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            14 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            15 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            16 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x7u8) as u8); bb = 30; continue; }
            17 => { fl_l = rcx; fl_r = (0x9u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 18; continue; }
            18 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 19; continue; }
            19 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 20; continue; }
            20 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 21; continue; }
            21 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (rdx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xeu8) as u8); bb = 30; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xdu8) as u8); bb = 30; continue; }
            25 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xcu8) as u8); bb = 30; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xbu8) as u8); bb = 30; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xau8) as u8); bb = 30; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x9u8) as u8); bb = 30; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; r9 = r8.wrapping_add(0x8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r9)) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x8u8) as u8); bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a50e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140204cc0; const P1: u64 = 0x140206e10; const P2: u64 = 0x141834d64; const P3: u64 = 0x1414a5424; const P4: u64 = 0x140206290; const P5: u64 = 0x140206010; const P6: u64 = 0x14307e950; const P7: u64 = 0x141833c0f; const P8: u64 = 0x1414a52a6;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a50e4, funcinfo_rva: 0x36b9514, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rdi = (rdx) as u64; rbp = (rcx) as u64; rcx = (r8) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a511b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; r9 = (rax) as u64; r8 = (rsi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5131); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rcx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); bb = 20; continue; }
            4 => { rax = (0x2e8ba2e8ba2e8bau64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 5; continue; }
            5 => { r12 = ((rbp.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rcx = (0x58u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5174); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rdx = (rsi) as u64; rcx = ((rax.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5188); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 11; continue; }
            10 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a51f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a51ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (r14) as u64; r8 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5213); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8, (rbx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rbx) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = (((rax) & (r14))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(r12)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 18; continue; }
            15 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 19; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 20; continue; }
            20 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a52a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a52a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x141834d64; const P5: u64 = 0xc; const P6: u64 = 0x14266a0e0; const P7: u64 = 0x1414a4d84; const P8: u64 = 0x1414a4e24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a52a8, funcinfo_rva: 0x36b9470, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (r8) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; rcx = (0xd0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a52ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rax = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rax as u32)) as u32); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rcx = ((rbx.wrapping_add(P0)) as u64) as u64; r8 = (rsi) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5305); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            2 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P3), (0x0u64) as u64); rdx = ((rbx.wrapping_add(P0)) as u64) as u64; r8 = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5321); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x141834d64; const P3: u64 = 0x18; const P4: u64 = 0x140203a40; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x38;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a5340, funcinfo_rva: 0x36b933c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64); rcx = (0x58u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a536d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P0), (rax) as u64); rbx = ((rsi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdx = (0x10u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a539b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P0), (rcx) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P1), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P3), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P5), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P6), (rcx) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P7), (0x7u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P7), (rax) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), (0x8u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P8), (rax) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5424(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x140205190; const P3: u64 = 0x20; const P4: u64 = 0x1414a4ed0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a5424, funcinfo_rva: 0x36b93e4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64);
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64 as u64; fl_r = (0x10u64) as u64 as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; }
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a5457); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = ((rbx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64);
    r8 = ((rdi.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0xffu8) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64);
    rcx = (((crate::simd_ops::ru8(r8.wrapping_add(P3))) as i8 as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(1)) as u64;
    rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a5480); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rbx) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5494(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1402050d0; const P2: u64 = 0x1414a5340; const P3: u64 = 0x14148bd18; const P4: u64 = 0x140571d70; const P5: u64 = 0x140204cc0; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1414a5612; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a5494, funcinfo_rva: 0x36b939c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rdi = (r9) as u64; rbx = (r8) as u64; r14 = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rcx) as u64); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a54da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((r14.wrapping_add(0x20u64)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(r15, (xmm0) as u128); crate::simd_ops::lk_and_u64(r15.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(r15.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a54ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((r14.wrapping_add(0x40u64)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5523); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x68u64)) as u64) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5530); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0xfu64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x30u64)))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x10u64)))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_sub(0x48u64))) as u8 as u64)); rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5568); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { rdx = (r14) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r14)) as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5587); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 6; continue; }
            6 => { rdx = (r15) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a55a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 8; continue; }
            8 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a55bb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rbx } else { rcx }) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a55d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x60u64), (rax) as u64); fl_l = (rdi) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdi.wrapping_add(0x1u64)) as u64) as u64; rax = (rbx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a560c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5616); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rax = (r14) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5638(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266a028; const P2: u64 = 0x1414a52a8; const P3: u64 = 0x141833c33; const P4: u64 = 0x14183af0c; const P5: u64 = 0x1412dd770; const P6: u64 = 0x14266a038; const P7: u64 = 0x1411f4730; const P8: u64 = 0x1402034f0; const P9: u64 = 0x14148aef4; const P10: u64 = 0x14266a060; const P11: u64 = 0x1426467f0; const P12: u64 = 0x141834d30; const P13: u64 = 0x1434278a0; const P14: u64 = 0x140200450; const P15: u64 = 0x1412dd710; const P16: u64 = 0x142645ac0; const P17: u64 = 0x140203300; const P18: u64 = 0x1402012a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xb0); rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); r14 = r8; r15 = rdx; rsi = rcx; rax = P1; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x18); rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P6; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x34); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x80); rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x80); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x80u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x1c; r8 = P10; rdx = ((r9.wrapping_sub(0x19u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a577e, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(P13)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x18; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1f8u64))) as u64; rsp = ((rsp).wrapping_add(0x1b0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x18; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0x141834d30; const P6: u64 = 0x10; const P7: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P4)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = r8; bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::lk_and_u64(rdi.wrapping_add(P6), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (0xfu64) as u64); crate::simd_ops::wu8(rdi, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a5908, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a590c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140200450; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x68u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rbp = 0x1000; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x50u64), (0x0u64) as u64); rsi = 0xf; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x40u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 8; continue; }
            8 => { rcx = r8; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = r8; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a5a09, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5a0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14129d960; const P1: u64 = 0x78; const P2: u64 = 0x68; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x1414a5808; const P7: u64 = 0x18; const P8: u64 = 0x1414a590c; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = ((rcx).wrapping_add(0xa0u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = 0xb0; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a61b0; const P1: u64 = 0x90; const P2: u64 = 0x140203300; const P3: u64 = 0x141834d30; const P4: u64 = 0x78; const P5: u64 = 0x10; const P6: u64 = 0x1402012a0; const P7: u64 = 0x8; const P8: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xa0u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x18; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5b24(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266a028; const P1: u64 = 0x8; const P2: u64 = 0x1414a73d0; const P3: u64 = 0x18; const P4: u64 = 0x141833c27; const P5: u64 = 0x10; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P6), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5b88(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266a0e0; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5bb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a5a0c; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc8; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5be8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414a7140; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5c58(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414a5494;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a5c58, funcinfo_rva: 0x36b9368, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = (r9) as u64; rbp = (r8) as u64; r14 = (rdx) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0xb0u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5c82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a5cd0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a5cf4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1426cdcb0; const P2: u64 = 0x14183acda; const P3: u64 = 0x14183af0c; const P4: u64 = 0x1412dd770; const P5: u64 = 0x14266a0a0; const P6: u64 = 0x1411f4730; const P7: u64 = 0x1402034f0; const P8: u64 = 0x14148aef4; const P9: u64 = 0x14266a060; const P10: u64 = 0x1426467f0; const P11: u64 = 0x141834d30; const P12: u64 = 0x1434278a0; const P13: u64 = 0x140200450; const P14: u64 = 0x1412dd710; const P15: u64 = 0x142645ac0; const P16: u64 = 0x141833c2d; const P17: u64 = 0x1414a7390; const P18: u64 = 0x1414afa98; const P19: u64 = 0x1414a6274; const P20: u64 = 0x1414a5c58; const P21: u64 = 0x141834d64; const P22: u64 = 0x1414aa718; const P23: u64 = 0x14266a108; const P24: u64 = 0x1414a705c; const P25: u64 = 0x1414a590c; const P26: u64 = 0x141833c49; const P27: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 95>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1b8); rsp = ((rsp).wrapping_sub(0x2b8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1a0u64), rax); r12 = r9; r15 = r8; r14 = rdx; r13 = rcx; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x220u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = P1; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = 0x1; rcx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P5; rcx = rbp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x40u64), ((rsi as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x40); rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x180); rcx = rbp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x180); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x198u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x180u64)) } else { rax }) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x26; r8 = P9; rdx = ((r9.wrapping_sub(0x25u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x198u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x180u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a5e63, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x190u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x180u64), ((rsi as u8)) as u8); rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x118); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x118); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = P1; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rsi); bb = 12; continue; }
            12 => { rdi = r13.wrapping_add(0x18); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 39; continue; } bb = 13; continue; }
            13 => { rdx = rsp.wrapping_add(0x78); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbp.wrapping_sub(0x70); r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_sub(0x50); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x44u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x44); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rax = rbp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_sub(0x40); r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x20); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0xc8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x68u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rcx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0xcu64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x68); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; bb = 25; continue; }
            21 => { crate::simd_ops::lk_and_u64(r14, (0x0u64) as u64); crate::simd_ops::lk_and_u64(r14.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r14, rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); bb = 38; continue; }
            24 => { r15 = r12; bb = 25; continue; }
            25 => { rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rax = P23; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); bb = 28; continue; }
            27 => { rbx = r12; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rsi = rbx.wrapping_add(0x8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14, r12); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; bb = 32; continue; }
            32 => { crate::simd_ops::wu64(r14, r15); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { r15 = ((((r15 as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0xb0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r14; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x2b8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            39 => { rcx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a6113, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6114(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1414a5a0c; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0xc8; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a614c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1414a5870; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_and_u64(rax, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x40; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    loop { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x40; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rbx; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rdx = 0x40;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a61b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 7; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = r8; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rsi.wrapping_add(P0), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rsi.wrapping_add(P2), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a6273, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6274(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acda; const P4: u64 = 0x38; const P5: u64 = 0x20; const P6: u64 = 0x30; const P7: u64 = 0x58; const P8: u64 = 0x40; const P9: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x10u64) as u64; rax = rdx; rdx = rcx; rbp = r9; rsi = r8; rdi = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_r = (0x10u64) as u64; rdx = ((rdi.wrapping_add(P5)) as u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_r = (0x10u64) as u64; rdx = ((rdi.wrapping_add(P8)) as u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a633c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14266a138; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6354(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183af0c; const P2: u64 = 0x1412dd770; const P3: u64 = 0x14266a160; const P4: u64 = 0x1411f4730; const P5: u64 = 0x1402034f0; const P6: u64 = 0x14148aef4; const P7: u64 = 0x14266a1c0; const P8: u64 = 0x1426467f0; const P9: u64 = 0x141834d30; const P10: u64 = 0x1434278a0; const P11: u64 = 0x140200450; const P12: u64 = 0x1412dd710; const P13: u64 = 0x142645ac0; const P14: u64 = 0x1414a6644; const P15: u64 = 0x14266a200;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 57>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xb0); rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); r15 = rdx; r14 = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P3; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x34); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x80); rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x80); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x80u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x1e; r8 = P7; rdx = ((r9.wrapping_sub(0x1du64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a6484, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P10)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P15; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x34); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x80); rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x80); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x80u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x24; r8 = P7; rdx = ((r9.wrapping_sub(0x23u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a65c2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { xmm0 = (crate::simd_ops::ru128(P10)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x80u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x1b0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6644(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x140200850; const P2: u64 = 0x11; const P3: u64 = 0x1401e3410; const P4: u64 = 0x1401de2f0;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsi = (rsi & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64));
    rcx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64));
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu8(rax, ((rdi as u8)) as u8);
    rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64));
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6698(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a66a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414a6644; const P2: u64 = 0x14183af0c; const P3: u64 = 0x1412dd770; const P4: u64 = 0x14266a240; const P5: u64 = 0x1411f4730; const P6: u64 = 0x1402034f0; const P7: u64 = 0x14148aef4; const P8: u64 = 0x14266a1c0; const P9: u64 = 0x1426467f0; const P10: u64 = 0x141834d30; const P11: u64 = 0x1434278a0; const P12: u64 = 0x140200450; const P13: u64 = 0x1412dd710; const P14: u64 = 0x142645ac0; const P15: u64 = 0x140200850; const P16: u64 = 0x1401e3410; const P17: u64 = 0x1401de2f0; const P18: u64 = 0x14266a280;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0xd8); rsp = ((rsp).wrapping_sub(0x1c0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); rdi = rdx; r14 = rcx; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x90); rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x90); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x2f; r8 = P8; rdx = ((r9.wrapping_sub(0x2eu64)) as u32) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a67d9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = r14.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x11u64), ((r15 as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (rsi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r14.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 10; continue; }
            10 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P18; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x90); rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x90); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x3a; r8 = P8; rdx = ((r9.wrapping_sub(0x39u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a6942, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), ((r15 as u8)) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | ((((r15 as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x1c0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a69ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6a14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401eae50; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414a6ff4; const P3: u64 = 0x1414a7720; const P4: u64 = 0x1414937f0; const P5: u64 = 0x1414a6c74; const P6: u64 = 0x1401e6f50; const P7: u64 = 0x1414a6c7a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a6a14, funcinfo_rva: 0x36b9648, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = (rdx) as u64; r15 = (rcx) as u64; r13 = (rdx) as u64; r13 = ((r13).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; rbx = (0xfffffffffffffffu64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { r9 = ((rax.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r15))) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rbx) as u64); rcx = (0xfffffffffffffff0u64) as u64; bb = 5; continue; }
            3 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; rcx = (r9) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 4; continue; }
            4 => { rbx = (rcx) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rbx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6abc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rax) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 10; continue; }
            6 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6adf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 9; continue; }
            8 => { rsi = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rdi) as u64); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rbx) as u64); bb = 10; continue; }
            10 => { r13 = (((r13) & (0xfffffffffffffff0u64))) as u64; rcx = ((rsi.wrapping_add(r13)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); r12 = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); crate::simd_ops::wu64(rcx, (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = (r14) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rdx, (rdi) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx, (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = ((rcx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; bb = 23; continue; }
            17 => { crate::simd_ops::wu64(rdx, (rdi) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx, (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = ((rcx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (rcx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6bd9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6bd9, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6a14) }; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r15) as u64); fl_l = (r14) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { r14 = ((r14).wrapping_sub(r13)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rsi) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(r12, (rdi) as u64); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(r12).wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(r12, (rax) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(r12).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(r12).wrapping_sub(0x10u64), (rdi) as u64); crate::simd_ops::wu64(r14.wrapping_add(r12).wrapping_sub(0x8u64), (rdi) as u64); r12 = ((r12).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r12) as u64); rax = ((r14.wrapping_sub(0x10u64)) as u64) as u64; rax = ((rax).wrapping_add(r12)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r12) as u64); rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6c41); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6c41, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6a14) }; r9 = (rbx) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (rsi) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6c55); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6c55, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6a14) }; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6c6f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            25 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6c75); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6c7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401eae50; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414a6ff4; const P3: u64 = 0x1414a7720; const P4: u64 = 0x1414937f0; const P5: u64 = 0x1414a6ed4; const P6: u64 = 0x1401e6f50; const P7: u64 = 0x1414a6eda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a6c7c, funcinfo_rva: 0x36b968c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = (rdx) as u64; r15 = (rcx) as u64; r13 = (rdx) as u64; r13 = ((r13).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; rsi = (0xfffffffffffffffu64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 1; continue; }
            1 => { r9 = ((rax.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r15))) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = (rsi) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (rsi) as u64); rcx = (0xfffffffffffffff0u64) as u64; bb = 5; continue; }
            3 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; r12 = (r9) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; r12 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { r12 }) as u64; fl_l = (r12) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 23; continue; } bb = 4; continue; }
            4 => { rcx = (r12) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rsi = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6d27); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 10; continue; }
            6 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6d4a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 9; continue; }
            8 => { rdi = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (rbx) as u64); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); bb = 10; continue; }
            10 => { r13 = (((r13) & (0xfffffffffffffff0u64))) as u64; rcx = ((rdi.wrapping_add(r13)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); r12 = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r8, (rbx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; rdx = (rdi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = (r14) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = ((rcx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); bb = 21; continue; }
            15 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); fl_l = ((rcx).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { fl_l = (rcx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6e39); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6e39, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6c7c) }; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); fl_l = (r14) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { r14 = ((r14).wrapping_sub(r13)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rdi) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r12, (rbx) as u64); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(r12).wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(r12, (rax) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(r12).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(r12).wrapping_sub(0x10u64), (rbx) as u64); crate::simd_ops::wu64(r14.wrapping_add(r12).wrapping_sub(0x8u64), (rbx) as u64); r12 = ((r12).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); rax = ((r14.wrapping_sub(0x10u64)) as u64) as u64; rax = ((rax).wrapping_add(r12)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); bb = 21; continue; }
            21 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6ea1); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6ea1, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6c7c) }; r9 = (rsi) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (rdi) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6eb5); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414a6eb5, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414a6c7c) }; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6ecf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a6ed5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6edc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = ((rax.wrapping_add(P0)) as u64) as u64;
    fl_l = 0; fl_r = rax;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P1), (0x0u64) as u64);
    r9 = (((r9) & (r8))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, r9);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6f14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414a4be4; const P2: u64 = 0x58; const P3: u64 = 0x48; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x140205190; const P7: u64 = 0x78; const P8: u64 = 0x80; const P9: u64 = 0x88; const P10: u64 = 0x90; const P11: u64 = 0x141833c43; const P12: u64 = 0x140203300; const P13: u64 = 0x141834d30; const P14: u64 = 0x1402012a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; r14 = rdx; rdi = rcx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rcx.wrapping_add(P0), rbp); r8 = rsp.wrapping_add(0x20); rdx = rsi; rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rdx = ((rsi.wrapping_add(P3)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(P4), rbp); crate::simd_ops::wu64(rcx.wrapping_add(P5), rbp); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(P7), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P8), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P9), rbp); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu64(rdi.wrapping_add(P10), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (((rax) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), rbp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rdi.wrapping_add(0xbcu64), ((rbp as u16)) as u16); rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x18; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a6ff4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 6; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a705c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140200850; const P1: u64 = 0x1414a6a14; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x1401e3410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a705c, funcinfo_rva: 0x36b95dc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; rbx = ((rcx.wrapping_add(0xbdu64)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7084); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0xa0u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0xb0u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rdx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdx.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); fl_l = (crate::simd_ops::lk_add_u64(rsi.wrapping_add(0xa8u64), (0x10u64) as u64)) as u64; fl_r = fl_l; bb = 5; continue; }
            4 => { r8 = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a70d7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a70e5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a70f2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7117); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a712a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414b20e4; const P2: u64 = 0x14266a390; const P3: u64 = 0x1414a6c7c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; rsi = rcx; rcx = 0x28; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); r8 = rsp.wrapping_add(0x30); rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            4 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = 0x18; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rsi.wrapping_add(0x78); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 8; continue; }
            7 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rbx.wrapping_add(0x8); bb = 11; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = ((((rsi as u32)) | (0xffffffffu32))) as u32 as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rdi); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (0x10u64) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            13 => { r8 = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a72c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbd; const P1: u64 = 0x140200850; const P2: u64 = 0xa0; const P3: u64 = 0xa8; const P4: u64 = 0x8; const P5: u64 = 0x1401e3410; const P6: u64 = 0x1401de2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rdi = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(r14)) as u64; r14 = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; bb = 2; continue; }
            2 => { rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((rdi as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((rdi as u8)) as u8); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a73b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x98; const P2: u64 = 0x90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((rcx.wrapping_add(P0)) as u64) as u64; return rax; }
            1 => { crate::lift_ops::trap(0x80000003, 0x1414a73b5, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, 0x1414a73b6, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, 0x1414a73b7, rcx, 0); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdx, rax); rax = rdx; return rax; }
            5 => { crate::lift_ops::trap(0x80000003, 0x1414a73c6, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, 0x1414a73c7, rcx, 0); return rax; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a73d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x140226ba0; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x1414a6edc; const P4: u64 = 0x1414b2488; const P5: u64 = 0x14183af0c; const P6: u64 = 0x1412dd770; const P7: u64 = 0x14266a2c8; const P8: u64 = 0x1411f4730; const P9: u64 = 0x1402034f0; const P10: u64 = 0x14148aef4; const P11: u64 = 0x14266a308; const P12: u64 = 0x1426467f0; const P13: u64 = 0x14266a348; const P14: u64 = 0x141834d30; const P15: u64 = 0x1434278a0; const P16: u64 = 0x140200450; const P17: u64 = 0x1412dd710; const P18: u64 = 0x142645ac0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 63>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xd0); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rax); r14 = rcx; r13 = 0x1; rsi = (rsi & !0xffu64) | ((((r13 as u8)) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r13 as u32)) as u32); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = r14.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rdi as u32)) as u32); rdi = (tmp_xchg) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { rbx = (0x7fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = rdi; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsi = (rsi & !0xffu64) | ((((r13 as u8)) as u8 as u64)); bb = 11; continue; }
            10 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = r15; rbx = (0x7fffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 12; continue; }
            12 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 15; continue; }
            15 => { rdx = ((r13 as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P7; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x34); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0xa0); rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0xa0); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x5e; r8 = P11; rdx = ((r13 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 18; continue; }
            18 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a75ca, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 21; continue; }
            21 => { rdx = ((r13 as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P13; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x34); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0xa0); rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0xa0); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x63; r8 = P11; rdx = ((r13 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 22; continue; }
            22 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a76b8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { xmm0 = (crate::simd_ops::ru128(P15)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xb0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xa0u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P18), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x1d0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30; const P1: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbp = r9; r14 = r8; r12 = rdx; rsi = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 7; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = r8; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsi, r12); r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r12)) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(r12)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414a7806, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7808(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7824(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffd8; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x1414b1258;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = rsp;
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    r8 = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P3), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xffffffffffffffd8; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x1414ab4cc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = rsp;
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    r8 = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = ((rax.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P3), ((xmm1 as u64)) as u64);
    crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7894(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } else { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a78d4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa858; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1414aa8d4; const P5: u64 = 0x1414a874c; const P6: u64 = 0x80; const P7: u64 = 0x1414b19c8; const P8: u64 = 0x1414ab1c4;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a78d4, funcinfo_rva: 0x36ba3fc, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (rax) as u64); rsi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (rdx) as u64); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7908); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7928); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(P2), (0x98u64) as u64)) as u64; fl_r = fl_l; bb = 3; continue; }
            2 => { r8 = (rdi) as u64; rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7940); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a794d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7955); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7962); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a796a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0xd0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7998(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0x141834d64; const P5: u64 = 0x18; const P6: u64 = 0x1414a8554;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a7998, funcinfo_rva: 0x36b9be8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    rax = (rsp) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = (r8) as u64;
    rbx = (rdx) as u64;
    rsi = (rcx) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rcx) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P3), (rcx) as u64);
    rcx = (0x68u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a79c7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu16(rax.wrapping_add(P5), (0x101u16) as u16);
    crate::simd_ops::wu64(rsi, (rax) as u64);
    rdx = (rbx) as u64;
    rcx = (rsi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a79e6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64);
    crate::simd_ops::wu64(rsi.wrapping_add(P5), (rax) as u64);
    rax = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7a0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x141834d64; const P5: u64 = 0x14183acbc; const P6: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a7a0c, funcinfo_rva: 0x36ba374, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; r14 = (rcx) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P3), (0x0u64) as u64); rcx = (0x68u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7a3d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(P3), (rax) as u64); rdi = ((rax.wrapping_add(P2)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P1), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7a6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x0u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(P6), (0x0u8) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), (rsi) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rsi) as u64); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx).wrapping_add(P1), (0x0u8) as u8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x140205190; const P5: u64 = 0x28; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a7ad0, funcinfo_rva: 0x36ba2d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (r9) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); rcx = (0x40u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7afc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(P0), (rax) as u64); rdi = ((rax.wrapping_add(P2)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P3), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414a7b2d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7b54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414a7b54, funcinfo_rva: 0x36ba2fc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (r9) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (0x38u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a7b7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rdi.wrapping_add(P0), (rax) as u64);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rdx) as u64);
    rbx = ((rax.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64);
    crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (0x68u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414a7ba5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    crate::simd_ops::wu64(rbx, (rax) as u64);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P3), (0x0u64) as u64);
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7c4c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414a8960; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; rdx = rcx; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbx); crate::simd_ops::wu64(rbx, rbx); crate::simd_ops::wu64(rbx.wrapping_add(P2), rbx); fl_l = (crate::simd_ops::lk_and_u64(rdi.wrapping_add(P0), (0x0u64) as u64)) as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsi, rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P0), rcx); bb = 2; continue; }
            2 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141489d2c; const P2: u64 = 0x14266ae20; const P3: u64 = 0x10; const P4: u64 = 0x14149b0f0; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rbx = rcx; rcx = rdx; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = P2; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); bb = 2; continue; }
            2 => { rdx = rbx; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rax; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = rbx; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7d84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a7bd4; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, rdx); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdi, r8); r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), r8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7dfc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14148b9a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7e08(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143426428; const P2: u64 = 0x14148b98c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 3; continue; }
            3 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; bb = 5; continue; }
            5 => { rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; rdx = 0x9e3779b9; rcx = ((rcx).wrapping_shl(6u32)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = rsi; rcx = ((rcx).wrapping_shr(2u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) ^ (rsi))) as u64; crate::simd_ops::wu64(rdi, rcx); rsi = rcx; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7e94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x140205190; const P2: u64 = 0x140204cc0; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = (((rax) ^ (rsp))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rbx = (crate::simd_ops::ru64(rdx)) as u64; r14 = rcx; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); xmm0 = 0u128; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rdx = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = rsp.wrapping_add(0x20); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbp; fl_r = (0x10u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdi } else { rcx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = 0x9e3779b9; rcx = ((rcx).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::lk_xor_u64(r14, (rcx) as u64); fl_l = rbp; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = rbp.wrapping_add(0x1); rax = rdi; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { fl_l = ((rbx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 1; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414a7f79, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a7f9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414b1ba8; const P1: u64 = 0x141091370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rax = rdx; rsi = rcx; rdx = rsp.wrapping_add(0x20); rcx = rax; rbx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = 0x1; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rcx = rsp.wrapping_add(0x48); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x9e3779b9; rcx = ((rcx).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::lk_xor_u64(rsi, (rcx) as u64); fl_l = rbx; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 1; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 1; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a8030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140200850; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aca18; const P3: u64 = 0x141834d64; const P4: u64 = 0x1414a8960; const P5: u64 = 0x141834d30; const P6: u64 = 0x1414af9bc; const P7: u64 = 0x14149c840; const P8: u64 = 0x1414b10fc; const P9: u64 = 0x1414abfc8; const P10: u64 = 0x1401e3410; const P11: u64 = 0x1401de2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x40); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rdx; r15 = rcx; rsi = rcx.wrapping_add(0x90); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 7; continue; }
            1 => { r12 = rbx.wrapping_add(0x10); rdx = r15.wrapping_add(0x10); rcx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x78u64))) as u32 as u64; rcx = rbp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rax = (crate::simd_ops::ru64(r13)) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rcx = 0x68; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rsp.wrapping_add(0x20); xmm1 = ((crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64) as u128; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x68; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), r14); r8 = rbp.wrapping_add(0x98); rdx = rsp.wrapping_add(0x38); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r13; rdx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x88u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r9 = r14; r8 = rsp.wrapping_add(0x40); rdx = r12; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x80u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); r9 = r14; r8 = rsp.wrapping_add(0x48); rdx = r12; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            5 => { rdx = r15.wrapping_add(0x10); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r13; r8 = rax; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x78u64))) as u32 as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r14; r8 = rax; rdx = r12; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x88u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r9 = r14; r8 = rsp.wrapping_add(0x50); rdx = r12; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x0u8) as u8); rcx = rbp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; rsp = ((rsp).wrapping_add(0x140u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414a8270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140200850; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aca18; const P3: u64 = 0x141834d64; const P4: u64 = 0x1414a8960; const P5: u64 = 0x141834d30; const P6: u64 = 0x1414af9bc; const P7: u64 = 0x14149c840; const P8: u64 = 0x1414b10fc; const P9: u64 = 0x1414abfc8; const P10: u64 = 0x1401e3410; const P11: u64 = 0x1401de2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x40); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = rdx; r15 = rcx; rsi = rcx.wrapping_add(0x90); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 7; continue; }
            1 => { r13 = rbx.wrapping_add(0x10); rdx = r15.wrapping_add(0x10); rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x78u64))) as u32 as u64; rcx = rbp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rcx = 0x68; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = r12; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x68; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = r13; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r14 = rax; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), r14); r8 = rbp.wrapping_add(0x98); rdx = rsp.wrapping_add(0x38); rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r12; rdx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x88u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r9 = r14; r8 = rsp.wrapping_add(0x40); rdx = r13; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x80u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); r9 = r14; r8 = rsp.wrapping_add(0x48); rdx = r13; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rdx = r15.wrapping_add(0x10); rcx = rbp.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = r12; r8 = rax; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x78u64))) as u32 as u64; rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = r14; r8 = rax; rdx = r13; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x88u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r9 = r14; r8 = rsp.wrapping_add(0x50); rdx = r13; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), (0x0u8) as u8); rcx = rbp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; rsp = ((rsp).wrapping_add(0x140u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
