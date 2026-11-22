#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fed00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x3; const P3: u64 = 0x1426e7d15; const P4: u64 = 0x143407638; const P5: u64 = 0x1405fc3c0;
    let (mut rax, mut r10, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    r10 = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32);
    r8 = P4;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r10 as u32)) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423fef10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe8f0; const P1: u64 = 0x1434075d8; const P2: u64 = 0x1426e7d15; const P3: u64 = 0x1405fc3c0; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 309>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x848); rsp = ((rsp).wrapping_sub(0x968u64)) as u64; rbx = rdx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32) as u128; rcx = rbp.wrapping_add(0x7b0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x7b3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7b0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x7b8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x7b8u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x898u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32) as u128; rcx = rbp.wrapping_add(0x728); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x72bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x728u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x730); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x730u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8a0u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32) as u128; rcx = rbp.wrapping_add(0x6a0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6a3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6a0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x6a8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x6a8u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8a8u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32) as u128; rcx = rbp.wrapping_add(0x618); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x61bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x618u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x620); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x620u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32) as u128; rcx = rbp.wrapping_add(0x590); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x593u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x590u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x598); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x598u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32) as u128; rcx = rbp.wrapping_add(0x508); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x50bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x508u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x510); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x510u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32) as u128; rcx = rbp.wrapping_add(0x480); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x483u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 21; continue; }
            20 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x480u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x488); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x488u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; rcx = rbp.wrapping_add(0x3f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x3fbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3f8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x400); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x400u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; rcx = rbp.wrapping_add(0x370); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x373u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 27; continue; }
            26 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x370u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x378); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x378u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; rcx = rbp.wrapping_add(0x2e8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2ebu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 30; continue; }
            29 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r13 = rbp.wrapping_add(0x2f0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r13 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x2f0u64)) } else { r13 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; rcx = rbp.wrapping_add(0x260); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x263u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 33; continue; }
            32 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x260u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r12 = rbp.wrapping_add(0x268); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r12 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x268u64)) } else { r12 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; rcx = rbp.wrapping_add(0x1d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x1dbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 36; continue; }
            35 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1d8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rbp.wrapping_add(0x1e0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1e0u64)) } else { r15 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; rcx = rbp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x153u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 39; continue; }
            38 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x150u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r14 = rbp.wrapping_add(0x158); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r14 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x158u64)) } else { r14 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rcx = rbp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xcbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 42; continue; }
            41 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rbp.wrapping_add(0xd0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64)) } else { rsi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x43u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 45; continue; }
            44 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = rbp.wrapping_add(0x48); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x48u64)) } else { rdi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x45u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 48; continue; }
            47 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = rbp.wrapping_sub(0x40); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64)) } else { r9 }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x898u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8a0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8a8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x890u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x1d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x260); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x2e8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x370); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x3f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x480); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x508); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x590); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x618); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x6a0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x728); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x7b0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x890u64))) as u64; rsp = ((rsp).wrapping_add(0x968u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ff5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe8f0; const P1: u64 = 0x143407610; const P2: u64 = 0x1426e7d15; const P3: u64 = 0x1405fc3c0; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 233>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x608); rsp = ((rsp).wrapping_sub(0x708u64)) as u64; rbx = rdx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32) as u128; rcx = rbp.wrapping_add(0x570); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x573u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x570u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x578); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x578u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x658u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32) as u128; rcx = rbp.wrapping_add(0x4e8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4ebu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4e8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x4f0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x4f0u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x660u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32) as u128; rcx = rbp.wrapping_add(0x460); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x463u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x460u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x468); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x468u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x668u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; rcx = rbp.wrapping_add(0x3d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x3dbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3d8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x3e0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x3e0u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32) as u128; rcx = rbp.wrapping_add(0x350); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x353u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x350u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbp.wrapping_add(0x358); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x358u64)) } else { rdx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rdx); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; rcx = rbp.wrapping_add(0x2c8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2cbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2c8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r13 = rbp.wrapping_add(0x2d0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r13 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x2d0u64)) } else { r13 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; rcx = rbp.wrapping_add(0x240); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x243u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 21; continue; }
            20 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x240u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r12 = rbp.wrapping_add(0x248); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r12 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x248u64)) } else { r12 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; rcx = rbp.wrapping_add(0x1b8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x1bbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1b8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rbp.wrapping_add(0x1c0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x1c0u64)) } else { r15 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; rcx = rbp.wrapping_add(0x130); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 27; continue; }
            26 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r14 = rbp.wrapping_add(0x138); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r14 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x138u64)) } else { r14 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rcx = rbp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xabu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 30; continue; }
            29 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xa8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rbp.wrapping_add(0xb0); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64)) } else { rsi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x23u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 33; continue; }
            32 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = rbp.wrapping_add(0x28); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x28u64)) } else { rdi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x65u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 36; continue; }
            35 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x68u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = rbp.wrapping_sub(0x60); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64)) } else { r9 }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x658u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x660u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x668u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x650u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x130); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x1b8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x240); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x2c8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x350); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x3d8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x460); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x4e8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x570); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x650u64))) as u64; rsp = ((rsp).wrapping_add(0x708u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ffb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x143407658; const P2: u64 = 0x8; const P3: u64 = 0x4; const P4: u64 = 0x1426e7d15; const P5: u64 = 0x1405fc3c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r8 = P1;
    r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    rdx = ((crate::simd_ops::ru8(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ffb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143407668; const P2: u64 = 0x4; const P3: u64 = 0x1426e7d15; const P4: u64 = 0x1405fc3c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r8 = P1;
    r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ffba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1433feadc; const P2: u64 = 0x1426e7d15; const P3: u64 = 0x1405fc3c0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    r8 = P1;
    r9 = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rbx = rcx;
    rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1423ffd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); r14 = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm9) as u128); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rbp.wrapping_add(0x67); r8 = rsi; rdx = rbx; rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; fl_r = 0x0; xmm7 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm6 = 0u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 12; continue; }
            12 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { r9 = rbp.wrapping_add(0x67); r8 = rsi; rdx = rbx; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm9 = (xmm7) as u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 17; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; xmm9 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 17; continue; }
            17 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r9 = rbp.wrapping_add(0x67); r8 = rsi; rdx = rbx; rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm8 = (xmm7) as u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; xmm8 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 23; continue; }
            23 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 26; continue; }
            26 => { r9 = rbp.wrapping_add(0x67); r8 = rsi; rdx = rbx; rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 29; continue; }
            28 => { xmm7 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 29; continue; }
            29 => { r11 = rsp.wrapping_add(0xb0); rax = (((rdi as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; xmm6 = crate::simd_ops::vinsertps(xmm6, xmm9, 0x10u8); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm6 = crate::simd_ops::vinsertps(xmm6, xmm8, 0x20u8); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm6 = crate::simd_ops::vinsertps(xmm6, xmm7, 0x30u8); xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(r14, (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142400190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0x58); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; xmm6 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = 0u128; bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 18; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 23; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 25; continue; }
            25 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 28; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 30; continue; }
            30 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 33; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 33; continue; }
            33 => { crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { bb = 35; continue; }
            35 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 36; continue; }
            36 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x78u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 39; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 39; continue; }
            39 => { crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 41; continue; } bb = 42; continue; }
            42 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 45; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { bb = 47; continue; }
            47 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 47; continue; } bb = 48; continue; }
            48 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 51; continue; }
            50 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 54; continue; } bb = 52; continue; }
            52 => { bb = 53; continue; }
            53 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 53; continue; } bb = 54; continue; }
            54 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 55; continue; }
            55 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 57; continue; }
            56 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 57; continue; }
            57 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 60; continue; } bb = 58; continue; }
            58 => { bb = 59; continue; }
            59 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 59; continue; } bb = 60; continue; }
            60 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 63; continue; }
            62 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 63; continue; }
            63 => { crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 66; continue; } bb = 64; continue; }
            64 => { bb = 65; continue; }
            65 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 65; continue; } bb = 66; continue; }
            66 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 68; continue; } bb = 67; continue; }
            67 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 69; continue; }
            68 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 69; continue; }
            69 => { crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 72; continue; } bb = 70; continue; }
            70 => { bb = 71; continue; }
            71 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 71; continue; } bb = 72; continue; }
            72 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 75; continue; }
            74 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 75; continue; }
            75 => { crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 78; continue; } bb = 76; continue; }
            76 => { bb = 77; continue; }
            77 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 77; continue; } bb = 78; continue; }
            78 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 80; continue; } bb = 79; continue; }
            79 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 81; continue; }
            80 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 81; continue; }
            81 => { crate::simd_ops::wu32(rsi.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 84; continue; } bb = 82; continue; }
            82 => { bb = 83; continue; }
            83 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 83; continue; } bb = 84; continue; }
            84 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 86; continue; } bb = 85; continue; }
            85 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 87; continue; }
            86 => { rbx = (crate::simd_ops::ru64(rbp)) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 87; continue; }
            87 => { crate::simd_ops::wu32(rsi.wrapping_add(0x34u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 90; continue; } bb = 88; continue; }
            88 => { bb = 89; continue; }
            89 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 89; continue; } bb = 90; continue; }
            90 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 92; continue; } bb = 91; continue; }
            91 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 93; continue; }
            92 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 93; continue; }
            93 => { crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 96; continue; } bb = 94; continue; }
            94 => { bb = 95; continue; }
            95 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 95; continue; } bb = 96; continue; }
            96 => { r9 = rbp.wrapping_add(0x60); r8 = r14; rdx = rbx; rcx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 98; continue; } bb = 97; continue; }
            97 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 99; continue; }
            98 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; bb = 99; continue; }
            99 => { r11 = rsp.wrapping_add(0x140); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x3cu64), ((xmm6 as u32)) as u32); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142400760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; xmm6 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = 0u128; bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x79); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x71u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 18; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 21; continue; }
            21 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x61u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 24; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x51u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 30; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 30; continue; }
            30 => { crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 33; continue; }
            33 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x41u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 36; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 39; continue; }
            39 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 42; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 42; continue; }
            42 => { crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 44; continue; } bb = 45; continue; }
            45 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 48; continue; }
            47 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 48; continue; }
            48 => { crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { bb = 50; continue; }
            50 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 50; continue; } bb = 51; continue; }
            51 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x11u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 54; continue; }
            53 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 54; continue; }
            54 => { crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { bb = 56; continue; }
            56 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 56; continue; } bb = 57; continue; }
            57 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 60; continue; }
            59 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 60; continue; }
            60 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { bb = 62; continue; }
            62 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 62; continue; } bb = 63; continue; }
            63 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_add(0x7); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0xfu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 65; continue; } bb = 64; continue; }
            64 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 66; continue; }
            65 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 66; continue; }
            66 => { crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 69; continue; } bb = 67; continue; }
            67 => { bb = 68; continue; }
            68 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 68; continue; } bb = 69; continue; }
            69 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_add(0x17); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x1fu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 71; continue; } bb = 70; continue; }
            70 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm6) as u128; bb = 72; continue; }
            71 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 72; continue; }
            72 => { crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 75; continue; } bb = 73; continue; }
            73 => { bb = 74; continue; }
            74 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 74; continue; } bb = 75; continue; }
            75 => { r9 = rbp.wrapping_add(0x67); r8 = r14; rdx = rbx; rcx = rbp.wrapping_add(0x27); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x2fu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 77; continue; } bb = 76; continue; }
            76 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 78; continue; }
            77 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32) as u128; bb = 78; continue; }
            78 => { r11 = rsp.wrapping_add(0x100); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((xmm6 as u32)) as u32); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142400be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141121ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rsi = rdx; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); r8 = rbp; rcx = rsp.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = ((xmm0 as u64)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { r9 = rsp.wrapping_add(0x80); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rbx = ((xmm0 as u64)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 21; continue; }
            21 => { r9 = rsp.wrapping_add(0x80); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = 0; fl_l = 0; fl_r = 0; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 24; continue; }
            23 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rbx = ((xmm0 as u64)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { r9 = rsp.wrapping_add(0x80); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), (0x0u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142400e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141121ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { r9 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); r8 = rbp; rcx = rsp.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = ((xmm0 as u64)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x70); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 17; continue; }
            16 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rbx = ((xmm0 as u64)) as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r9 = rsp.wrapping_add(0x70); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x0u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = (((rsi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142400fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141121ad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); r8 = rbp; rcx = rsp.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = ((xmm0 as u64)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { r9 = rsp.wrapping_add(0x60); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = (((rsi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142401120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434075d0; const P1: u64 = 0x140000000;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r13 = P0; rbp = 0; fl_l = 0; fl_r = 0; r12 = P1; r15 = rdx; r14 = rcx; crate::simd_ops::wu128(rsp, (xmm0) as u128); r11 = ((xmm0 as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r10 = ((r11.wrapping_add(rbx)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; rdx = r13; r8 = 0x2; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp, r11); fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            8 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 3; continue; }
            10 => { rbx = rbp; bb = 11; continue; }
            11 => { rdx = ((r11.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = rdx; rax = rdx; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_sub(0x1u64))) as u32) as u32 as u64; rdi = rdx.wrapping_sub(0x1); bb = 13; continue; }
            13 => { rsi = ((crate::simd_ops::ru8(r8.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; r9 = r13; r10 = 0x2; bb = 14; continue; }
            14 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 15; continue; }
            15 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rcx = rax; bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r14, (xmm0) as u128); r9 = (crate::simd_ops::ru64(r14)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = r9; r10 = ((r10).wrapping_add(r9)) as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0x2du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = r9.wrapping_add(0x1); bb = 22; continue; }
            22 => { rsi = (((rdi as u8)) as u32) as u32 as u64; r11 = (r11 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = rbp; rbx = rsi.wrapping_add(0x7); fl_l = rdx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 23; continue; }
            23 => { r14 = (0xcccccccccccccccu64) as u64; bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x33a6890u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 34; continue; } bb = 25; continue; }
            25 => { fl_l = rcx; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = r8; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 28; continue; }
            28 => { r11 = (r11 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 33; continue; }
            29 => { r9 = ((r9).wrapping_sub(r13)) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_sub(1)) as u64; rax = rdi; r8 = rdi; fl_l = rdi; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 31; continue; }
            31 => { r8 = ((crate::simd_ops::ru8(r8.wrapping_sub(0x1u64))) as u32) as u32 as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 13; continue; }
            32 => { rax = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; bb = 33; continue; }
            33 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 34; continue; }
            34 => { rdx = ((rdx).wrapping_sub(r9)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rbp = 0x16; bb = 41; continue; }
            36 => { fl_l = ((r11 as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rbp = 0x22; bb = 41; continue; }
            38 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { fl_l = 0; fl_r = rcx; rcx = (0u64.wrapping_sub(fl_r as u64)) as u64; bb = 40; continue; }
            40 => { crate::simd_ops::wu64(r15, rcx); bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x10u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142401380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); r8 = rbp; rcx = rsp.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; xmm6 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = 0u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x70); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm6 as u32)) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424014f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rsi = rdx; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { r9 = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); r8 = rbp; rcx = rsp.wrapping_add(0x30); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; xmm6 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = 0u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x80); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (xmm6) as u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 17; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { r9 = rsp.wrapping_add(0x80); r8 = rbp; rdx = rbx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((xmm6 as u32)) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rax = (((rdi as u8)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142401a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe8f0; const P1: u64 = 0x143407648; const P2: u64 = 0x1426e7d15; const P3: u64 = 0x1405fc3c0; const P4: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 81>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x170); rsp = ((rsp).wrapping_sub(0x270u64)) as u64; rbx = rdx; rsi = rcx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rcx = rbp.wrapping_add(0xdc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xdfu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xdcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rbp.wrapping_add(0xe4); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0xe4u64)) } else { r15 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; rcx = rbp.wrapping_add(0x54); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x57u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x54u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r14 = rbp.wrapping_add(0x5c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r14 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x5cu64)) } else { r14 }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; rcx = rbp.wrapping_sub(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x31u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x34u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = rbp.wrapping_sub(0x2c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x2cu64)) } else { rdi }) as u64; xmm1 = (crate::simd_ops::ru128(rbx)) as u128; rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x47u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = rsp.wrapping_add(0x4c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x4cu64)) } else { r9 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x44); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x54); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0xdc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; r11 = rsp.wrapping_add(0x270); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142401dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646980; const P1: u64 = 0x141376a70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbp = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = ((xmm0 as u64)) as u64; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 6; continue; }
            6 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r9 = rsp.wrapping_add(0xc0); r8 = rsi; rdx = rbx; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; xmm6 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm9 = 0u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); xmm9 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32) as u128; bb = 12; continue; }
            12 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { r9 = rsp.wrapping_add(0xc0); r8 = rsi; rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm8 = (xmm6) as u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32) as u128; bb = 18; continue; }
            18 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 21; continue; }
            21 => { r9 = rsp.wrapping_add(0xc0); r8 = rsi; rdx = rbx; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm7 = (xmm6) as u128; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 24; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32) as u128; bb = 24; continue; }
            24 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rcx = (((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { r9 = rsp.wrapping_add(0xc0); r8 = rsi; rdx = rbx; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            29 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32) as u128; bb = 30; continue; }
            30 => { r11 = rsp.wrapping_add(0xb0); rax = (((rdi as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm0 = (xmm9) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm8, 0x10u8); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm7, 0x20u8); xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, xmm6, 0x30u8); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430782bc; const P1: u64 = 0x1430782b4; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x8; const P5: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rcx;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P0;
    rax = P1;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { rax }) as u64;
    crate::simd_ops::lk_and_u32(rbx, (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rbx, (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x10u8) as u8);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((r8 as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P4), ((r8 as u8)) as u8);
    r9 = (0xffffffffffffffffu64) as u64;
    loop { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x30);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424020c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe8f0; const P1: u64 = 0x143407680; const P2: u64 = 0x1426e7d15; const P3: u64 = 0x1405fc3c0; const P4: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; rbx = rdx; rdi = rcx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; rcx = rsp.wrapping_add(0x144); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x147u64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x144u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rbp = rsp.wrapping_add(0x14c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rbp = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x14cu64)) } else { rbp }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; rcx = rsp.wrapping_add(0xbc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbfu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xbcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rsp.wrapping_add(0xc4); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0xc4u64)) } else { rsi }) as u64; xmm1 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; rcx = rsp.wrapping_add(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x37u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = rsp.wrapping_add(0x3c); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64)) } else { r9 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x34); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xbc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x144); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; r11 = rsp.wrapping_add(0x1d0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424026e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434075d0; const P1: u64 = 0x140000000; const P2: u64 = 0x1430782bc; const P3: u64 = 0x1430782b4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = P0; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r13 = P1; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r12 = rdx; r14 = rcx; crate::simd_ops::wu128(rsp, (xmm0) as u128); r11 = ((xmm0 as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r10 = ((rbx.wrapping_add(r11)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; rdx = rsi; r8 = 0x2; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp, r11); fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            8 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 3; continue; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = ((rbx.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = rdx; rax = rdx; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_sub(0x1u64))) as u32) as u32 as u64; rdi = rdx.wrapping_sub(0x1); bb = 13; continue; }
            13 => { r9 = rsi; r10 = 0x2; rsi = ((crate::simd_ops::ru8(r8.wrapping_add(r13).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; bb = 14; continue; }
            14 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r13).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 15; continue; }
            15 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rcx = rax; bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r14, (xmm0) as u128); r11 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r14)) as u64; rdi = ((r11.wrapping_add(r8)) as u64) as u64; fl_l = r11; fl_r = (0x4u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 29; continue; } bb = 20; continue; }
            20 => { r9 = P2; r10 = r8; fl_l = r10; fl_r = r9; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rbx = 0x4; bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(r9))) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x27e54e0u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x27e54e0u64))) as u32) as u32 as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 25; continue; }
            25 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((r8).wrapping_add(0x4u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; r15 = (((rbp as u8)) as u32) as u32 as u64; bb = 36; continue; }
            26 => { rsi = P0; r9 = ((r9).wrapping_sub(rsi)) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 27; continue; }
            27 => { rcx = ((rcx).wrapping_sub(1)) as u64; rax = rdi; r8 = rdi; fl_l = rdi; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 28; continue; }
            28 => { r8 = ((crate::simd_ops::ru8(r8.wrapping_sub(0x1u64))) as u32) as u32 as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 13; continue; }
            29 => { fl_l = r11; fl_r = (0x5u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 36; continue; } bb = 30; continue; }
            30 => { r9 = P3; r10 = r8; fl_l = r10; fl_r = r9; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = 0x5; bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(r9))) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x27e54e0u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r13).wrapping_add(0x27e54e0u64))) as u32) as u32 as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 34; continue; }
            34 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { fl_l = ((r8).wrapping_add(0x5u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 36; continue; }
            36 => { fl_l = r8; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu8(r12, ((rbp as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 40; continue; }
            39 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1434075d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbp = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r12 = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r13); r13 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r14); r14 = rcx; crate::simd_ops::wu128(rsp, (xmm0) as u128); r11 = ((xmm0 as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { r10 = ((r11.wrapping_add(rbx)) as u64) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = r11; bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; rdx = r13; r8 = 0x2; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_sub(r11)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp, r11); fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            8 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 3; continue; }
            10 => { rbx = rbp; bb = 11; continue; }
            11 => { rdx = ((r11.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = rdx; rax = rdx; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_sub(0x1u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = rdx.wrapping_sub(0x1); bb = 13; continue; }
            13 => { rsi = ((crate::simd_ops::ru8(r8.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u32) as u32 as u64; r9 = r13; r10 = 0x2; bb = 14; continue; }
            14 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r12).wrapping_add(0x26d01e0u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 15; continue; }
            15 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rcx = rax; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rsp)) as u128; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu128(r14, (xmm0) as u128); r11 = (crate::simd_ops::ru64(r14)) as u64; r10 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = r11; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r10 = ((r10).wrapping_add(r11)) as u64; fl_l = r11; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r12).wrapping_add(0x33a6890u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 31; continue; } bb = 22; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x19999999; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 29; continue; } bb = 23; continue; }
            23 => { if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 25; continue; }
            25 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            26 => { r9 = ((r9).wrapping_sub(r13)) as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 27; continue; }
            27 => { rcx = ((rcx).wrapping_sub(1)) as u64; rax = rdi; r8 = rdi; fl_l = rdi; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 28; continue; }
            28 => { r8 = ((crate::simd_ops::ru8(r8.wrapping_sub(0x1u64))) as u32) as u32 as u64; fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 13; continue; }
            29 => { rax = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; bb = 30; continue; }
            30 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 31; continue; }
            31 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rbp = 0x16; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            33 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rbp = 0x22; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(r15, ((rcx as u32)) as u32); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    r9 = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0xcbf29ce484222325u64) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::wu64(r9, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    r10 = (0x100000001b3u64) as u64;
    loop { rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) | (0x20u64))) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::wu64(r9, rax);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142402ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x1426cdcb0; const P4: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rcx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u32(rbx, (0xff000010u32) as u32);
    crate::simd_ops::lk_or_u32(rbx, (0x10u32) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x10u8) as u8);
    crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rcx as u8)) as u8);
    rax = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x30);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142403070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b16a0; const P1: u64 = 0x141377100; const P2: u64 = 0x1426d01e0; const P3: u64 = 0x1402250d0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = rdx; r13 = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = r15.wrapping_add(0x20); rdx = rsp.wrapping_add(0x20); rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x8u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_sub(0xffffffffffffff80u64)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r12 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; rbp = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((r12.wrapping_add(rbp)) as u64) as u64; rax = rbp; fl_l = rbp; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(r15)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r10 = P2; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { r11 = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = r14; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = r11; fl_r = rcx; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_sub(1)) as u64; rdx = ((crate::simd_ops::ru8(r11.wrapping_add(rcx))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 8; continue; }
            8 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r10 = P2; bb = 7; continue; }
            11 => { r10 = P2; bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(0x50u64)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = ((r8.wrapping_add(rbp)) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = (0x6666666666666667u64) as u64; rax = r9; let prod = (rax as i64 as i128).wrapping_mul((r12) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(5u32) as u64)) as u64; rcx = rdi; rcx = ((rcx).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = rdi.wrapping_add(0x1); rax = r9; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rbx = rdx; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rbx }) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rbx as u32)) as u32); rcx = ((rdi as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; let y_addr: u64 = r15; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r15.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rdx); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = r15.wrapping_add(0x10); rdx = rsp.wrapping_add(0x30); rcx = r13.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x8u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rcx); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424033b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426d01e0; const P1: u64 = 0x1424b16a0; const P2: u64 = 0x141377100; const P3: u64 = 0x140698600;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = r8; rsi = rdx; r9 = (crate::simd_ops::ru64(r8)) as u64; rbx = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; fl_l = r10; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { r8 = ((xmm0 as u64)) as u64; fl_l = r8; fl_r = r9; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = P0; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r9))) as u32) as u32 as u64; r10 = ((r10).wrapping_sub(1)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 7; continue; } bb = 4; continue; }
            4 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 7; continue; } bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rbp = (0xde5fb9d2630458e9u64) as u64; rax = rbp; r8 = rsi; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = rbx.wrapping_add(0x8); r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r8 = rdi; rax = rbp; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = rbx.wrapping_add(0x8); r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x60u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rdx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x50u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { r8 = ((r8).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { rdx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x140f366d0; const P2: u64 = 0x142405dd0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rcx = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x142405dd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x140f366d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rsi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x48; const P2: u64 = 0x140f366d0; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x100; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x18; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { r10 = ((r9.wrapping_add(P3)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(P5)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P9), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424059f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140f366d0; const P1: u64 = 0x142405dd0; const P2: u64 = 0x142405060; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x19u64), (0x1u8) as u8); rsi = rcx.wrapping_add(0x50); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142405f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143426650; const P2: u64 = 0x143427930; const P3: u64 = 0x143426fc8; const P4: u64 = 0x1426cdcb0; const P5: u64 = 0x1424059f0; const P6: u64 = 0x143a00288; const P7: u64 = 0x141377260; const P8: u64 = 0x1424037c0; const P9: u64 = 0x14183acc8; const P10: u64 = 0x140f366d0; const P11: u64 = 0x142405640; const P12: u64 = 0x1426d01e0; const P13: u64 = 0x141377460; const P14: u64 = 0x1425371a0; const P15: u64 = 0x142533200; const P16: u64 = 0x142405730;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142405f70, funcinfo_rva: 0x37df2f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x138, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x27u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x100u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xf0u64), (xmm6) as u128); crate::simd_ops::wu16(rbp.wrapping_sub(0x50u64), (0x0u16) as u16); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x4du64), ((rdi as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((xmm3 as u32)) as u32); xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rdi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm2 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (rdi) as u64); rcx = (((xmm2 >> 64) as u64)) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 3; continue; }
            2 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x8000000000000000u64) as u64; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rdx) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (rax) as u64); xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rdi as u8)) as u8); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406093); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(P6)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rdx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424060b8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; xmm6 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); r12 = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; r14 = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(3u32)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((r12.wrapping_add(0x1u64)) as u64) as u64; rbx = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406105); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240611f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406128); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu128(r15.wrapping_add(r14), (xmm6) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(r14).wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 14; continue; }
            14 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; r9 = ((r9).wrapping_add(rcx)) as u64; rax = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r9) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406183); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r9) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424061a0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 16; continue; }
            16 => { r8 = (0x2aaaaaaaaaaaaaabu64) as u64; r13 = ((P12) as u64) as u64; bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm0) as u128); rbx = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx.wrapping_mul(8)) as u64) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rsi = ((rdi.wrapping_add(rcx)) as u64) as u64; rax = (r8) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = (rdx) as u64; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; rax = (r14) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406227); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((r14 as u32)) as u32); r8 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 19; continue; }
            19 => { fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = ((rax.wrapping_add(0x18u64)) as u64) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rax as u32)) as u32); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 23; continue; }
            23 => { rcx = (rdi) as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = (r8) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rbx) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424062b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 24; continue; }
            24 => { xmm1 = (crate::simd_ops::ru128(rbx)) as u128; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 25; continue; }
            25 => { rcx = (rsi) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { r10 = ((xmm1 as u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rcx) as u64 as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 27; continue; }
            27 => { rdx = ((rdx).wrapping_sub(1)) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r13))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r13))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 31; continue; } bb = 28; continue; }
            28 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 30; continue; }
            30 => { bb = 32; continue; }
            31 => { fl_l = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r11 = ((fl_l as u32)) as u32 as u64; r14 = (((xmm1 >> 64) as u64)) as u64; rsi = ((xmm1 as u64)) as u64; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax, ((r11 as u32)) as u32); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 33; continue; }
            33 => { rcx = ((rbp.wrapping_sub(0x79u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406345); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((r15).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r15) as u64); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406359); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (r12) as u64 as u64; rdi = (0x0u32) as u32 as u64; r8 = (0x2aaaaaaaaaaaaaabu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 34; continue; }
            34 => { bb = 43; continue; }
            35 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 36; continue; }
            36 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x79u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 43; continue; } bb = 39; continue; }
            39 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424063e3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 43; continue; }
            40 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 42; continue; }
            41 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424063f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 43; continue; }
            43 => { rcx = ((rbp.wrapping_sub(0x69u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424063ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; rsp = ((rsp).wrapping_add(0x100u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a001ac; const P1: u64 = 0x1434077a8; const P2: u64 = 0x143a00290;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsi = P2; rdx = rsi; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp, rdi); rdi = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rsi = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = ((r8 as u32)) as u32 as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; while rcx != 0 { crate::simd_ops::wu64(rdi, crate::simd_ops::ru64(rsi)); rdi = rdi.wrapping_add(8); rsi = rsi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } rdi = (crate::simd_ops::ru64(rsp)) as u64; bb = 8; continue; }
            8 => { fl_l = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P0, ((r8 as u32)) as u32); bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e42d0; const P1: u64 = 0x143426d28; const P2: u64 = 0xb0; const P3: u64 = 0x8; const P4: u64 = 0x3; const P5: u64 = 0x1434077c0; const P6: u64 = 0x1424c0160; const P7: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x88u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; } else { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx)) as u64; }
    xmm2 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128);
    r8 = ((xmm2 as u64)) as u64;
    rdx = rcx;
    rcx = P5;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1423f08f0; const P2: u64 = 0x142407c80; const P3: u64 = 0x143c1bfb0; const P4: u64 = 0x143c1bfa8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0xffffffff; rcx = ((rax as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(rbx)) as u64; crate::simd_ops::wu32(r9, ((rsi as u32)) as u32); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = (((rsi as u32)).wrapping_shr(1u32)) as u32 as u64; rsi = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0x1; bb = 6; continue; }
            4 => { rsi = (((rsi as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rsi = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rdi as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { r10 = (crate::simd_ops::ru64(P4)) as u64; bb = 9; continue; }
            9 => { rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(r9.wrapping_add(0xcu64), (0x0u8) as u8); bb = 18; continue; }
            15 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(r9.wrapping_add(0xcu64), (0x1u8) as u8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((rdi as u32)) as u32); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645580; const P1: u64 = 0x142645868; const P2: u64 = 0x142645748; const P3: u64 = 0x1423f0c20; const P4: u64 = 0x142442de0; const P5: u64 = 0x143406c90; const P6: u64 = 0x1423f0780; const P7: u64 = 0x14136eec0; const P8: u64 = 0x142406e58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142406d60, funcinfo_rva: 0x37df4fc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), (rbp) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((rbp as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(0x130u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406dd1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406dd7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406dfa); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406e0c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406e14); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rdx = ((P5) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406e4a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406e53); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P8, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434077d8; const P1: u64 = 0x1423fe030; const P2: u64 = 0x141378580; const P3: u64 = 0x1423f0c20; const P4: u64 = 0x142442de0; const P5: u64 = 0x1423f0b30; const P6: u64 = 0x142645868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142406e60, funcinfo_rva: 0x37df53c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rbx as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406e8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); rax = ((rsp.wrapping_add(0x68u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406eb9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (rbp) as u64); rbx = (crate::simd_ops::ru64(r14.wrapping_add(rcx))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406ee0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406ee8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406efb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406f03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rbx))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406f14); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142406f2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            10 => { rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142406f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645580; const P1: u64 = 0x142645868; const P2: u64 = 0x142645748; const P3: u64 = 0x1423f0c20; const P4: u64 = 0x142442de0; const P5: u64 = 0x141378850; const P6: u64 = 0x143406c90; const P7: u64 = 0x1423f0780; const P8: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), rbp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((rbp as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(0x130u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rcx = r14.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = P6; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14240705a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x141378930; const P4: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142407070, funcinfo_rva: 0x37df5a4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = (r9) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (r8) as u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (r9) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424070a2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = (rsi) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424070bb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rsi as u32)) as u32); }
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424074d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(6u32)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(P1))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0xc;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(6u32)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(P1))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi, rax);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378490; const P1: u64 = 0x141378730; const P2: u64 = 0x1434077d8; const P3: u64 = 0x1423fe030; const P4: u64 = 0x141378580; const P5: u64 = 0x1423f0c20; const P6: u64 = 0x142442de0; const P7: u64 = 0x1423f0b30; const P8: u64 = 0x142645868;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142407590, funcinfo_rva: 0x37df660, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (rcx) as u64; rax = ((r11.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (rax) as u64); r9 = (rdx) as u64; r8 = (rcx) as u64; rcx = ((r11.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424075b6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rax)) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, (rbp) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1000u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1000u64), (r8) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424075d6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424075e5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x1000u64))) as u64; rdi = (rbp) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbx as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240760f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); rax = ((rsp.wrapping_add(0x88u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240763f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax, (rbp) as u64); rbx = (crate::simd_ops::ru64(r14.wrapping_add(rcx))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rcx), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142407666); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240766e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142407681); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142407689); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(rbx))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240769a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424076af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            13 => { rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424076e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378730; const P1: u64 = 0x1426455b0; const P2: u64 = 0x142645580; const P3: u64 = 0x142645868; const P4: u64 = 0x142645748; const P5: u64 = 0x1423f0c20; const P6: u64 = 0x142442de0; const P7: u64 = 0x141378850; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200; const P10: u64 = 0x143406c90; const P11: u64 = 0x1423f0780; const P12: u64 = 0x14136eec0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1000u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x1000u64), rbp); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1008u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x1008u64), rbp); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x1000u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 5; continue; }
            5 => { rsi = rbp; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = ((rsi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rbp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((rbp as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(0x130u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x131u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rsi = ((rsi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 13; continue; }
            13 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rdi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rdx = rcx; r9 = rdi; rcx = r10; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rdx = P10; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x142407896, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424078a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645798; const P1: u64 = 0x142645790; const P2: u64 = 0x1423f08f0; const P3: u64 = 0x142407c80; const P4: u64 = 0x143c1bfb0; const P5: u64 = 0x143c1bfa8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424078a0, funcinfo_rva: 0x37df6d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; rdx = (0x40u32) as u32 as u64; rax = ((rcx.wrapping_add(0x4u64)) as u64) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0xffffffffu32) as u32); rax = ((rax.wrapping_add(0x40u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x1000u64), (rdi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1f0003u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x7fffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424078fd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x1008u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240790a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; rdx = (0xffffffffu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142407917); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240791c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(rbx)) as u64; r11 = (0x1u32) as u32 as u64; crate::simd_ops::wu64(r9, (r11) as u64); r8 = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = (crate::simd_ops::ru64(P5)) as u64; rdx = ((r10.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_sub(r10)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = ((r8 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru64(P5)) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(r9.wrapping_add(0xcu64), ((r11 as u8)) as u8); fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((r11 as u32)) as u32); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(P5)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424079e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407b30; const P1: u64 = 0x1426455a8; const P2: u64 = 0x108; const P3: u64 = 0x8; const P4: u64 = 0x142645598;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    rcx = rax;
    rdi = rax;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    rcx = ((rcx).wrapping_add(1)) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx);
    rcx = rdi;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407b30; const P1: u64 = 0x1426455a8; const P2: u64 = 0x108; const P3: u64 = 0x110; const P4: u64 = 0x1; const P5: u64 = 0x4; const P6: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rdi = rax;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    rcx = rdi;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rdx as u32)) as u32); } else { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P5))) as u32 as u64; fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); }
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsi, ((rbx as u32)) as u32);
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143d01040; const P1: u64 = 0x143d01048; const P2: u64 = 0x14183acc2; const P3: u64 = 0x143d01148; const P4: u64 = 0x143d01150;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    r8 = 0x108;
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = P0;
    crate::simd_ops::wu64(P3, (0x0u64) as u64);
    crate::simd_ops::wu32(P4, (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x108; const P2: u64 = 0x8; const P3: u64 = 0x142645598;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rcx = rdi;
    r8 = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_add(1)) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P1), rax);
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x108; const P2: u64 = 0x110; const P3: u64 = 0x1; const P4: u64 = 0x4; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdi = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); } else { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P4))) as u32 as u64; fl_l = ((rcx).wrapping_sub(1)) as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142407fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x142407c80; const P2: u64 = 0x143c4ac00; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax as u32)) as u32 as u64;
    r9 = P2;
    rdx = ((rdx).wrapping_shl(32u32)) as u64;
    rcx = ((r9.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64;
    r8 = ((rdi as u32)) as u32 as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rax = ((rax).wrapping_shr(6u32)) as u64;
    rdx = (((rdx) | (rbx))) as u64;
    r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64;
    r9 = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(r9)) as u64;
    fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64;
    if !(((fl_l as u64) != 0)) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((r8) as u32 & 63)))) as u64; fl_l = (((rax) >> (((r8) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r8) as u32 & 63)))) as u64; crate::simd_ops::wu64(r9, rax); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14183acc2; const P5: u64 = 0x11; const P6: u64 = 0x1426469b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    rdi = r8;
    rsi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0xffffffffu32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x4u32) as u32);
    r8 = 0xef;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((r9 as u8)) as u8);
    rcx = ((rcx).wrapping_add(0x11u64)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    r8 = rdi;
    rcx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rdx = 0xef;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x140000000; const P2: u64 = 0x142645790; const P3: u64 = 0x14263f6d0; const P4: u64 = 0x141834e48; const P5: u64 = 0x143407808; const P6: u64 = 0x1423fe030; const P7: u64 = 0x1426456d8; const P8: u64 = 0x1426456b0; const P9: u64 = 0x1426455b0; const P10: u64 = 0x143406cc0; const P11: u64 = 0x1423f0780; const P12: u64 = 0x14136eec0; const P13: u64 = 0x1424081e9;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x142408090, funcinfo_rva: 0x37df7b8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424080ab); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((rax as u32)) as u32 as u64; r15 = ((P1) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x3c4aa00u64), (rsi) as u64); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(rsi)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424080cc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424080d4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; r8 = (((r8) | (rbx))) as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shr(6u32)) as u64; r10 = ((rdi as u32)) as u32 as u64; r10 = ((((r10 as u32)) & (0x3fu32))) as u32 as u64; r9 = ((rax.wrapping_mul(8).wrapping_add(0x3c4ac00u64)) as u64) as u64; rdx = (0x1u32) as u32 as u64; rdx = ((rdx).wrapping_shl((((r10) as u32) & 63))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(r15))) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x3c4ac08u64), (rbp) as u64); rdx = (!(rdx)) as u64; rdx = (((rdx) & (crate::simd_ops::ru64(r9.wrapping_add(r15))))) as u64; fl_l = (((rdx) >> (((r10) as u32 & 63))) & 1) as u64; rdx = ((rdx) | ((1u64) << (((r10) as u32 & 63)))) as u64; crate::simd_ops::wu64(r9.wrapping_add(r15), (rdx) as u64); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x3c4ac08u64), (r8) as u64); bb = 4; continue; }
            4 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (0xa87c0u32) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rax = (0xa87c8u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx), (r14) as u64); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_add(0x8u64), ((rbp as u32)) as u32); rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142408161); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rdx = ((rsi.wrapping_add(0x11u64)) as u64) as u64; rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142408171); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (((crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240817e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru32(r15.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x2c5a8e0u64))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rdx.wrapping_add(0x60u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142408194); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = ((rdi as u32)) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424081a7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424081b0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rdx = ((P10) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424081db); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424081e4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P13, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424081f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14183acc2; const P5: u64 = 0x11; const P6: u64 = 0x1426469b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32);
    rsi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    r8 = 0xef;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0xffffffffu32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x4u32) as u32);
    rdi = r9;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    rcx = ((rcx).wrapping_add(0x11u64)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    r8 = rdi;
    rcx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rdx = 0xef;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424082a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xc; const P3: u64 = 0x14183acc2; const P4: u64 = 0x11; const P5: u64 = 0x1426469b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rdi = rdx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rsi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x4u32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rcx).wrapping_add(0x11u64)) as u64;
    r8 = 0xef;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r8 = rdi;
    rcx = ((rsi.wrapping_add(P4)) as u64) as u64;
    rdx = 0xef;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424083d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14183acc2; const P5: u64 = 0x11; const P6: u64 = 0x1426469b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    rdi = r8;
    rsi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0xffffffffu32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x4u32) as u32);
    r8 = 0xef;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8);
    rcx = ((rcx).wrapping_add(0x11u64)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    r8 = rdi;
    rcx = ((rsi.wrapping_add(P5)) as u64) as u64;
    rdx = 0xef;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142409b50; const P1: u64 = 0x14340782c; const P2: u64 = 0x14240b010; const P3: u64 = 0x140000000; const P4: u64 = 0x143407830; const P5: u64 = 0x143407828; const P6: u64 = 0x14240a630; const P7: u64 = 0x14240a590; const P8: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x70u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x170u64)) as u64; rdi = (rdx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rbx as u8)) as u8); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((r8 as u8)) as u8); bb = 5; continue; }
            2 => { r9 = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x3u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rbx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x68u64)) } else { rbx }) as u64; rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { rsi = ((P3) as u64) as u64; r14 = ((P4) as u64) as u64; r15 = ((P5) as u64) as u64; bb = 10; continue; }
            10 => { rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffff6u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x72u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(rax).wrapping_add(0x240864cu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x240863cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rsi)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x2u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 13; continue; }
            13 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x2u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 13; continue; }
            15 => { rdx = (rdx & !0xffu64) | (((0x7cu8) as u8 as u64)); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rdx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 18; continue; }
            18 => { rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x170u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x142408470u64) { 0x40 => { bb = 1; continue; } 0x4b => { bb = 2; continue; } 0x52 => { bb = 3; continue; } 0x5c => { bb = 4; continue; } 0x79 => { bb = 5; continue; } 0xdb => { bb = 6; continue; } 0xe4 => { bb = 7; continue; } 0xec => { bb = 8; continue; } 0x10d => { bb = 9; continue; } 0x122 => { bb = 10; continue; } 0x12d => { bb = 11; continue; } 0x143 => { bb = 12; continue; } 0x157 => { bb = 13; continue; } 0x16c => { bb = 14; continue; } 0x182 => { bb = 15; continue; } 0x18c => { bb = 16; continue; } 0x197 => { bb = 17; continue; } 0x1a1 => { bb = 18; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424086d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143407850; const P1: u64 = 0x142409b50; const P2: u64 = 0x142408470; const P3: u64 = 0x14240a630; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143407838;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x21u64) as u64); r9 = 0x21; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rdx }) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434078a8; const P1: u64 = 0x142409b50; const P2: u64 = 0x142408470; const P3: u64 = 0x14240a630; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143407838;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = 0x19; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rdx }) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434078a8; const P1: u64 = 0x142409b50; const P2: u64 = 0x142408470; const P3: u64 = 0x14240a630; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143407890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = 0x19; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rdx }) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434078a8; const P1: u64 = 0x142409b50; const P2: u64 = 0x142408470; const P3: u64 = 0x14240a630; const P4: u64 = 0x14240ab10; const P5: u64 = 0x143407878;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = 0x19; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x140); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rdx }) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aea0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdx = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x142409d70; const P4: u64 = 0x1401df680;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; r9 = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; rbp = r8; rsi = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; r9 = rsp.wrapping_add(0x20); r8 = 0x1; rdx = rbx; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r9 = rax; bb = 9; continue; }
            8 => { rcx = 0x1; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0x3; const P3: u64 = 0x1427e54e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = rdx; rax = rcx; r9 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r11 = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r11 = (crate::simd_ops::ru64(r11)) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); fl_l = r8; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r11))) as u32) as u32 as u64; rdi = P3; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = r8; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(r11.wrapping_add(r8))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; bb = 9; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = (0xffffffffffffffffu64) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x8; const P2: u64 = 0x1427e54e0; const P3: u64 = 0x1;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r10 = ((crate::simd_ops::ru8(r9.wrapping_add(P0))) as u32) as u32 as u64; r11 = ((r9.wrapping_add(P1)) as u64) as u64; rbx = r8; rdi = rdx; r8 = rcx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = ((r10.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r10 = (((r10 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x7c00; r10 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r11 = (crate::simd_ops::ru64(r11)) as u64; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 10; continue; }
            10 => { r8 = ((rdx.wrapping_add(rdi)) as u64) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { r9 = P2; bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(P3)) as u64) as u64; r11 = r11.wrapping_add(0x1); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r9))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(r9))) as u32) as u32 as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142408ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240aa00; const P4: u64 = 0x14240a590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx, (0xff000020u32) as u32);
    crate::simd_ops::lk_or_u32(rdx, (0x20u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x20u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    rdx = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rdx = (rdx & !0xffu64) | (((0x5cu8) as u8 as u64));
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142409700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ac30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax);
    r8 = rsp.wrapping_add(0x40);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    rdx = rcx;
    rcx = rbx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240a1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405eb9e0; const P1: u64 = 0x1426468c8; const P2: u64 = 0x142408f50; const P3: u64 = 0x1426469b8; const P4: u64 = 0x14183acc2; const P5: u64 = 0x141373300; const P6: u64 = 0x14240ab10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rax = 0x1050; rsp = ((rsp).wrapping_sub(rax)) as u64; rsi = r8; rbp = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r9 = 0xfff; r8 = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rax = 0xffffffff; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r8 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_sub(r9)) as u64; rax = rdi.wrapping_add(0x8); rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 8; continue; }
            8 => { rcx = ((r9.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x4u64), ((rbx as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 28; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x1000001u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); rdx = 0x8000; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rbx = rsp.wrapping_add(0x48); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rbx }) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(0x8000u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x8000u32) as u32); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rsp.wrapping_add(0x48); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); r9 = rbp; rdx = 0x8000; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rdx = ((r9.wrapping_add(rax)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 21; continue; }
            20 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsp.wrapping_add(0x48); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)) } else { rdx }) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((rax.wrapping_mul(8)) as u32) as u32 as u64; r10 = ((r9 as u32)) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; r9 = ((r9).wrapping_sub(r10)) as u64; rax = rdi.wrapping_add(0x8); r8 = (((r8 as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 24; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 26; continue; }
            26 => { rcx = ((r10.wrapping_add(rax)) as u64) as u64; r8 = rdx; rdx = r9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x4u64), ((rbx as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { r11 = rsp.wrapping_add(0x1050); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240ac30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378aa0; const P1: u64 = 0x142409b50; const P2: u64 = 0x1427e54e0; const P3: u64 = 0x14240ab10; const P4: u64 = 0x142409430;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14240ac30, funcinfo_rva: 0x37dfc78, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r14 = (r8) as u64; rsi = (rdx) as u64; rbp = (rcx) as u64; rdi = ((rdx.wrapping_add(0x8u64)) as u64) as u64; r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r9 = (((r9 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rax = (rdi) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240acc1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (rdi) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (r12) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = ((r13.wrapping_add(0x1u64)) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x2000002u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240ad39); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r9 = ((rsp.wrapping_add(0x78u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x78u64)) } else { r9 }) as u64; r10 = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rdx = (0xffffffffffffffffu64) as u64; bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 20; continue; }
            20 => { fl_l = (rbx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u32) as u32 as u64; r11 = ((P2) as u64) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 25; continue; }
            25 => { bb = 29; continue; }
            26 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rdi))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            28 => { rbx = (0xffffffffffffffffu64) as u64; bb = 29; continue; }
            29 => { rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240ae16); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_and_u32(rbp, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rbp, (0x20u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r12) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); r9 = (r12) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240ae59); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (r15) as u64 as u64; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { r15 }) as u64; r8 = (r15) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14240ae6e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbp) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240aea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378dc0; const P1: u64 = 0x142409b50; const P2: u64 = 0x142409430;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rdx; r14 = rcx; rdi = rdx.wrapping_add(0x8); r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = rdi; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r8 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = rdi; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = rdx; rbp = ((rbp).wrapping_sub(rax)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rsp.wrapping_add(0x30u64)))) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; crate::simd_ops::lk_and_u32(r14, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(r14, (0x20u32) as u32); crate::simd_ops::wu8(r14.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r9 = (((xmm0 >> 64) as u64)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rbx)) as u64; r8 = (0xffffffffffffffffu64) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141378c80;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    xmm1 = (crate::simd_ops::ru128(rdx)) as u128;
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8);
    rdx = rsp.wrapping_add(0x40);
    rbx = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14119f940; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x1401de830; const P4: u64 = 0x1401e5380; const P5: u64 = 0x1401e66f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 10; continue; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x58u64), ((rbx as u32)) as u32); rcx = rsi.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fc200; const P1: u64 = 0x1401e5730; const P2: u64 = 0x143426f3c; const P3: u64 = 0x143426fc8; const P4: u64 = 0x1411a13e0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0; const P8: u64 = 0x14020bcb0; const P9: u64 = 0x1426cdcb0; const P10: u64 = 0x14240b380;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (0x7u64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (0x8u64) as u64); crate::simd_ops::wu32(rdi, (0x3f800000u32) as u32); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbp); xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rbp as u32)) as u32); xmm4 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = 0u128; xmm3 = crate::simd_ops::vroundss(xmm3, (xmm1 as u32), 0x2u8); rax = 0; fl_l = 0; fl_r = 0; rdx = (0x8000000000000000u64) as u64; xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = rdx; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm3 as u32)))) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; xmm0 = 0u128; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 6; continue; }
            5 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vroundss(xmm1, (xmm0 as u32), 0x2u8); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rdx; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = 0x100000; r8 = 0x10; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rbp; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x5cu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x58u64), (1) as u32); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); r8 = rsp.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240b9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, r9);
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rdx.wrapping_add(P3), (0x10u16) as u16);
    crate::simd_ops::wu16(rcx.wrapping_add(P3), ((rax as u16)) as u16);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), r9);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P7))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240bb70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140ff8210; const P1: u64 = 0x141379500; const P2: u64 = 0x142442de0; const P3: u64 = 0x1425371a0; const P4: u64 = 0x1401f2fe0; const P5: u64 = 0x1401e5660; const P6: u64 = 0x142533200; const P7: u64 = 0x143a08320; const P8: u64 = 0x141379870;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rdx; rbx = rcx; rcx = rdx.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x2eu64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x2au64), ((rbx as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rbx); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = 0x10; crate::simd_ops::wu16(rbp.wrapping_sub(0x30u64), ((rcx as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rbx); rdx = rbp.wrapping_sub(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; r8 = rdi.wrapping_add(0x10); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; ymm0_hi = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(r8, (xmm0) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(r8, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x18u64), (0x1u8) as u8); bb = 5; continue; }
            5 => { rcx = rax; rcx = (((rcx) | (0x2u64))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdx, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 17; continue; }
            17 => { rdx = rbx; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((((rax as u8)) ^ (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            19 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x30u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { bb = 28; continue; }
            25 => { rcx = (crate::simd_ops::ru64(P7)) as u64; xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x170u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x300); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rdx); r8 = rdi; rdx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), (0x10u16) as u16); crate::simd_ops::wu16(rdx.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbx); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); bb = 28; continue; }
            28 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240be20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140ff8210; const P1: u64 = 0x1413796f0; const P2: u64 = 0x141379680; const P3: u64 = 0x14240bb70; const P4: u64 = 0x143a08320; const P5: u64 = 0x1401e5470; const P6: u64 = 0x1424968a0; const P7: u64 = 0x14240b930;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x158); rsp = ((rsp).wrapping_sub(0x258u64)) as u64; rsi = rcx; r12 = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), ((rax as u32)) as u32); rdi = rcx.wrapping_add(0x300); rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r15 = (0x6666666666666667u64) as u64; bb = 2; continue; }
            2 => { r14 = rbx.wrapping_add(0x50); rcx = rbx.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r14 = ((r14).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = r15; let prod = (rax as i64 as i128).wrapping_mul((r14) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r9 = rdx; r9 = ((((r9) as i64).wrapping_shr(5u32) as u64)) as u64; rax = r9; rax = ((rax).wrapping_shr(63u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = r15; let prod = (rax as i64 as i128).wrapping_mul((rbx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = rbp.wrapping_add(0x1b0); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b0u64))) as u64; bb = 5; continue; }
            4 => { rbx = r14; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x318u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x330u64))) as u64; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r13 = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x1b0u64), rax); rax = r8.wrapping_sub(0x1); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x320u64)))) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r15 = rax; r14 = 0x4; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x33cu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_sub(r13)) as u64; fl_l = rax; fl_r = r14; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; bb = 8; continue; }
            8 => { fl_l = r13; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x328u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x2f8u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 10; continue; }
            10 => { rbx = rsi.wrapping_add(0x1b8); crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), r12); rax = rbp; rdi = 0x10; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = rbx; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(r8, rdx); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rdx); rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu16(r8.wrapping_add(0x20u64), ((rdx as u16)) as u16); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x28u64), rdx); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), rdx); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r12); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); rcx = ((rcx).wrapping_add(0x50u64)) as u64; r8 = r8.wrapping_add(0x50); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), rax); bb = 15; continue; }
            15 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r12; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = ((r9.wrapping_add(r13)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r15) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = rdx; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x328u64)))) as u64; rax = ((rax).wrapping_add(r15)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r15) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r10 = rbp; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rax); rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(r8, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rax); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu16(r8.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(r8.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r12); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); bb = 20; continue; }
            19 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = ((rax).wrapping_add(1)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x22u64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x26u64), ((r12 as u16)) as u16); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r12); crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r12); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r12); bb = 20; continue; }
            20 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = r9; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 21; continue; }
            21 => { rbx = rbp; rdx = rbp; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rbx = r12; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 25; continue; }
            25 => { r10 = rsi.wrapping_add(0x318); rcx = rsi.wrapping_add(0x1b8); bb = 26; continue; }
            26 => { rax = ((rbx.wrapping_add(r13)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r15) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r13 = ((r12.wrapping_add(rcx)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 27; continue; }
            27 => { r9 = (crate::simd_ops::ru64(r10)) as u64; r9 = (((r9) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x320u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1b8u64), rcx); rax = rbp.wrapping_add(0x1b8); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r10)) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { rax }) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x140u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x110u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x120u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), ((xmm1 as u64)) as u64); rax = rsi.wrapping_add(0x128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = rbp.wrapping_sub(0x70); rdx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(r10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rax = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x20u64), (0x10u16) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), r8); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1a0u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), ((rax as u32)) as u32); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x30u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rdx = rsp.wrapping_add(0x40); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r13.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x0u64) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x1a0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1a0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { r9 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rcx = rsi.wrapping_add(0x1b8); r10 = rsi.wrapping_add(0x318); bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_add(1)) as u64; r12 = ((r12).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = r14; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b0u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsi.wrapping_add(0x328u64), r13); rcx = rbp; rdi = rbp; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rsp = ((rsp).wrapping_add(0x258u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240c430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x80; const P2: u64 = 0x18; const P3: u64 = 0x88; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = r8; rax = ((rbx.wrapping_add(P1)) as u64) as u64; rbp = rdx; r14 = rcx; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); r8 = rsi; crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); rdx = rbp; rcx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240c570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143407920; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P8), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(0x60u64), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(0x80u64), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240c6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2f8; const P1: u64 = 0x1e8; const P2: u64 = 0x140ff8210; const P3: u64 = 0x1401e5660; const P4: u64 = 0x318; const P5: u64 = 0x328; const P6: u64 = 0x330; const P7: u64 = 0x1b8; const P8: u64 = 0x14183acbc; const P9: u64 = 0x14240be20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = r8; r15 = rdx; rsi = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rdi = ((r14.wrapping_add(rsi)) as u64) as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdi = rbp; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = r15; r8 = ((r8).wrapping_sub(rdx)) as u64; fl_l = r8; fl_r = rbp; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdi }) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(rsi).wrapping_add(P7)))) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P6), (rdi) as u64); r15 = ((r15).wrapping_add(rdi)) as u64; fl_l = rbp; fl_r = rdi; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 11; continue; }
            11 => { rcx = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240c7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x310; const P1: u64 = 0x2; const P2: u64 = 0x33c; const P3: u64 = 0x318; const P4: u64 = 0x2f8; const P5: u64 = 0x1b8; const P6: u64 = 0x14240bb70; const P7: u64 = 0x141379680; const P8: u64 = 0x14240be20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rax = (0xaaaaaaaaaaaaaaabu64) as u64; rdx = ((r8.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P2), ((r9 as u8)) as u8); let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rsi = rcx; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((rcx.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbx = rbp; rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbp)) as u64; fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P3), r14); bb = 5; continue; }
            5 => { rcx = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240d230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143407960; const P1: u64 = 0x142417470; const P2: u64 = 0x14246ff60; const P3: u64 = 0x143407978; const P4: u64 = 0x1426455a8; const P5: u64 = 0x142645598; const P6: u64 = 0x142645590; const P7: u64 = 0x142645748; const P8: u64 = 0x143a082f0; const P9: u64 = 0x14240d430; const P10: u64 = 0x1401dffd0; const P11: u64 = 0x14240d100; const P12: u64 = 0x143a08298; const P13: u64 = 0x1426cdcb0; const P14: u64 = 0x142409b50; const P15: u64 = 0x1401de830; const P16: u64 = 0x14240ab10; const P17: u64 = 0x142442de0; const P18: u64 = 0x1401e3450; const P19: u64 = 0x1423f0c20; const P20: u64 = 0x143406c90; const P21: u64 = 0x1423f0780; const P22: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x299u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x299u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdi.wrapping_add(0x299u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            5 => { rcx = rdi.wrapping_add(0x260); rdx = 0x8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = P3; crate::simd_ops::wu64(rax, rcx); rax = ((rax).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x8u64)))) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x280u64), rax); rax = (tmp_xchg) as u64; crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x288u64), (1) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x299u64), (0x0u8) as u8); rcx = rdi.wrapping_add(0x250); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x258u64), (1) as u32); rcx = rdi.wrapping_add(0x250); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x248); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P8)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rbx.wrapping_add(0x88); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rbx.wrapping_add(0x288); rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P12, rsi); crate::simd_ops::wu8(rdi.wrapping_add(0x140u64), ((rsi as u8)) as u8); rax = ((rsi as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdi.wrapping_add(0x240u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x241u64), ((rsi as u8)) as u8); rax = ((rsi as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdi.wrapping_add(0x298u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rdi.wrapping_add(0x29c); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x3b8); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x29c); rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x268u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi.wrapping_add(0x268u64), rsi); rcx = rdi.wrapping_add(0x130); rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P19, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rdx = P20; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14240d425, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240d480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x1426455a8; const P2: u64 = 0x14183acc8; const P3: u64 = 0x142645598; const P4: u64 = 0x142645590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx.wrapping_add(0x3a8); rbp = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x3b8u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3c0u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = rbx; bb = 4; continue; }
            4 => { rbx = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = rax.wrapping_add(0x8); rax = rbx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx.wrapping_add(0x8); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, rdx); fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x3b8u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3c0u64))) as u32 as u64; rsi = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbp.wrapping_add(0x3c0u64), (0x0u32) as u32); bb = 15; continue; }
            14 => { r8 = ((r8).wrapping_sub(r8)) as u64; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_sub(rbx)) as u64; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x3c0u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3c0u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0xffffffff; rcx = rdi; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rcx = rbp.wrapping_add(0x250); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x258u64), (1) as u32); rcx = rbp.wrapping_add(0x250); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_add(0x248); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240d9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240da50; const P1: u64 = 0x142475560; const P2: u64 = 0x142646558;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rax = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = rdx;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64;
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240da50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427c70; const P1: u64 = 0x142646538; const P2: u64 = 0x142646558; const P3: u64 = 0x1438796a0; const P4: u64 = 0x1438796b8; const P5: u64 = 0x140000000; const P6: u64 = 0x14183acc8; const P7: u64 = 0x14240e600; const P8: u64 = 0x14240e330;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0; let mut xmm4: u128 = 0; let mut ymm4_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu32(r11.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (((crate::simd_ops::ru32(rsp.wrapping_add(0x110u64))) as i32 as i64) as u64) as u64; r13 = ((r8 as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; r13 = ((((r13 as u32)) as i32).wrapping_mul(((rdi as u32)) as i32) as u32) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_sub(0x40u64), (0x6u32) as u32); rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x474e5089u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0xa1a0a0du32) as u32); crate::simd_ops::wu128(r11.wrapping_sub(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x110u64), ((r13 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = ((r13 as u32)) as u32 as u64; bb = 3; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x110u64), ((r13 as u32)) as u32); bb = 3; continue; }
            3 => { rsi = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r12); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r15); rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rcx as u32)) as u32); rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r12 = ((((r13 as u32)) as i32 as i64) as u64) as u64; rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); r14 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 92; continue; } bb = 9; continue; }
            9 => { r9 = ((((rbx as u32)) as i32 as i64) as u64) as u64; r8 = P3; rcx = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rcx); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; fl_l = rbx; fl_r = rcx; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x7fffffffu32) as u32); rdx = P4; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { r8 } else { rdx }) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); bb = 11; continue; }
            11 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (r8 as u32) } else { (rsi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rsi as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 90; continue; } bb = 12; continue; }
            12 => { rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); bb = 13; continue; }
            13 => { r10 = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { r11 = r9; rax = rcx; fl_l = 0; fl_r = r11; r11 = (0u64.wrapping_sub(fl_r as u64)) as u64; r9 = r14; bb = 15; continue; }
            15 => { r8 = rax; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = P5; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x240e264u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            17 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r14; fl_r = r12; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 71; continue; } bb = 20; continue; }
            20 => { r13 = r14; r11 = ((rbx.wrapping_add(r14)) as u64) as u64; r13 = ((r13).wrapping_sub(r9)) as u64; rdx = rcx; r9 = r12; fl_l = r9; fl_r = r14; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r8 = rdx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 44; continue; } bb = 22; continue; }
            22 => { rcx = P5; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x240e280u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            23 => { bb = 24; continue; }
            24 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 25; continue; }
            25 => { r8 = rax; bb = 24; continue; }
            26 => { bb = 27; continue; }
            27 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 28; continue; }
            28 => { r8 = rax; bb = 27; continue; }
            29 => { bb = 30; continue; }
            30 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub(crate::simd_ops::ru8(r8.wrapping_add(r11)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 31; continue; }
            31 => { r8 = rax; bb = 30; continue; }
            32 => { bb = 33; continue; }
            33 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(r11))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(1u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 34; continue; }
            34 => { r8 = rax; bb = 33; continue; }
            35 => { bb = 36; continue; }
            36 => { rcx = ((crate::simd_ops::ru8(r8.wrapping_add(r11))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 37; continue; }
            37 => { r8 = rax; bb = 36; continue; }
            38 => { bb = 39; continue; }
            39 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 40; continue; }
            40 => { r8 = rax; bb = 39; continue; }
            41 => { bb = 42; continue; }
            42 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 43; continue; }
            43 => { r8 = rax; bb = 42; continue; }
            44 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 45; continue; }
            45 => { bb = 70; continue; }
            46 => { bb = 47; continue; }
            47 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 48; continue; }
            48 => { r8 = rdx; bb = 47; continue; }
            49 => { bb = 50; continue; }
            50 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(crate::simd_ops::ru8(rdx))) as u8 as u64)); rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 51; continue; }
            51 => { r8 = rdx; bb = 50; continue; }
            52 => { bb = 53; continue; }
            53 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(crate::simd_ops::ru8(r8.wrapping_add(r13)))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 54; continue; }
            54 => { r8 = rdx; bb = 53; continue; }
            55 => { bb = 56; continue; }
            56 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(r13))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 57; continue; }
            57 => { r8 = rdx; bb = 56; continue; }
            58 => { bb = 59; continue; }
            59 => { r12 = ((crate::simd_ops::ru8(rdx.wrapping_add(r13))) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(r13)) as u64) as u64; rsi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rax = rcx; rax = ((rax).wrapping_sub(r14)) as u64; r15 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r15 as u32))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = 0; fl_r = ((rdi as u32)) as u32 as u64; rdi = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rdi = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; rcx = ((rax.wrapping_add(rsi)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r12 as u32))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = 0; fl_r = ((r14 as u32)) as u32 as u64; r14 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; r14 = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r15 as u32))) as u32 as u64; rbx = ((rcx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rbx as u32)) as u32 as u64; rbx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rbx = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 61; continue; } bb = 60; continue; }
            60 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 62; continue; } bb = 61; continue; }
            61 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rsi = ((r15 as u32)) as u32 as u64; rsi = (if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { (r12 as u32) } else { (rsi as u32) }) as u32 as u64; bb = 62; continue; }
            62 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub((rsi as u8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 69; continue; } bb = 63; continue; }
            63 => { r8 = rdx; bb = 59; continue; }
            64 => { rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 65; continue; }
            65 => { r8 = rdx; bb = 64; continue; }
            66 => { bb = 67; continue; }
            67 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r14))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(crate::simd_ops::ru8(rdx))) as u8 as u64)); rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r8.wrapping_add(r11), ((rax as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 68; continue; }
            68 => { r8 = rdx; bb = 67; continue; }
            69 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; bb = 70; continue; }
            70 => { r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x110u64))) as u32 as u64; bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 88; continue; } bb = 72; continue; }
            72 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 79; continue; } bb = 73; continue; }
            73 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 79; continue; } bb = 74; continue; }
            74 => { rcx = ((r13 as u32)) as u32 as u64; xmm3 = 0u128; ymm3_hi = 0; xmm4 = 0u128; ymm4_hi = 0; fl_l = ((((rcx as u32)) & (0x8000000fu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((((rcx as u32)) | (0xfffffff0u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; bb = 76; continue; }
            76 => { rax = ((r13 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; bb = 77; continue; }
            77 => { let pm_in: u64 = crate::simd_ops::ru64(r15.wrapping_add(rdx)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovsxbd(pm_in as u128), crate::simd_ops::vpmovsxbd((pm_in >> 32) as u128)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpabsd(xmm1), crate::simd_ops::vpabsd(ymm1_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let pm_in: u64 = crate::simd_ops::ru64(r15.wrapping_add(rdx).wrapping_add(0x8u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovsxbd(pm_in as u128), crate::simd_ops::vpmovsxbd((pm_in >> 32) as u128)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm3), crate::simd_ops::vpaddd(ymm2_hi, ymm3_hi)); xmm3 = y_lo; ymm3_hi = y_hi; rdx = ((rdx).wrapping_add(0x10u64)) as u64; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpabsd(xmm1), crate::simd_ops::vpabsd(ymm1_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm4), crate::simd_ops::vpaddd(ymm2_hi, ymm4_hi)); xmm4 = y_lo; ymm4_hi = y_hi; fl_l = rdx; fl_r = rcx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 77; continue; } bb = 78; continue; }
            78 => { let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm3, xmm4), crate::simd_ops::vpaddd(ymm3_hi, ymm4_hi)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vphaddd(xmm0, xmm0), crate::simd_ops::vphaddd(ymm0_hi, ymm0_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vphaddd(xmm1, xmm1), crate::simd_ops::vphaddd(ymm1_hi, ymm1_hi)); xmm2 = y_lo; ymm2_hi = y_hi; xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; xmm0 = crate::simd_ops::vpaddd(xmm2, xmm0); ymm0_hi = 0; rdi = ((xmm0 as u32)) as u32 as u64; bb = 79; continue; }
            79 => { r8 = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = r12; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 86; continue; } bb = 80; continue; }
            80 => { rax = r12; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rax; fl_r = (0x2u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 84; continue; } bb = 81; continue; }
            81 => { r10 = r12.wrapping_sub(0x1); bb = 82; continue; }
            82 => { rax = (((crate::simd_ops::ru8(r15.wrapping_add(rdx))) as i8 as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = 0; fl_r = ((rcx as u32)) as u32 as u64; rcx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((crate::simd_ops::ru8(r15.wrapping_add(rdx).wrapping_add(0x1u64))) as i8 as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = 0; fl_r = ((rcx as u32)) as u32 as u64; rcx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = rdx; fl_r = r10; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 82; continue; } bb = 83; continue; }
            83 => { fl_l = rdx; fl_r = r12; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 85; continue; } bb = 84; continue; }
            84 => { rax = (((crate::simd_ops::ru8(rdx.wrapping_add(r15))) as i8 as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; fl_l = 0; fl_r = ((rcx as u32)) as u32 as u64; rcx = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 85; continue; }
            85 => { rax = ((r8.wrapping_add(r9)) as u32) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 86; continue; }
            86 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)) } else { (rax as u32) }) as u32 as u64; rdi = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)) } else { (rdi as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x5; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 87; continue; }
            87 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 89; continue; }
            88 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; bb = 89; continue; }
            89 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 90; continue; }
            90 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 91; continue; }
            91 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = r15; crate::simd_ops::wu8(rdi, ((r8 as u8)) as u8); rcx = rdi.wrapping_add(0x1); r8 = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; r8 = P3; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x90u64)))) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; rbx = ((rbx).wrapping_sub(r9)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 92; continue; }
            92 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; r8 = rsp.wrapping_add(0xf8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r9 = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 93; continue; }
            93 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x39u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 94; continue; }
            94 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rdx = 0xd; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rcx = (((rcx as u32)).wrapping_add(0x39u32)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(24u32) as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0xd000000u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0x52444849u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), ((rax as u8)) as u8); rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x12u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x13u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(24u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x14u64), ((rax as u8)) as u8); rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x15u64), ((rax as u8)) as u8); rax = ((rcx as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x16u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x98u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x17u64), ((rcx as u8)) as u8); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), (0x8u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x19u64), ((rax as u8)) as u8); rax = rdi.wrapping_add(0x1d); crate::simd_ops::wu16(rdi.wrapping_add(0x1au64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x1cu64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((((rax as u32)) as i32).wrapping_shr(24u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(16u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(8u32) as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x2u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xf8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x54414449u32) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; r8 = (((crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as i32 as i64) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as i32 as i64) as u64) as u64; rcx = rsi; rax = ((rax).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, (0x0u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x444e4549u32) as u32); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; bb = 7; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14240da50u64) { 0x63 => { bb = 1; continue; } 0x68 => { bb = 2; continue; } 0x70 => { bb = 3; continue; } 0xa3 => { bb = 4; continue; } 0xc4 => { bb = 5; continue; } 0xcd => { bb = 6; continue; } 0xcf => { bb = 7; continue; } 0xee => { bb = 8; continue; } 0x109 => { bb = 9; continue; } 0x140 => { bb = 10; continue; } 0x170 => { bb = 11; continue; } 0x185 => { bb = 12; continue; } 0x1a0 => { bb = 13; continue; } 0x1b2 => { bb = 14; continue; } 0x1c0 => { bb = 15; continue; } 0x1c9 => { bb = 16; continue; } 0x1dd => { bb = 17; continue; } 0x1e6 => { bb = 18; continue; } 0x1f3 => { bb = 19; continue; } 0x1fc => { bb = 20; continue; } 0x210 => { bb = 21; continue; } 0x21d => { bb = 22; continue; } 0x231 => { bb = 23; continue; } 0x240 => { bb = 24; continue; } 0x250 => { bb = 25; continue; } 0x255 => { bb = 26; continue; } 0x260 => { bb = 27; continue; } 0x274 => { bb = 28; continue; } 0x279 => { bb = 29; continue; } 0x280 => { bb = 30; continue; } 0x298 => { bb = 31; continue; } 0x29d => { bb = 32; continue; } 0x2a0 => { bb = 33; continue; } 0x2bd => { bb = 34; continue; } 0x2c2 => { bb = 35; continue; } 0x2d0 => { bb = 36; continue; } 0x2f2 => { bb = 37; continue; } 0x2f7 => { bb = 38; continue; } 0x300 => { bb = 39; continue; } 0x314 => { bb = 40; continue; } 0x319 => { bb = 41; continue; } 0x320 => { bb = 42; continue; } 0x334 => { bb = 43; continue; } 0x339 => { bb = 44; continue; } 0x346 => { bb = 45; continue; } 0x34b => { bb = 46; continue; } 0x350 => { bb = 47; continue; } 0x366 => { bb = 48; continue; } 0x36b => { bb = 49; continue; } 0x370 => { bb = 50; continue; } 0x388 => { bb = 51; continue; } 0x38d => { bb = 52; continue; } 0x390 => { bb = 53; continue; } 0x3aa => { bb = 54; continue; } 0x3af => { bb = 55; continue; } 0x3b0 => { bb = 56; continue; } 0x3d4 => { bb = 57; continue; } 0x3d9 => { bb = 58; continue; } 0x3e0 => { bb = 59; continue; } 0x424 => { bb = 60; continue; } 0x428 => { bb = 61; continue; } 0x432 => { bb = 62; continue; } 0x44c => { bb = 63; continue; } 0x451 => { bb = 64; continue; } 0x46a => { bb = 65; continue; } 0x46f => { bb = 66; continue; } 0x470 => { bb = 67; continue; } 0x484 => { bb = 68; continue; } 0x489 => { bb = 69; continue; } 0x4a3 => { bb = 70; continue; } 0x4ab => { bb = 71; continue; } 0x4b7 => { bb = 72; continue; } 0x4be => { bb = 73; continue; } 0x4c4 => { bb = 74; continue; } 0x4d7 => { bb = 75; continue; } 0x4de => { bb = 76; continue; } 0x4f0 => { bb = 77; continue; } 0x518 => { bb = 78; continue; } 0x534 => { bb = 79; continue; } 0x53f => { bb = 80; continue; } 0x54b => { bb = 81; continue; } 0x550 => { bb = 82; continue; } 0x578 => { bb = 83; continue; } 0x57d => { bb = 84; continue; } 0x58b => { bb = 85; continue; } 0x591 => { bb = 86; continue; } 0x5d8 => { bb = 87; continue; } 0x5de => { bb = 88; continue; } 0x5e3 => { bb = 89; continue; } 0x5f8 => { bb = 90; continue; } 0x607 => { bb = 91; continue; } 0x66c => { bb = 92; continue; } 0x6a8 => { bb = 93; continue; } 0x6c7 => { bb = 94; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240f720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffffc; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x1; const P3: u64 = 0x8; const P4: u64 = 0x142646560; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x8; rsi = r8; r14 = rdx; rbx = rcx; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdi = ((rdi.wrapping_mul(2).wrapping_add(P2)) as u32) as u32 as u64; rdx = ((((rdi as u32)) as i32 as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rdi = 0x2; rcx = rbp; rdx = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { rbx = ((rax.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (((crate::simd_ops::ru32(rbx.wrapping_add(P0))) as i32 as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu32(r14, ((crate::simd_ops::ru32(r14)).wrapping_shr(8u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (0xfffffff8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(rbx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = 0x8; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240f860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427e90; const P1: u64 = 0x143428770; const P2: u64 = 0x143427f10; const P3: u64 = 0x143428980; const P4: u64 = 0x1434289a0; const P5: u64 = 0x143427f20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r10 = (((crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as i32 as i64) as u64) as u64; r15 = 0; fl_l = 0; fl_r = 0; rsi = r8; r8 = (((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as i32 as i64) as u64) as u64; rbp = rdx; rdx = ((r15 as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm11) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm12) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm13) as u128); crate::simd_ops::wu128(rsp, (xmm14) as u128); xmm14 = (crate::simd_ops::ru128(P0)) as u128; xmm8 = (crate::simd_ops::ru128(P1)) as u128; xmm10 = (crate::simd_ops::ru128(P2)) as u128; xmm11 = (crate::simd_ops::ru128(P3)) as u128; xmm12 = (crate::simd_ops::ru128(P4)) as u128; xmm13 = (crate::simd_ops::ru128(P5)) as u128; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm9) as u128); xmm9 = 0u128; bb = 3; continue; }
            3 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(0x8u32)) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(r9))) as u64) as u128; xmm1 = (xmm0) ^ (xmm8); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rsi))) as u64) as u128; xmm7 = crate::simd_ops::vpunpcklbw(xmm9, xmm1); xmm1 = (xmm0) ^ (xmm8); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rbp))) as u64) as u128; xmm6 = crate::simd_ops::vpunpcklbw(xmm9, xmm1); xmm1 = crate::simd_ops::vpunpcklbw(xmm8, xmm0); xmm4 = crate::simd_ops::vpsrlw(xmm1, 0x4u64); xmm0 = crate::simd_ops::vpmulhw(xmm7, xmm10); xmm1 = crate::simd_ops::vpaddw(xmm0, xmm4); xmm3 = crate::simd_ops::vpsraw(xmm1, 0x4u64); xmm2 = crate::simd_ops::vpmulhw(xmm6, xmm13); xmm0 = crate::simd_ops::vpaddw(xmm2, xmm4); xmm1 = crate::simd_ops::vpsraw(xmm0, 0x4u64); xmm5 = crate::simd_ops::vpackuswb(xmm3, xmm1); xmm0 = crate::simd_ops::vpmulhw(xmm6, xmm12); xmm2 = crate::simd_ops::vpaddw(xmm0, xmm4); xmm1 = crate::simd_ops::vpmulhw(xmm7, xmm11); xmm2 = crate::simd_ops::vpaddw(xmm2, xmm1); xmm0 = crate::simd_ops::vpsraw(xmm2, 0x4u64); xmm3 = crate::simd_ops::vpackuswb(xmm0, xmm14); xmm2 = crate::simd_ops::vpunpcklbw(xmm5, xmm3); xmm1 = crate::simd_ops::vpunpckhbw(xmm5, xmm3); xmm0 = crate::simd_ops::vpunpcklwd(xmm2, xmm1); xmm1 = crate::simd_ops::vpunpckhwd(xmm2, xmm1); crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = ((rdx.wrapping_add(0x7u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; bb = 5; continue; }
            5 => { xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm14 = (crate::simd_ops::ru128(rsp)) as u128; bb = 6; continue; }
            6 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdi = r8; fl_l = rax; fl_r = r8; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); r11 = rcx.wrapping_add(0x2); rbp = ((rbp).wrapping_sub(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r14); rsi = ((rsi).wrapping_sub(r9)) as u64; rbx = ((rax.wrapping_add(r9)) as u64) as u64; r14 = r10; fl_l = rdi; fl_r = rax; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbx.wrapping_add(rsi))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(rbp))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(20u32)) as u32 as u64; rax = ((rdx.wrapping_sub(0x80u64)) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(0x80000u32)) as u32 as u64; r10 = ((((rax as u32)) as i32).wrapping_mul((0x166f00u32) as i32) as u32) as u32 as u64; rax = 0x80; rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x58200u32) as i32) as u32) as u32 as u64; rax = 0x80; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0xb6d00u32) as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((r9 as u32))) as u32 as u64; r10 = (((((r10 as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xffff0000u32))) as u32 as u64; rax = ((r9.wrapping_add(rcx)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((r8.wrapping_sub(0x80u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1c5a00u32) as i32) as u32) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; r10 = ((rcx as u32)) as u32 as u64; bb = 10; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu8(r11.wrapping_sub(0x2u64), ((r10 as u8)) as u8); rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r11.wrapping_sub(0x1u64), ((rdx as u8)) as u8); crate::simd_ops::wu8(r11, ((rax as u8)) as u8); crate::simd_ops::wu8(r11.wrapping_add(0x1u64), (0xffu8) as u8); r11 = ((r11).wrapping_add(r14)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240fb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 2>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rdi = (((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; rbx = r9; rbp = r8; r14 = rdx; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rax = rcx.wrapping_add(0x2); rsi = (((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; r14 = ((r14).wrapping_sub(rbx)) as u64; rbp = ((rbp).wrapping_sub(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rbx.wrapping_add(rbp))) as u32) as u32 as u64; r10 = ((crate::simd_ops::ru8(r14.wrapping_add(rbx))) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_shl(20u32)) as u32 as u64; rcx = ((r8.wrapping_sub(0x80u64)) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(0x80000u32)) as u32 as u64; r11 = ((((rcx as u32)) as i32).wrapping_mul((0x166f00u32) as i32) as u32) as u32 as u64; rcx = 0x80; rcx = (((rcx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x58200u32) as i32) as u32) as u32 as u64; rcx = 0x80; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r8 = ((((rcx as u32)) as i32).wrapping_mul((0xb6d00u32) as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((r10 as u32))) as u32 as u64; r11 = (((((r11 as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xffff0000u32))) as u32 as u64; rcx = ((r10.wrapping_add(rdx)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((r9.wrapping_sub(0x80u64)) as u32) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1c5a00u32) as i32) as u32) as u32 as u64; r8 = (((((r8 as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r10 as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(20u32) as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; r11 = ((rcx as u32)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = 0xff; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (r15 as u32) } else { (rcx as u32) }) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rax.wrapping_sub(0x2u64), ((r11 as u8)) as u8); rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu8(rax.wrapping_sub(0x1u64), ((r8 as u8)) as u8); crate::simd_ops::wu8(rax, ((rdx as u8)) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x1u64), (0xffu8) as u8); rax = ((rax).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14240fc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427e60;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; r10 = r8; rsi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdi = rcx; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x1u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; rbx = rdx; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; rbx = ((rbx).wrapping_sub(r10)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); r11 = r10.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r14 = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); rbp = r10; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(P0)) as u128; xmm6 = 0u128; r15 = ((rax as u32)) as u32 as u64; r9 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(rbp))) as u64) as u128; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r11))) as u32) as u32 as u64; rbp = rbp.wrapping_add(0x8); xmm2 = crate::simd_ops::vpunpcklbw(xmm0, xmm6); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64) as u128; xmm1 = crate::simd_ops::vpunpcklbw(xmm0, xmm6); xmm3 = crate::simd_ops::vpsubw(xmm1, xmm2); xmm2 = crate::simd_ops::vpsllw(xmm2, 0x2u64); xmm5 = crate::simd_ops::vpaddw(xmm3, xmm2); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; r14 = r14.wrapping_add(0x10); xmm0 = crate::simd_ops::vpsllw(xmm5, 0x2u64); xmm3 = crate::simd_ops::vpaddw(xmm0, xmm7); xmm1 = crate::simd_ops::vpslldq(xmm5, 0x2u8); xmm0 = (xmm1 & !(0xffffu128 << 0)) | (((((r8 as u32)) as u16) as u128) << 0); xmm2 = crate::simd_ops::vpsubw(xmm0, xmm5); xmm4 = crate::simd_ops::vpaddw(xmm2, xmm3); xmm0 = crate::simd_ops::vpsrldq(xmm5, 0x2u8); xmm0 = (xmm0 & !(0xffffu128 << 112)) | (((((rcx as u32)) as u16) as u128) << 112); xmm1 = crate::simd_ops::vpsubw(xmm0, xmm5); xmm2 = crate::simd_ops::vpaddw(xmm1, xmm3); xmm0 = crate::simd_ops::vpunpcklwd(xmm4, xmm2); xmm1 = crate::simd_ops::vpunpckhwd(xmm4, xmm2); xmm3 = crate::simd_ops::vpsrlw(xmm0, 0x4u64); xmm2 = crate::simd_ops::vpsrlw(xmm1, 0x4u64); xmm0 = crate::simd_ops::vpackuswb(xmm3, xmm2); crate::simd_ops::wu128(r14.wrapping_sub(0x10u64), (xmm0) as u128); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r11).wrapping_sub(0x1u64))) as u32) as u32 as u64; r11 = r11.wrapping_add(0x8); r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_sub(0x9u64))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; r11 = rsi; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((r9.wrapping_add(r9)) as u32) as u32 as u64; rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0x8u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(4u32)) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rdi), ((r8 as u8)) as u8); rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 7; continue; }
            7 => { r9 = ((rdx.wrapping_add(r10)) as u64) as u64; fl_l = r11; fl_r = rdx; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r10 = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x2); r8 = ((rax as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdx = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_sub(0x3u64), ((rdx as u8)) as u8); rdx = ((rax.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_sub(0x2u64), ((rdx as u8)) as u8); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = ((rsi.wrapping_add(rsi)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(2u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = rdi; crate::simd_ops::wu8(rcx.wrapping_add(rdi).wrapping_sub(0x1u64), ((rdx as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142410050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143429160; const P1: u64 = 0x1434291c0; const P2: u64 = 0x143428ca0; const P3: u64 = 0x143429480;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0; let mut xmm4: u128 = 0; let mut ymm4_hi: u128 = 0; let mut xmm5: u128 = 0; let mut ymm5_hi: u128 = 0; let mut xmm6: u128 = 0; let mut ymm6_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r10 = rdx; r11 = rcx; rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rax = ((rcx.wrapping_add(r8)) as u64) as u64; rbx = ((rcx.wrapping_add(r11)) as u64) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_add(r10)) as u64) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu128(rsp, (xmm6) as u128); let y_addr: u64 = P0; xmm5 = crate::simd_ops::ru128(y_addr); ymm5_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; xmm6 = crate::simd_ops::ru128(y_addr); ymm6_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; r14 = r10.wrapping_add(0x8); rbx = r8; rdi = r11; rcx = (((rcx) & (0xffffffffffffffe0u64))) as u64; rbx = ((rbx).wrapping_sub(r10)) as u64; rdi = ((rdi).wrapping_sub(r10)) as u64; rsi = (0xfffffffffffffff8u64) as u64; fl_l = rsi; fl_r = r10; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_sub(0x8u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm0 = y_lo; ymm0_hi = y_hi; let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(rbx).wrapping_sub(0x8u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmulld(xmm0, xmm6), crate::simd_ops::vpmulld(ymm0_hi, ymm6_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm3), crate::simd_ops::vpaddd(ymm2_hi, ymm3_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm3, xmm5), crate::simd_ops::vpaddd(ymm3_hi, ymm5_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpsrad(xmm1, 0x2u64), crate::simd_ops::vpsrad(ymm1_hi, 0x2u64)); xmm4 = y_lo; ymm4_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm4, crate::simd_ops::ru128(P2)), crate::simd_ops::vpshufb(ymm4_hi, crate::simd_ops::ru128((P2).wrapping_add(16)))); xmm2 = y_lo; ymm2_hi = y_hi; let (p_lo, p_hi) = crate::simd_ops::vpermpd(xmm2, ymm2_hi, 0xd8u8); xmm3 = p_lo; ymm3_hi = p_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm3, crate::simd_ops::ru128(P3)), crate::simd_ops::vpshufb(ymm3_hi, crate::simd_ops::ru128((P3).wrapping_add(16)))); xmm0 = y_lo; ymm0_hi = y_hi; let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(rbx)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm3 = y_lo; ymm3_hi = y_hi; crate::simd_ops::wu64(r14.wrapping_add(rdi).wrapping_sub(0x8u64), ((xmm0 as u64)) as u64); let pm_in: u64 = crate::simd_ops::ru64(r14); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmulld(xmm0, xmm6), crate::simd_ops::vpmulld(ymm0_hi, ymm6_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm3), crate::simd_ops::vpaddd(ymm2_hi, ymm3_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm3, xmm5), crate::simd_ops::vpaddd(ymm3_hi, ymm5_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpsrad(xmm1, 0x2u64), crate::simd_ops::vpsrad(ymm1_hi, 0x2u64)); xmm4 = y_lo; ymm4_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm4, crate::simd_ops::ru128(P2)), crate::simd_ops::vpshufb(ymm4_hi, crate::simd_ops::ru128((P2).wrapping_add(16)))); xmm2 = y_lo; ymm2_hi = y_hi; let (p_lo, p_hi) = crate::simd_ops::vpermpd(xmm2, ymm2_hi, 0xd8u8); xmm3 = p_lo; ymm3_hi = p_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm3, crate::simd_ops::ru128(P3)), crate::simd_ops::vpshufb(ymm3_hi, crate::simd_ops::ru128((P3).wrapping_add(16)))); xmm0 = y_lo; ymm0_hi = y_hi; let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(rbx).wrapping_add(0x8u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm3 = y_lo; ymm3_hi = y_hi; crate::simd_ops::wu64(r14.wrapping_add(rdi), ((xmm0 as u64)) as u64); let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(0x8u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmulld(xmm0, xmm6), crate::simd_ops::vpmulld(ymm0_hi, ymm6_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm3), crate::simd_ops::vpaddd(ymm2_hi, ymm3_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm3, xmm5), crate::simd_ops::vpaddd(ymm3_hi, ymm5_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpsrad(xmm1, 0x2u64), crate::simd_ops::vpsrad(ymm1_hi, 0x2u64)); xmm4 = y_lo; ymm4_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm4, crate::simd_ops::ru128(P2)), crate::simd_ops::vpshufb(ymm4_hi, crate::simd_ops::ru128((P2).wrapping_add(16)))); xmm2 = y_lo; ymm2_hi = y_hi; let (p_lo, p_hi) = crate::simd_ops::vpermpd(xmm2, ymm2_hi, 0xd8u8); xmm3 = p_lo; ymm3_hi = p_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm3, crate::simd_ops::ru128(P3)), crate::simd_ops::vpshufb(ymm3_hi, crate::simd_ops::ru128((P3).wrapping_add(16)))); xmm0 = y_lo; ymm0_hi = y_hi; let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(rbx).wrapping_add(0x10u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm3 = y_lo; ymm3_hi = y_hi; crate::simd_ops::wu64(r14.wrapping_add(rdi).wrapping_add(0x8u64), ((xmm0 as u64)) as u64); let pm_in: u64 = crate::simd_ops::ru64(r14.wrapping_add(0x10u64)); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmovzxbd(pm_in as u128), crate::simd_ops::vpmovzxbd((pm_in >> 32) as u128)); xmm0 = y_lo; ymm0_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpmulld(xmm0, xmm6), crate::simd_ops::vpmulld(ymm0_hi, ymm6_hi)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm2, xmm3), crate::simd_ops::vpaddd(ymm2_hi, ymm3_hi)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpaddd(xmm3, xmm5), crate::simd_ops::vpaddd(ymm3_hi, ymm5_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpsrad(xmm1, 0x2u64), crate::simd_ops::vpsrad(ymm1_hi, 0x2u64)); xmm4 = y_lo; ymm4_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm4, crate::simd_ops::ru128(P2)), crate::simd_ops::vpshufb(ymm4_hi, crate::simd_ops::ru128((P2).wrapping_add(16)))); xmm2 = y_lo; ymm2_hi = y_hi; let (p_lo, p_hi) = crate::simd_ops::vpermpd(xmm2, ymm2_hi, 0xd8u8); xmm3 = p_lo; ymm3_hi = p_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpshufb(xmm3, crate::simd_ops::ru128(P3)), crate::simd_ops::vpshufb(ymm3_hi, crate::simd_ops::ru128((P3).wrapping_add(16)))); xmm0 = y_lo; ymm0_hi = y_hi; crate::simd_ops::wu64(r14.wrapping_add(rdi).wrapping_add(0x10u64), ((xmm0 as u64)) as u64); r14 = r14.wrapping_add(0x20); rdx = (((rdx as u32)).wrapping_add(0x20u32)) as u32 as u64; rax = ((rsi.wrapping_add(r14)) as u64) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp)) as u128; ymm6_hi = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 9; continue; }
            9 => { rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = ((((r9 as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = r9; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = r11; rax = ((rcx.wrapping_add(r8)) as u64) as u64; rbx = ((rbx).wrapping_sub(r8)) as u64; r10 = ((r10).wrapping_sub(r8)) as u64; fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r10))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = rax.wrapping_add(0x1); r8 = (((r8 as u32)).wrapping_add(0x2u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(2u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax).wrapping_sub(0x1u64), ((r8 as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = r11; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142410280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428970; const P1: u64 = 0x143427ec0; const P2: u64 = 0x143427ef0; const P3: u64 = 0x143428960; const P4: u64 = 0x143428950; const P5: u64 = 0x143428990; const P6: u64 = 0x143427ed0; const P7: u64 = 0x143427ee0; const P8: u64 = 0x143427e50; const P9: u64 = 0x143427ea0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0x1a8u64)) as u64; xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(0x20u64))) as u128; xmm2 = crate::simd_ops::vpunpcklwd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(0x60u64))); xmm3 = crate::simd_ops::vpunpckhwd(xmm1, crate::simd_ops::ru128(r8.wrapping_add(0x60u64))); xmm0 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P0)); xmm5 = (crate::simd_ops::ru128(r8)) as u128; xmm1 = crate::simd_ops::vpaddw(xmm5, crate::simd_ops::ru128(r8.wrapping_add(0x40u64))); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P0)); crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P1)); xmm2 = crate::simd_ops::vpsubw(xmm5, crate::simd_ops::ru128(r8.wrapping_add(0x40u64))); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P1)); crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm12) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x88u64), (xmm13) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x98u64), (xmm14) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0xa8u64), (xmm15) as u128); xmm12 = (crate::simd_ops::ru128(r8.wrapping_add(0x10u64))) as u128; xmm13 = (crate::simd_ops::ru128(r8.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(r8.wrapping_add(0x50u64))) as u128; xmm6 = (crate::simd_ops::ru128(r8.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm3 = 0u128; xmm0 = crate::simd_ops::vpunpcklwd(xmm3, xmm1); xmm15 = crate::simd_ops::vpsrad(xmm0, 0x4u64); xmm1 = crate::simd_ops::vpunpckhwd(xmm3, xmm1); xmm0 = crate::simd_ops::vpsrad(xmm1, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = crate::simd_ops::vpunpckhwd(xmm3, xmm2); xmm7 = crate::simd_ops::vpunpcklwd(xmm6, xmm13); xmm9 = crate::simd_ops::vpunpckhwd(xmm6, xmm13); xmm0 = crate::simd_ops::vpunpcklwd(xmm3, xmm2); xmm0 = crate::simd_ops::vpsrad(xmm0, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpsrad(xmm1, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = crate::simd_ops::vpaddw(xmm6, xmm12); xmm0 = crate::simd_ops::vpaddw(xmm8, xmm13); xmm2 = crate::simd_ops::vpunpcklwd(xmm1, xmm0); xmm6 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P2)); xmm4 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P3)); xmm3 = crate::simd_ops::vpunpckhwd(xmm1, xmm0); xmm5 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P3)); xmm1 = crate::simd_ops::vpmaddwd(xmm9, crate::simd_ops::ru128(P4)); xmm0 = crate::simd_ops::vpmaddwd(xmm7, crate::simd_ops::ru128(P4)); xmm0 = crate::simd_ops::vpaddd(xmm0, xmm6); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm10 = crate::simd_ops::vpunpcklwd(xmm8, xmm12); xmm11 = crate::simd_ops::vpunpckhwd(xmm8, xmm12); xmm8 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P2)); xmm0 = crate::simd_ops::vpaddd(xmm1, xmm8); xmm1 = crate::simd_ops::vpmaddwd(xmm11, crate::simd_ops::ru128(P5)); xmm13 = crate::simd_ops::vpaddd(xmm1, xmm5); xmm1 = crate::simd_ops::vpmaddwd(xmm9, crate::simd_ops::ru128(P6)); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm10, crate::simd_ops::ru128(P5)); xmm14 = crate::simd_ops::vpaddd(xmm0, xmm4); xmm0 = crate::simd_ops::vpmaddwd(xmm7, crate::simd_ops::ru128(P6)); xmm9 = crate::simd_ops::vpaddd(xmm1, xmm5); xmm1 = crate::simd_ops::vpmaddwd(xmm11, crate::simd_ops::ru128(P7)); xmm12 = crate::simd_ops::vpaddd(xmm0, xmm4); xmm0 = crate::simd_ops::vpmaddwd(xmm10, crate::simd_ops::ru128(P7)); xmm10 = (crate::simd_ops::ru128(P8)) as u128; xmm7 = crate::simd_ops::vpaddd(xmm0, xmm6); xmm0 = crate::simd_ops::vpaddd(xmm15, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm6 = crate::simd_ops::vpaddd(xmm1, xmm8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))); xmm4 = crate::simd_ops::vpaddd(xmm1, xmm10); xmm5 = crate::simd_ops::vpaddd(xmm0, xmm10); xmm1 = crate::simd_ops::vpaddd(xmm4, xmm6); xmm0 = crate::simd_ops::vpaddd(xmm5, xmm7); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm15) as u128); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm15 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm1 = crate::simd_ops::vpsubd(xmm4, xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm0 = crate::simd_ops::vpsubd(xmm5, xmm7); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm1 = crate::simd_ops::vpaddd(xmm7, crate::simd_ops::ru128(rsp)); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm0 = crate::simd_ops::vpackssdw(xmm3, xmm2); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpaddd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))); xmm5 = crate::simd_ops::vpaddd(xmm0, xmm10); xmm4 = crate::simd_ops::vpaddd(xmm1, xmm10); xmm1 = crate::simd_ops::vpaddd(xmm4, xmm9); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm1 = crate::simd_ops::vpsubd(xmm4, xmm9); xmm0 = crate::simd_ops::vpaddd(xmm5, xmm12); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm10 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(rsp)); xmm0 = crate::simd_ops::vpsubd(xmm5, xmm12); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm0 = crate::simd_ops::vpsubd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))); xmm6 = (crate::simd_ops::ru128(P8)) as u128; xmm5 = crate::simd_ops::vpaddd(xmm0, xmm6); xmm11 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm4 = crate::simd_ops::vpaddd(xmm1, xmm6); xmm1 = crate::simd_ops::vpaddd(xmm4, xmm13); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm1 = crate::simd_ops::vpsubd(xmm4, xmm13); xmm0 = crate::simd_ops::vpaddd(xmm5, xmm14); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm8 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm0 = crate::simd_ops::vpsubd(xmm5, xmm14); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm0 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm5 = crate::simd_ops::vpaddd(xmm0, xmm6); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))); xmm0 = crate::simd_ops::vpaddd(xmm5, crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))); xmm7 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm4 = crate::simd_ops::vpaddd(xmm1, xmm6); xmm1 = crate::simd_ops::vpaddd(xmm4, crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm0 = crate::simd_ops::vpsubd(xmm5, crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm1 = crate::simd_ops::vpsubd(xmm4, crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))); xmm6 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm3 = crate::simd_ops::vpsrad(xmm0, 0xau64); xmm5 = crate::simd_ops::vpunpcklwd(xmm10, xmm7); xmm2 = crate::simd_ops::vpsrad(xmm1, 0xau64); xmm1 = crate::simd_ops::vpunpcklwd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))); xmm0 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm3 = crate::simd_ops::vpunpckhwd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))); xmm4 = crate::simd_ops::vpunpcklwd(xmm15, xmm0); xmm9 = crate::simd_ops::vpunpckhwd(xmm15, xmm0); xmm2 = crate::simd_ops::vpunpckhwd(xmm8, xmm11); xmm10 = crate::simd_ops::vpunpckhwd(xmm10, xmm7); xmm7 = crate::simd_ops::vpunpcklwd(xmm9, xmm2); xmm9 = crate::simd_ops::vpunpckhwd(xmm9, xmm2); xmm0 = crate::simd_ops::vpunpcklwd(xmm8, xmm11); xmm6 = crate::simd_ops::vpunpcklwd(xmm4, xmm0); xmm8 = crate::simd_ops::vpunpckhwd(xmm4, xmm0); xmm0 = crate::simd_ops::vpunpcklwd(xmm5, xmm1); xmm4 = crate::simd_ops::vpunpckhwd(xmm5, xmm1); xmm1 = crate::simd_ops::vpunpcklwd(xmm10, xmm3); xmm5 = crate::simd_ops::vpunpckhwd(xmm10, xmm3); xmm10 = crate::simd_ops::vpunpcklwd(xmm6, xmm0); xmm13 = crate::simd_ops::vpunpckhwd(xmm6, xmm0); xmm0 = crate::simd_ops::vpunpcklwd(xmm9, xmm5); xmm2 = crate::simd_ops::vpunpcklwd(xmm8, xmm4); xmm3 = crate::simd_ops::vpunpcklwd(xmm7, xmm1); xmm6 = crate::simd_ops::vpunpckhwd(xmm7, xmm1); xmm1 = crate::simd_ops::vpunpcklwd(xmm2, xmm0); xmm2 = crate::simd_ops::vpunpckhwd(xmm2, xmm0); xmm0 = crate::simd_ops::vpmaddwd(xmm1, crate::simd_ops::ru128(P0)); xmm12 = crate::simd_ops::vpunpckhwd(xmm8, xmm4); xmm4 = crate::simd_ops::vpunpckhwd(xmm9, xmm5); xmm15 = crate::simd_ops::vpmaddwd(xmm1, crate::simd_ops::ru128(P1)); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P0)); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P1)); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); xmm2 = crate::simd_ops::vpsubw(xmm10, xmm3); xmm1 = crate::simd_ops::vpaddw(xmm3, xmm10); xmm10 = crate::simd_ops::vpunpcklwd(xmm6, xmm13); xmm5 = 0u128; xmm0 = crate::simd_ops::vpunpcklwd(xmm5, xmm1); xmm14 = crate::simd_ops::vpsrad(xmm0, 0x4u64); xmm1 = crate::simd_ops::vpunpckhwd(xmm5, xmm1); xmm0 = crate::simd_ops::vpsrad(xmm1, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); xmm1 = crate::simd_ops::vpunpckhwd(xmm5, xmm2); xmm0 = crate::simd_ops::vpunpcklwd(xmm5, xmm2); xmm0 = crate::simd_ops::vpsrad(xmm0, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpsrad(xmm1, 0x4u64); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpaddw(xmm6, xmm12); xmm1 = crate::simd_ops::vpaddw(xmm4, xmm13); xmm2 = crate::simd_ops::vpunpcklwd(xmm1, xmm0); xmm3 = crate::simd_ops::vpunpckhwd(xmm1, xmm0); xmm7 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P2)); xmm5 = crate::simd_ops::vpmaddwd(xmm3, crate::simd_ops::ru128(P3)); xmm3 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = crate::simd_ops::vpunpcklwd(xmm4, xmm12); xmm0 = crate::simd_ops::vpmaddwd(xmm8, crate::simd_ops::ru128(P4)); xmm11 = crate::simd_ops::vpunpckhwd(xmm6, xmm13); xmm6 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P2)); xmm0 = crate::simd_ops::vpaddd(xmm0, xmm6); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); xmm9 = crate::simd_ops::vpunpckhwd(xmm4, xmm12); xmm4 = crate::simd_ops::vpmaddwd(xmm2, crate::simd_ops::ru128(P3)); xmm1 = crate::simd_ops::vpmaddwd(xmm9, crate::simd_ops::ru128(P4)); xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm0 = crate::simd_ops::vpaddd(xmm1, xmm7); xmm1 = crate::simd_ops::vpmaddwd(xmm11, crate::simd_ops::ru128(P5)); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm10, crate::simd_ops::ru128(P5)); xmm0 = crate::simd_ops::vpaddd(xmm0, xmm4); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpaddd(xmm1, xmm5); xmm1 = crate::simd_ops::vpmaddwd(xmm9, crate::simd_ops::ru128(P6)); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm8, crate::simd_ops::ru128(P6)); xmm0 = crate::simd_ops::vpaddd(xmm0, xmm4); crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpmaddwd(xmm10, crate::simd_ops::ru128(P7)); xmm10 = (crate::simd_ops::ru128(P9)) as u128; xmm8 = crate::simd_ops::vpaddd(xmm1, xmm5); xmm1 = crate::simd_ops::vpmaddwd(xmm11, crate::simd_ops::ru128(P7)); xmm5 = crate::simd_ops::vpaddd(xmm0, xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm4 = crate::simd_ops::vpaddd(xmm1, xmm7); xmm1 = crate::simd_ops::vpaddd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))); xmm0 = crate::simd_ops::vpaddd(xmm14, xmm15); crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm15) as u128); xmm15 = crate::simd_ops::vpaddd(xmm0, xmm10); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))); xmm13 = crate::simd_ops::vpaddd(xmm0, xmm10); xmm0 = crate::simd_ops::vpsubd(xmm2, crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))); xmm2 = (crate::simd_ops::ru128(P9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm14) as u128); xmm14 = crate::simd_ops::vpaddd(xmm1, xmm10); xmm1 = crate::simd_ops::vpaddd(xmm3, crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))); xmm12 = crate::simd_ops::vpaddd(xmm1, xmm10); xmm1 = crate::simd_ops::vpsubd(xmm3, crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))); xmm10 = crate::simd_ops::vpaddd(xmm0, xmm10); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))); xmm9 = crate::simd_ops::vpaddd(xmm1, xmm2); xmm1 = crate::simd_ops::vpsubd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))); xmm7 = crate::simd_ops::vpaddd(xmm0, xmm2); xmm0 = crate::simd_ops::vpaddd(xmm15, xmm5); crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xe0u64), (xmm5) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xf0u64), (xmm4) as u128); xmm6 = crate::simd_ops::vpaddd(xmm1, xmm2); xmm3 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm0 = crate::simd_ops::vpaddd(xmm13, crate::simd_ops::ru128(rsp)); xmm1 = crate::simd_ops::vpaddd(xmm14, xmm4); xmm4 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm2 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm5 = crate::simd_ops::vpackssdw(xmm3, xmm2); rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r11 = r11; xmm1 = crate::simd_ops::vpaddd(xmm12, xmm8); xmm2 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpaddd(xmm10, crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))); xmm0 = crate::simd_ops::vpackssdw(xmm4, xmm2); xmm11 = crate::simd_ops::vpackuswb(xmm5, xmm0); xmm0 = crate::simd_ops::vpaddd(xmm9, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm2 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm0 = crate::simd_ops::vpaddd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))); xmm3 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpaddd(xmm7, crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))); xmm5 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm2 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm0 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))); xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm3 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm0 = crate::simd_ops::vpsubd(xmm10, crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm4 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpackssdw(xmm4, xmm2); xmm8 = crate::simd_ops::vpackuswb(xmm5, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))); xmm4 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm2 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpsubd(xmm9, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm5 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm2 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpsubd(xmm13, crate::simd_ops::ru128(rsp)); xmm0 = crate::simd_ops::vpackssdw(xmm4, xmm2); xmm6 = crate::simd_ops::vpackuswb(xmm5, xmm0); xmm0 = crate::simd_ops::vpsubd(xmm12, crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))); xmm2 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm0 = crate::simd_ops::vpsubd(xmm14, crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))); xmm3 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpsubd(xmm15, crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))); xmm5 = crate::simd_ops::vpackssdw(xmm3, xmm2); xmm2 = crate::simd_ops::vpsrad(xmm0, 0x11u64); xmm4 = crate::simd_ops::vpsrad(xmm1, 0x11u64); xmm1 = crate::simd_ops::vpackssdw(xmm4, xmm2); xmm3 = crate::simd_ops::vpackuswb(xmm5, xmm1); xmm1 = crate::simd_ops::vpunpckhbw(xmm8, xmm3); xmm0 = crate::simd_ops::vpunpcklbw(xmm8, xmm3); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm2 = crate::simd_ops::vpunpcklbw(xmm11, xmm6); xmm3 = crate::simd_ops::vpunpcklbw(xmm2, xmm0); xmm5 = crate::simd_ops::vpunpckhbw(xmm2, xmm0); xmm4 = crate::simd_ops::vpunpckhbw(xmm11, xmm6); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm2 = crate::simd_ops::vpunpckhbw(xmm4, xmm1); xmm0 = crate::simd_ops::vpunpcklbw(xmm4, xmm1); xmm1 = crate::simd_ops::vpunpcklbw(xmm3, xmm0); crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); rcx = ((rcx).wrapping_add(rax)) as u64; xmm4 = crate::simd_ops::vpunpckhbw(xmm3, xmm0); xmm3 = crate::simd_ops::vpunpcklbw(xmm5, xmm2); xmm0 = crate::simd_ops::vpshufd(xmm1, 0x4eu8); crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rcx = ((rcx).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vpshufd(xmm4, 0x4eu8); xmm2 = crate::simd_ops::vpunpckhbw(xmm5, xmm2); crate::simd_ops::wu64(rcx, ((xmm4 as u64)) as u64); rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rcx = ((rcx).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vpshufd(xmm3, 0x4eu8); crate::simd_ops::wu64(rcx, ((xmm3 as u64)) as u64); rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = crate::simd_ops::vpshufd(xmm2, 0x4eu8); crate::simd_ops::wu64(rcx, ((xmm2 as u64)) as u64); crate::simd_ops::wu64(rax.wrapping_add(rcx), ((xmm0 as u64)) as u64); xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x88u64))) as u128; xmm14 = (crate::simd_ops::ru128(r11.wrapping_sub(0x98u64))) as u128; xmm15 = (crate::simd_ops::ru128(r11.wrapping_sub(0xa8u64))) as u128; rsp = r11; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424109c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x108u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), (0x4u64) as u64); rax = r8.wrapping_add(0x20); bb = 1; continue; }
            1 => { rbx = (((crate::simd_ops::ru16(rax.wrapping_sub(0x10u64))) as i16 as i32) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x10u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x30u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x40u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x50u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru16(rax.wrapping_sub(0x20u64))) as i16 as i32) as u32) as u32 as u64; fl_l = (((rcx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((rcx as u32)) as u32); r13 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), ((rcx as u32)) as u32); rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp, ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rcx as u32)) as u32); bb = 10; continue; }
            9 => { r11 = (((crate::simd_ops::ru16(rax.wrapping_add(0x10u64))) as i16 as i32) as u32) as u32 as u64; r9 = (((crate::simd_ops::ru16(rax)) as i16 as i32) as u32) as u32 as u64; rdx = (((crate::simd_ops::ru16(rax.wrapping_add(0x40u64))) as i16 as i32) as u32) as u32 as u64; r14 = (((crate::simd_ops::ru16(rax.wrapping_add(0x50u64))) as i16 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru16(rax.wrapping_add(0x30u64))) as i16 as i32) as u32) as u32 as u64; r13 = (((crate::simd_ops::ru16(rax.wrapping_sub(0x20u64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(r9)) as u32) as u32 as u64; r8 = ((((rcx as u32)) as i32).wrapping_mul((0x8a9u32) as i32) as u32) as u32 as u64; rcx = ((((rdx as u32)) as i32).wrapping_mul((0x1d8fu32) as i32) as u32) as u32 as u64; rdx = ((((r9 as u32)) as i32).wrapping_mul((0xc3fu32) as i32) as u32) as u32 as u64; r9 = ((rsi.wrapping_add(rbx)) as u32) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as i16 as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; r8 = ((rcx.wrapping_add(r13)) as u32) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(12u32)) as u32 as u64; r13 = (((r13 as u32)).wrapping_shl(12u32)) as u32 as u64; r15 = ((r8.wrapping_add(rdx)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r12 = ((r10.wrapping_add(r13)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x160u64), ((r8 as u32)) as u32); r13 = (((r13 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r8 = ((r14.wrapping_add(r11)) as u32) as u32 as u64; rcx = ((r9.wrapping_add(r8)) as u32) as u32 as u64; r10 = ((rbx as u32)) as u32 as u64; rdi = ((((rcx as u32)) as i32).wrapping_mul((0x12d0u32) as i32) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((r14.wrapping_add(rbx)) as u32) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(0x200u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xe65u32) as i32) as u32) as u32 as u64; r13 = (((r13 as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((rsi.wrapping_add(r11)) as u32) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x2901u32) as i32) as u32) as u32 as u64; rcx = ((((r9 as u32)) as i32).wrapping_mul((0xfffff9c3u32) as i32) as u32) as u32 as u64; r9 = ((((r11 as u32)) as i32).wrapping_mul((0x312au32) as i32) as u32) as u32 as u64; r11 = ((((r14 as u32)) as i32).wrapping_mul((0x4c7u32) as i32) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((((r8 as u32)) as i32).wrapping_mul((0xffffe09eu32) as i32) as u32) as u32 as u64; r8 = ((((r10 as u32)) as i32).wrapping_mul((0x1805u32) as i32) as u32) as u32 as u64; r10 = ((((rsi as u32)) as i32).wrapping_mul((0x20dau32) as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((rbx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x160u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rbx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdi as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rdi as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((r15.wrapping_add(r8)) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rcx as u32)) as u32); rcx = ((r12.wrapping_add(r9)) as u32) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rcx as u32)) as u32); rcx = ((r10.wrapping_add(r13)) as u32) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = ((rdx.wrapping_add(r11)) as u32) as u32 as u64; r15 = (((((r15 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r12 = (((((r12 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r13 = (((((r13 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; fl_l = (((((rdx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), ((r12 as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x80u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rcx as u32)) as u32); rbx = (((crate::simd_ops::ru16(rax.wrapping_sub(0xeu64))) as i16 as i32) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x2u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x12u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x22u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x32u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x42u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x52u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (((crate::simd_ops::ru16(rax.wrapping_sub(0x1eu64))) as i16 as i32) as u32) as u32 as u64; fl_l = (((rcx as u32)).wrapping_shl(2u32)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc4u64), ((rcx as u32)) as u32); r13 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xa4u64), ((rcx as u32)) as u32); rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rcx as u32)) as u32); bb = 19; continue; }
            18 => { r11 = (((crate::simd_ops::ru16(rax.wrapping_add(0x12u64))) as i16 as i32) as u32) as u32 as u64; r9 = (((crate::simd_ops::ru16(rax.wrapping_add(0x2u64))) as i16 as i32) as u32) as u32 as u64; rdx = (((crate::simd_ops::ru16(rax.wrapping_add(0x42u64))) as i16 as i32) as u32) as u32 as u64; r14 = (((crate::simd_ops::ru16(rax.wrapping_add(0x52u64))) as i16 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru16(rax.wrapping_add(0x32u64))) as i16 as i32) as u32) as u32 as u64; r13 = (((crate::simd_ops::ru16(rax.wrapping_sub(0x1eu64))) as i16 as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(r9)) as u32) as u32 as u64; r8 = ((((rcx as u32)) as i32).wrapping_mul((0x8a9u32) as i32) as u32) as u32 as u64; rcx = ((((rdx as u32)) as i32).wrapping_mul((0x1d8fu32) as i32) as u32) as u32 as u64; rdx = ((((r9 as u32)) as i32).wrapping_mul((0xc3fu32) as i32) as u32) as u32 as u64; r9 = ((rsi.wrapping_add(rbx)) as u32) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((crate::simd_ops::ru16(rax.wrapping_add(0x22u64))) as i16 as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; r8 = ((rcx.wrapping_add(r13)) as u32) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(12u32)) as u32 as u64; r13 = (((r13 as u32)).wrapping_shl(12u32)) as u32 as u64; r15 = ((r8.wrapping_add(rdx)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r12 = ((r10.wrapping_add(r13)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x160u64), ((r8 as u32)) as u32); r13 = (((r13 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r8 = ((r14.wrapping_add(r11)) as u32) as u32 as u64; rcx = ((r9.wrapping_add(r8)) as u32) as u32 as u64; r10 = ((rbx as u32)) as u32 as u64; rdi = ((((rcx as u32)) as i32).wrapping_mul((0x12d0u32) as i32) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((r14.wrapping_add(rbx)) as u32) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(0x200u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0xe65u32) as i32) as u32) as u32 as u64; r13 = (((r13 as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((rsi.wrapping_add(r11)) as u32) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x2901u32) as i32) as u32) as u32 as u64; rcx = ((((r9 as u32)) as i32).wrapping_mul((0xfffff9c3u32) as i32) as u32) as u32 as u64; r9 = ((((r11 as u32)) as i32).wrapping_mul((0x312au32) as i32) as u32) as u32 as u64; r11 = ((((r14 as u32)) as i32).wrapping_mul((0x4c7u32) as i32) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((((r8 as u32)) as i32).wrapping_mul((0xffffe09eu32) as i32) as u32) as u32 as u64; r8 = ((((r10 as u32)) as i32).wrapping_mul((0x1805u32) as i32) as u32) as u32 as u64; r10 = ((((rsi as u32)) as i32).wrapping_mul((0x20dau32) as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((rbx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x160u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rcx as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rbx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdi as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rdi as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x200u32)) as u32 as u64; rcx = ((r15.wrapping_add(r8)) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rcx as u32)) as u32); rcx = ((r12.wrapping_add(r9)) as u32) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = ((r10.wrapping_add(r13)) as u32) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rcx as u32)) as u32); rcx = ((rdx.wrapping_add(r11)) as u32) as u32 as u64; r15 = (((((r15 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r12 = (((((r12 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; r13 = (((((r13 as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; fl_l = (((((rdx as u32)) as i32).wrapping_shr(10u32) as u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc4u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xa4u64), ((r12 as u32)) as u32); bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), ((rdx as u32)) as u32); rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x84u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((rcx as u32)) as u32); rbp = ((rbp).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 20; continue; }
            20 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; r11 = rsp.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), r11); fl_l = ((r13).wrapping_add(1)) as u64; r13 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), (0x8u64) as u64); bb = 21; continue; }
            21 => { r8 = (crate::simd_ops::ru32(r11.wrapping_sub(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r11)) as u32 as u64; rsi = (crate::simd_ops::ru32(r11.wrapping_add(0x4u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r11.wrapping_sub(0x4u64))) as u32 as u64; r10 = (crate::simd_ops::ru32(r11.wrapping_sub(0xcu64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r11.wrapping_sub(0x18u64))) as u32 as u64; rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0x8a9u32) as i32) as u32) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1d8fu32) as i32) as u32) as u32 as u64; rcx = ((((r8 as u32)) as i32).wrapping_mul((0xc3fu32) as i32) as u32) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(r11.wrapping_sub(0x8u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rsi.wrapping_add(r10)) as u32) as u32 as u64; r15 = ((rax.wrapping_add(r14)) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rax as u32))) as u32 as u64; r14 = (((r14 as u32)).wrapping_shl(12u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_shl(12u32)) as u32 as u64; r12 = ((r14.wrapping_add(r9)) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r9 = (crate::simd_ops::ru32(r11.wrapping_sub(0x14u64))) as u32 as u64; rbp = ((r15.wrapping_add(rcx)) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(0x1010000u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(0x1010000u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(0x1010000u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x1010000u32)) as u32 as u64; r8 = ((rdi.wrapping_add(r9)) as u32) as u32 as u64; rax = ((r8.wrapping_add(rdx)) as u32) as u32 as u64; rbx = ((((rax as u32)) as i32).wrapping_mul((0x12d0u32) as i32) as u32) as u32 as u64; rax = ((rsi.wrapping_add(r9)) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xe65u32) as i32) as u32) as u32 as u64; rax = ((rdi.wrapping_add(r10)) as u32) as u32 as u64; r11 = ((rbx as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x2901u32) as i32) as u32) as u32 as u64; rax = ((((r8 as u32)) as i32).wrapping_mul((0xfffff9c3u32) as i32) as u32) as u32 as u64; r8 = ((((rdi as u32)) as i32).wrapping_mul((0x20dau32) as i32) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((((rdx as u32)) as i32).wrapping_mul((0xffffe09eu32) as i32) as u32) as u32 as u64; rdx = ((((r9 as u32)) as i32).wrapping_mul((0x1805u32) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; r9 = ((((rsi as u32)) as i32).wrapping_mul((0x4c7u32) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rbx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r11 as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((r11 as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x150u64), ((rdx as u32)) as u32); r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdx = ((((r10 as u32)) as i32).wrapping_mul((0x312au32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x150u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; rax = ((rcx.wrapping_add(rbp)) as u32) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            24 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 26; continue; }
            26 => { rbp = (((rbp as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_sub(0x1u64), ((rax as u8)) as u8); rbp = (((((rbp as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 31; continue; }
            29 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 30; continue; }
            30 => { rbp = (rbp & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 31; continue; }
            31 => { rax = ((r12.wrapping_add(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x6u64), ((rbp as u8)) as u8); rax = (((((rax as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 36; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 36; continue; }
            34 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 36; continue; }
            36 => { r12 = (((r12 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu8(r13, ((rax as u8)) as u8); r12 = (((((r12 as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 41; continue; }
            39 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 40; continue; }
            40 => { r12 = (r12 & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 41; continue; }
            41 => { rax = ((r14.wrapping_add(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x5u64), ((r12 as u8)) as u8); rax = (((((rax as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 42; continue; }
            42 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 46; continue; }
            44 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 46; continue; }
            46 => { r14 = (((r14 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x1u64), ((rax as u8)) as u8); r14 = (((((r14 as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 51; continue; } bb = 47; continue; }
            47 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 51; continue; }
            49 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 50; continue; }
            50 => { r14 = (r14 & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 51; continue; }
            51 => { rax = ((r15.wrapping_add(r9)) as u32) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x4u64), ((r14 as u8)) as u8); rax = (((((rax as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 56; continue; } bb = 52; continue; }
            52 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 56; continue; }
            54 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 56; continue; }
            56 => { r15 = (((r15 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; crate::simd_ops::wu8(r13.wrapping_add(0x2u64), ((rax as u8)) as u8); r15 = (((((r15 as u32)) as i32).wrapping_shr(17u32) as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0xff; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 61; continue; } bb = 57; continue; }
            57 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 59; continue; } bb = 58; continue; }
            58 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 61; continue; }
            59 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0xff; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 61; continue; } bb = 60; continue; }
            60 => { r15 = (r15 & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 61; continue; }
            61 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rcx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x158u64))) as i32 as i64) as u64) as u64; r11 = ((r11).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x3u64), ((r15 as u8)) as u8); r13 = ((r13).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), r11); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 62; continue; }
            62 => { rsp = ((rsp).wrapping_add(0x108u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142411020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426468a0; const P1: u64 = 0x142646890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142411060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426468e0; const P1: u64 = 0x1426468b0; const P2: u64 = 0x1426468a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = 0x1;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rdx = rbx; rcx = ((rax as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142412cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x14137e0f0; const P3: u64 = 0x18; const P4: u64 = 0x14137e200; const P5: u64 = 0x28;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142412d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1401de830; const P2: u64 = 0x18; const P3: u64 = 0x8;
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
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142414720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x74; const P10: u64 = 0x68; const P11: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rdx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rdx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rdx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142414950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b0e40; const P1: u64 = 0x8; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rcx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424149d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143851ba8; const P1: u64 = 0x143851bac; const P2: u64 = 0x1; const P3: u64 = 0x143851ba0; const P4: u64 = 0x14137f920; const P5: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    r8 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r9 = (crate::simd_ops::ru32(P1)) as u32 as u64;
    rcx = ((r9.wrapping_mul(4)) as u64) as u64;
    rax = ((r8.wrapping_mul(4)) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = ((r8.wrapping_add(P2)) as u64) as u64; rdx = ((r9 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; }
    rcx = ((r8 as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), (0x0u32) as u32);
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    rdx = ((rax as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(P3)) as u64;
    rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64;
    crate::simd_ops::wu32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142414db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x48; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x74; const P10: u64 = 0x68; const P11: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rcx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rcx);
    crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424151f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rbp = rcx; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142415290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = r8; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rbp = rcx; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142415330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x48; const P5: u64 = 0x40; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(0x38u64)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424153e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x48; const P5: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(0x38u64)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142415510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b4e40; const P1: u64 = 0x142415290; const P2: u64 = 0x1424151f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; r13 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0xb0u64), ((r13 as u8)) as u8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(0x110u64)))) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(r14.wrapping_add(0xb0u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x108u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x120u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), r13); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x108u64), ((r13 as u32)) as u32); rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x130u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x110u64))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x120u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = rdx; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x130u64), rdx); crate::simd_ops::wu64(r8.wrapping_add(0x120u64), rcx); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x130u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(0x110u64)))) as u64; fl_l = rax; fl_r = r8; r8 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r8 }) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xa0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x108u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x120u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), r13); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbp; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; r8 = (((rcx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rbp)) as u64; fl_l = rbp; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x38u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(r14.wrapping_add(0xd4u64), ((r13 as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xd4u64))) as u32 as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { bb = 8; continue; }
            15 => { r15 = rbp; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(r15)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { r15 = ((r15).wrapping_add(0x38u64)) as u64; fl_l = r15; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { r15 = r13; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbp)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rbp = ((rbp).wrapping_add(0x38u64)) as u64; fl_l = rbp; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 22; continue; }
            22 => { rbp = r13; bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xc0u64))) as u64; rbx = rdx; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xc8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 26; continue; }
            26 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rbx = rdx; rsi = rdx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rsi = ((rsi).wrapping_add(0x30u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 31; continue; }
            31 => { rsi = rdx; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp; rdx = r15; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xd0u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), r13); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x58u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r12 = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 39; continue; }
            39 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xd0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); r8 = rbp; rdx = r15; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0xd4u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xb8u64), ((r13 as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0xd0); rdx = rsp.wrapping_add(0x78); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 46; continue; }
            46 => { bb = 9; continue; }
            47 => { rsi = rdx; bb = 28; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142415d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x28; const P4: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P4), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rax as u8)) as u8);
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
    crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(0xb0u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0xb4u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xc8u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0xd0u64), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0xc8; const P2: u64 = 0xc0; const P3: u64 = 0x8; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), ((rax as u8)) as u8); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411daad0; const P1: u64 = 0x142415510;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rdi = r8; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x80u64))) as u32 as u64; rsi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x84u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x70u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x7cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x7cu64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x7cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi.wrapping_add(0x7cu64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x80u64), (1) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0xd0u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(r14)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0xd0u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xc8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xc0u64))) as u64; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rdi); rcx = rdi; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_add(0x38u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 24; continue; }
            24 => { rcx = rdi; bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0xd4u64), ((rax as u32)) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x7cu64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x7cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rax as u32)) as u32); bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            33 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rdi); bb = 21; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411daad0; const P1: u64 = 0x142415510; const P2: u64 = 0x1413953d0; const P3: u64 = 0x1401ed8b0; const P4: u64 = 0x14307b558; const P5: u64 = 0x1401ee280;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x390u64)) as u64; rsi = r9; rdi = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xd4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xb4u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xd4u64), ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rbx = rcx.wrapping_add(0x88); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), (0x0u32) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (1) as u32); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x9cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(rdi.wrapping_add(0x94u64), ((rdx as u32)) as u32); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rax as u32)) as u32); bb = 15; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0xa0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x3a0u64), (0x10000u64) as u64); r8 = rsp.wrapping_add(0x3a0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rax); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x3a0u64), (0x10000u64) as u64); r8 = rsp.wrapping_add(0x3a0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rax); bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rax.wrapping_add(0x108u64), (0x1u32) as u32); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x120u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x130u64), (0x0u64) as u64); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xa8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x120u64), (0x4u64) as u64); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x368u64), rax); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { r11 = rsp.wrapping_add(0x390); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424169d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a2af38; const P1: u64 = 0x140657cc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xd0u64))) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x38u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 22; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xd0u64), ((rcx as u32)) as u32); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rsi = ((rsi).wrapping_add(0x30u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xd0u64))) as u32 as u64; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rcx = ((rcx).wrapping_add(0x38u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 34; continue; }
            34 => { bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 39; continue; }
            39 => { bb = 41; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbp.wrapping_add(0xd0); rcx = rbp; rdx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 46; continue; }
            46 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142416500; const P1: u64 = 0x141397670; const P2: u64 = 0x143881ed8; const P3: u64 = 0x1424b14c0; const P4: u64 = 0x1424b0d90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); r14 = rbp.wrapping_add(0xc0); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rdi)) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r15); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x28u64), r15); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r15); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r15); rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), r15); crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(0xb0u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(0xb4u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142416f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142415290; const P1: u64 = 0x1424151f0; const P2: u64 = 0x1411daad0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = rdx; r14 = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0xd4u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rbp = rbx; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xd0u64))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbp)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbp = ((rbp).wrapping_add(0x38u64)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbp = r12; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rbx = r12; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xc0u64))) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xc8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 12; continue; }
            12 => { rsi = ((rsi).wrapping_add(0x30u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rsi = rdx; rdi = rdx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { rdi = rdx; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rbp; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xd0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); r8 = rbx; rdx = rbp; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14.wrapping_add(0x88); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r12); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 25; continue; }
            25 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (1) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xb8u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x9cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x94u64))) as u32 as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(r14.wrapping_add(0x94u64), ((rdx as u32)) as u32); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x98u64), ((rax as u32)) as u32); bb = 30; continue; }
            30 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0xd0); rdx = rsp.wrapping_add(0x70); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            34 => { rdi = rdx; bb = 14; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424171a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0xc8; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(4u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x142415510; const P2: u64 = 0xd0; const P3: u64 = 0x30; const P4: u64 = 0xc0; const P5: u64 = 0xc8; const P6: u64 = 0x8; const P7: u64 = 0xb8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(P7)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646400; const P1: u64 = 0x1423f0f80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = r9;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = 0x10000;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0)) { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (rax as u32 as i32 as i64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646408; const P1: u64 = 0x1423f0f80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = r9;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0)) { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (rax as u32 as i32 as i64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424176d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x142646428; const P2: u64 = 0x142646458; const P3: u64 = 0x1423f0f80; const P4: u64 = 0x1438795f0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); rbx = (r9) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24177a0u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = (0x8004667eu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            3 => { r8 = (0x1002u32) as u32 as u64; bb = 8; continue; }
            4 => { r8 = (0x1001u32) as u32 as u64; bb = 8; continue; }
            5 => { rdx = (0x6u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x5u64)) as u32) as u32 as u64; bb = 9; continue; }
            6 => { r8 = (0x4u32) as u32 as u64; bb = 8; continue; }
            7 => { r8 = (0x8u32) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (0xffffu32) as u32 as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = ((P4) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x81u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r9, (xmm0) as u128); bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424176d0u64) { 0x1e => { bb = 1; continue; } 0x35 => { bb = 2; continue; } 0x4a => { bb = 3; continue; } 0x52 => { bb = 4; continue; } 0x5a => { bb = 5; continue; } 0x65 => { bb = 6; continue; } 0x6d => { bb = 7; continue; } 0x73 => { bb = 8; continue; } 0x78 => { bb = 9; continue; } 0x8e => { bb = 10; continue; } 0x92 => { bb = 11; continue; } 0xaa => { bb = 12; continue; } 0xc9 => { bb = 13; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424177c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x142646428; const P2: u64 = 0x142646458; const P3: u64 = 0x1423f0f80; const P4: u64 = 0x1438795f0; const P5: u64 = 0x14139b920; const P6: u64 = 0x14139b9e0; const P7: u64 = 0x1424178ac;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1424177c0, funcinfo_rva: 0x37e0c88, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), (0x1u32) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((P0) as u64) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x24178b0u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { r8 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rdx = (0x8004667eu32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x142417804); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            3 => { r8 = (0x1002u32) as u32 as u64; bb = 8; continue; }
            4 => { r8 = (0x1001u32) as u32 as u64; bb = 8; continue; }
            5 => { rdx = (0x6u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x5u64)) as u32) as u32 as u64; bb = 9; continue; }
            6 => { r8 = (0x4u32) as u32 as u64; bb = 8; continue; }
            7 => { r8 = (0x8u32) as u32 as u64; bb = 8; continue; }
            8 => { rdx = (0xffffu32) as u32 as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r9 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14241784d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14241785c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; bb = 13; continue; }
            12 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x81u32) as u32); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 13; continue; }
            13 => { rax = ((xmm0 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14241789e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1424178a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1424177c0u64) { 0x1d => { bb = 1; continue; } 0x34 => { bb = 2; continue; } 0x4c => { bb = 3; continue; } 0x54 => { bb = 4; continue; } 0x5c => { bb = 5; continue; } 0x67 => { bb = 6; continue; } 0x6f => { bb = 7; continue; } 0x75 => { bb = 8; continue; } 0x7a => { bb = 9; continue; } 0x93 => { bb = 10; continue; } 0x97 => { bb = 11; continue; } 0xa7 => { bb = 12; continue; } 0xc1 => { bb = 13; continue; } 0xc9 => { bb = 14; continue; } 0xce => { bb = 15; continue; } _ => { drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438795f0; const P1: u64 = 0x1; const P2: u64 = 0x5; const P3: u64 = 0x142646440; const P4: u64 = 0x1423f0f80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0xffffffffffffffffu64) as u64;
    rbx = r8;
    rdi = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = P0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x71u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = 0x1;
    rcx = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rdi, rax);
    fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1424179f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x10; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x142417e60; const P5: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rdx; r11 = rcx; fl_l = r8; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = r9; fl_r = r10; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((r10 as u32))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; r8 = ((((r8 as u32)) & ((rax as u32)))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r10 = (if (((fl_l as u32)) < ((fl_r as u32))) { (r8 as u32) } else { (r10 as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = rcx; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r8 = ((r9.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0xffffffffffffffffu64) as u64; r9 = ((r8.wrapping_add(P2)) as u64) as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rcx = rax; bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { r8 = r11; rdx = rbx; rcx = r9; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            9 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            11 => { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x142645598; const P2: u64 = 0x142646538; const P3: u64 = 0x14139bb80; const P4: u64 = 0x14139bf60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; r9 = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 2; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rdx = ((((rdx as u32)) & ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rax = rsp.wrapping_add(0x20); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { rcx } else { rax }) as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = r8; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = rdx.wrapping_add(0x28); rcx = (0xffffffffffffffffu64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((r8).wrapping_add(rcx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { r8 = rax; bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { rbx = rsi.wrapping_add(0x8); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = ((rdi as u32)) as u32 as u64; r8 = 0x10000; rax = crate::simd_ops::call_stk(r9, rax, rcx, rdx, r8, r9, rsp); rbp = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = (crate::simd_ops::ru32(rsi)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r12 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r14, rbp); rdi = r14.wrapping_add(0x8); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbp); rcx = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu32(r14.wrapping_add(0x30u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x34u64), rbp); fl_l = (crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x18u64), (0x40u64) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            16 => { r9 = rsp.wrapping_add(0x70); r8 = rsp.wrapping_add(0x78); rdx = r14; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x14139bbf0; const P5: u64 = 0x142646558;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rbx = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = 0x10000; r8 = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rbx; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), r14); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143408f70; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x40; const P7: u64 = 0x14139bbf0; const P8: u64 = 0x142646558;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r15 = ((rsi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru32(r14.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = ((rax.wrapping_add(P6)) as u64) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = 0x10000; r8 = rdi; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = rbx; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), r12); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P5), rax); bb = 9; continue; }
            9 => { rsi = ((rsi).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142417e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x14139bd20; const P2: u64 = 0x14139bbf0; const P3: u64 = 0x142646558; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r13 = r8; rsi = rdx; r15 = rcx; r12 = 0; fl_l = 0; fl_r = 0; rdi = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(r15)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as i32) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { bb = 17; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x34u64), (1) as u32); rcx = rbx.wrapping_add(0x8); rdx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r12 = (crate::simd_ops::ru64(rbx)) as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r14 = rbx.wrapping_add(0x40); fl_l = r14; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 8; continue; }
            8 => { rbx = r14.wrapping_sub(0x38); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rax); rsi = rbx.wrapping_add(0x40); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rsi, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx, rbx); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rax, rsi); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); r14 = ((r14).wrapping_add(0x40u64)) as u64; rbx = rsi; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = rbx.wrapping_sub(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x38u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u64(r15.wrapping_add(0x18u64), (0xffffffffffffffc0u64) as u64)) as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; r9 = 0x10000; rdx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142418000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x142645598; const P2: u64 = 0x142646538; const P3: u64 = 0x14139bb80; const P4: u64 = 0x14139bf60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; r15 = rcx; rbx = rcx.wrapping_add(0x8); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = ((rdi as u32)) as u32 as u64; r8 = 0x10000; rax = crate::simd_ops::call_stk(r9, rax, rcx, rdx, r8, r9, rsp); rbp = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = (crate::simd_ops::ru32(r15)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r12 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi, rbp); rdi = rsi.wrapping_add(0x8); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbp); rcx = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu32(rsi.wrapping_add(0x30u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x34u64), rbp); fl_l = (crate::simd_ops::lk_add_u64(r15.wrapping_add(0x18u64), (0x40u64) as u64)) as u64; fl_r = fl_l; bb = 9; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 10; continue; }
            8 => { r9 = rsp.wrapping_add(0x60); r8 = rsp.wrapping_add(0x70); rdx = rsi; rcx = r15.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(0x40u64)) as u64; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_142418140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x14139bbf0; const P5: u64 = 0x142646558;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rsi = rcx; rbp = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rbx = ((rax.wrapping_add(P3)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = 0x10000; r8 = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rbx; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), r14); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
