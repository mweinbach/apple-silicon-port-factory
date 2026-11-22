//! Decompiled `rend` hot functions shard.
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens, static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(never)]
pub unsafe extern "win64" fn ak_1417c0690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417bfbc0; const P1: u64 = 0x14397a880; const P2: u64 = 0x14397a830; const P3: u64 = 0x14397a7f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; r13 = (core::ptr::read_unaligned((rdx.wrapping_add(0x18u64)) as *const u64)) as u64; r12 = (r8) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rsi = (r9) as u64; r10 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = (core::ptr::read_unaligned((rsp.wrapping_add(0x158u64)) as *const u32)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rdi as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((rdx as u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x4u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(2u32)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = (core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4))) as *const u128)) as u128; core::ptr::write_unaligned((rsp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xb0u64)) as *mut u128, (xmm0) as u128); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (core::ptr::read_unaligned((rsp.wrapping_add(0x150u64)) as *const u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { xmm5 = (core::ptr::read_unaligned((P1) as *const u128)) as u128; r10 = (rdi) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x130u64)) as *mut u64, (rbx) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x138u64)) as *mut u64, (rbp) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x140u64)) as *mut u64, (r15) as u64); r15 = (core::ptr::read_unaligned((rsp.wrapping_add(0x160u64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((rsp.wrapping_add(0xf0u64)) as *mut u128, (xmm6) as u128); xmm6 = (core::ptr::read_unaligned((P2) as *const u128)) as u128; core::ptr::write_unaligned((rsp.wrapping_add(0xe0u64)) as *mut u128, (xmm7) as u128); xmm7 = (core::ptr::read_unaligned((P3) as *const u128)) as u128; bb = 5; continue; }
            5 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rbp = ((rdi as u32)) as u32 as u64; r8 = (0x1u32) as u32 as u64; rbp = ((((rbp as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; xmm4 = (core::ptr::read_unaligned((r12.wrapping_add(rax.wrapping_mul(4))) as *const u128)) as u128; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; xmm1 = (xmm4) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm7); xmm3 = (xmm4) as u128; rcx = ((rax as u32)) as u32 as u64; rax = (r9) as u64; xmm3 = crate::simd_ops::vcmpps(xmm3, xmm5, 0x1u8); rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (xmm3) as u128; xmm0 = (!(xmm0)) & (xmm4); xmm1 = (xmm1) & (xmm3); xmm1 = (xmm1) | (xmm0); xmm2 = (core::ptr::read_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0xb0u64)) as *const u128)) as u128; xmm0 = (xmm2) as u128; xmm0 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x1u8); xmm2 = (xmm2) & (xmm0); xmm0 = (!(xmm0)) & (xmm1); xmm2 = (xmm2) | (xmm0); xmm1 = (xmm7) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm2); xmm1 = (xmm1) & (xmm3); xmm3 = (!(xmm3)) & (xmm2); xmm1 = (xmm1) | (xmm3); xmm2 = crate::simd_ops::vrcpps(xmm4); xmm2 = crate::simd_ops::vmulps(xmm2, xmm1); core::ptr::write_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64)) as *mut u128, (xmm2) as u128); fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (core::ptr::read_unaligned((rsp.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x50u64)) as *const u64)) as u64; fl_l = ((r9).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; xmm4 = crate::simd_ops::vcmpps(xmm4, xmm5, 0x0u8); xmm0 = (xmm4) as u128; xmm4 = (!(xmm4)) & (xmm2); xmm0 = (xmm0) & (xmm6); xmm0 = (xmm0) | (xmm4); core::ptr::write_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64)) as *mut u128, (xmm0) as u128); rax = (core::ptr::read_unaligned((rsp.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x20u64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((r10.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32); rax = ((r8 as u32)) as u32 as u64; rcx = (core::ptr::read_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64)) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rsp.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x24u64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((r10.wrapping_add(rcx)) as *mut u32, ((rax as u32)) as u32); rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = (core::ptr::read_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64)) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rsp.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x28u64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((r10.wrapping_add(rdx)) as *mut u32, ((rax as u32)) as u32); rax = ((r8.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = (core::ptr::read_unaligned((rsp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x50u64)) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rsp.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x2cu64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((r10.wrapping_add(rdx)) as *mut u32, ((rax as u32)) as u32); bb = 9; continue; }
            9 => { r8 = (((r8 as u32)).wrapping_add(0x4u32)) as u32 as u64; core::ptr::write_unaligned((rsi.wrapping_add(rbx.wrapping_mul(4))) as *mut u128, (xmm1) as u128); rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r10 = ((r10).wrapping_add(0x4u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 11; continue; }
            11 => { xmm7 = (core::ptr::read_unaligned((rsp.wrapping_add(0xe0u64)) as *const u128)) as u128; xmm6 = (core::ptr::read_unaligned((rsp.wrapping_add(0xf0u64)) as *const u128)) as u128; r15 = (core::ptr::read_unaligned((rsp.wrapping_add(0x140u64)) as *const u64)) as u64; rbp = (core::ptr::read_unaligned((rsp.wrapping_add(0x138u64)) as *const u64)) as u64; rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x130u64)) as *const u64)) as u64; bb = 12; continue; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x100u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn bigfish_141a9a450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x8; const P4: u64 = 0x580e0; const P5: u64 = 0x143c8ceec; const P6: u64 = 0x14020d3b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (core::ptr::read_unaligned((rcx.wrapping_add(P0)) as *const u64)) as u64;
    rdi = (core::ptr::read_unaligned((rcx.wrapping_add(P1)) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    r8 = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64;
    xmm0 = ((core::ptr::read_unaligned((rdi.wrapping_add(rbx.wrapping_mul(4))) as *const u32)) as u32) as u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (core::ptr::read_unaligned((rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2)) as *const u32)) as u32 as u64;
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { rax = (core::ptr::read_unaligned((rcx.wrapping_add(P3)) as *const u64)) as u64; r8 = (0x2eu32) as u32 as u64; rcx = (core::ptr::read_unaligned((rdx.wrapping_add(P4)) as *const u64)) as u64; r9 = ((core::ptr::read_unaligned((P5) as *const u16)) as u32) as u32 as u64; rdx = (core::ptr::read_unaligned((rax.wrapping_add(rbx.wrapping_mul(8))) as *const u64)) as u64; rcx = (core::ptr::read_unaligned((rcx.wrapping_add(r9.wrapping_mul(8))) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; }
    core::ptr::write_unaligned((rdi.wrapping_add(rbx.wrapping_mul(4))) as *mut u32, (0x0u32) as u32);
    rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn coreshared_142039910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c16200; const P1: u64 = 0x14220ea70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (core::ptr::read_unaligned((rdx) as *const u64)) as u64; rbx = (rcx) as u64; r9 = (core::ptr::read_unaligned((r8) as *const u64)) as u64; rcx = (core::ptr::read_unaligned((rax.wrapping_add(0xc0u64)) as *const u64)) as u64; rdx = (rcx) as u64; rax = (core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u64)) as u64; fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x19u64)) as *const u8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x20u64)) as *const u64)) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u64)) as u64; bb = 4; continue; }
            3 => { rdx = (rax) as u64; rax = (core::ptr::read_unaligned((rax) as *const u64)) as u64; bb = 4; continue; }
            4 => { fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x19u64)) as *const u8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (core::ptr::read_unaligned((rdx.wrapping_add(0x19u64)) as *const u8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (r9) as u64 as u64; fl_r = (core::ptr::read_unaligned((rdx.wrapping_add(0x20u64)) as *const u64)) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(0x28u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((P0) as u64) as u64; bb = 9; continue; }
            9 => { fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x4u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { core::ptr::write_unaligned((rbx) as *mut u32, (0x0u32) as u32); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = ((core::ptr::read_unaligned((rax.wrapping_add(0x3u64)) as *const u8)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(0x8u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 14; continue; }
            13 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (core::ptr::read_unaligned((rax) as *const u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (core::ptr::read_unaligned((rdx) as *const u64)) as u64; bb = 16; continue; }
            16 => { core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rdx) as u64); rax = (0xffffffffffffffffu64) as u64; bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (core::ptr::read_unaligned((rdx.wrapping_add(rax)) as *const u8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 18; continue; }
            18 => { core::ptr::write_unaligned((rsp.wrapping_add(0x28u64)) as *mut u64, (rax) as u64); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; xmm0 = (core::ptr::read_unaligned((rsp.wrapping_add(0x20u64)) as *const u128)) as u128; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u128, (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (core::ptr::read_unaligned((rax) as *const u32)) as u32 as u64; rax = (rbx) as u64; core::ptr::write_unaligned((rbx) as *mut u32, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn coreshared_1420472c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rbx) as u64); r11 = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((core::ptr::read_unaligned((rdx) as *const u16)) as u32) as u32 as u64; rax = (core::ptr::read_unaligned((r11) as *const u64)) as u64; r10 = ((core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0)) as *const u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = ((core::ptr::read_unaligned((rbx.wrapping_add(P1)) as *const u16)) as u32) as u32 as u64; rax = (core::ptr::read_unaligned((r11.wrapping_add(0x20u64)) as *const u64)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            2 => { fl_l = (core::ptr::read_unaligned((rax) as *const u64)) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = ((r9).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64; return rax; }
            5 => { rcx = (core::ptr::read_unaligned((rbx) as *const u32)) as u32 as u64; rax = ((core::ptr::read_unaligned((rdx.wrapping_add(P2)) as *const u16)) as u32) as u32 as u64; rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x8u64)) as *const u64)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (core::ptr::read_unaligned((r11.wrapping_add(0x30u64)) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8))) as *const u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn luau_1420ec940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1420fd0a0; const P2: u64 = 0x1420fcb60; const P3: u64 = 0x1420ecb90; const P4: u64 = 0x14210e4e0; const P5: u64 = 0x1433dfa08; const P6: u64 = 0x14210e640;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { core::ptr::write_unaligned((rsp.wrapping_add(0x18u64)) as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x60u64)) as *mut u64, (rbx) as u64); r13 = ((P0) as u64) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x68u64)) as *mut u64, (rsi) as u64); r12 = (r9) as u64; r15 = (r8) as u64; rdi = (rdx) as u64; rbp = (rcx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rax = (((core::ptr::read_unaligned((rdi.wrapping_add(0x10u64)) as *const u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rsi = (core::ptr::read_unaligned((rdi) as *const u64)) as u64; rdx = (r15) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rax) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(core::ptr::read_unaligned((rsi.wrapping_add(0x20u64)) as *const u64))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(63u32)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; core::ptr::write_unaligned((rbp.wrapping_add(0x54u64)) as *mut u32, ((rdx as u32)) as u32); bb = 4; continue; }
            4 => { fl_l = (core::ptr::read_unaligned((rbx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 5; continue; }
            5 => { r11 = (core::ptr::read_unaligned((rsi.wrapping_add(0x10u64)) as *const u64)) as u64; fl_l = (r11) as u64 as u64; fl_r = (r11) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 6; continue; }
            6 => { rsi = ((core::ptr::read_unaligned((r11.wrapping_add(0x3u64)) as *const u8)) as u32) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 7; continue; }
            7 => { rdx = (core::ptr::read_unaligned((rbp.wrapping_add(0x18u64)) as *const u64)) as u64; rcx = (r11) as u64; rdx = (core::ptr::read_unaligned((rdx.wrapping_add(0xb40u64)) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 8; continue; }
            8 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { bb = 17; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x8u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (rax) as u64; rax = (core::ptr::read_unaligned((rbp.wrapping_add(0x18u64)) as *const u64)) as u64; rcx = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0xaa0u64)) as *const u64)) as u64; bb = 13; continue; }
            12 => { rax = (core::ptr::read_unaligned((rdi) as *const u64)) as u64; rcx = (core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u64)) as u64; bb = 13; continue; }
            13 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (core::ptr::read_unaligned((rbp.wrapping_add(0x18u64)) as *const u64)) as u64; rdx = (core::ptr::read_unaligned((rdx.wrapping_add(0xb40u64)) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            15 => { rax = (r13) as u64; bb = 16; continue; }
            16 => { fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 17; continue; }
            17 => { fl_l = (core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 18; continue; }
            18 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rdi = (rax) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (0x64u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 19; continue; }
            19 => { bb = 1; continue; }
            20 => { fl_l = ((((rsi as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l; rsi = (rsi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); core::ptr::write_unaligned((r11.wrapping_add(0x3u64)) as *mut u8, ((rsi as u8)) as u8); bb = 21; continue; }
            21 => { xmm0 = (core::ptr::read_unaligned((rbx) as *const u128)) as u128; core::ptr::write_unaligned((r12) as *mut u128, (xmm0) as u128); xmm1 = ((core::ptr::read_unaligned((rbx.wrapping_add(0x10u64)) as *const u64)) as u64) as u128; core::ptr::write_unaligned((r12.wrapping_add(0x10u64)) as *mut u64, ((xmm1 as u64)) as u64); bb = 22; continue; }
            22 => { rsi = (core::ptr::read_unaligned((rsp.wrapping_add(0x68u64)) as *const u64)) as u64; rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x60u64)) as *const u64)) as u64; rbp = (core::ptr::read_unaligned((rsp.wrapping_add(0x70u64)) as *const u64)) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { r9 = (rdi) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (r15) as u64); r8 = (rax) as u64; rdx = (r12) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            24 => { r8 = (r15) as u64; rdx = (rdi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rdx = ((P5) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn luau_1420edd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14210d8d0; const P1: u64 = 0x1420ec940;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { core::ptr::write_unaligned((rsp.wrapping_add(0x8u64)) as *mut u64, (rbx) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rbp) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x18u64)) as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rcx) as u64; rdi = ((r9 as u32)) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; r14 = (r8) as u64; rcx = (((rcx as u32)).wrapping_shr(30u32)) as u32 as u64; r15 = (rdx) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = ((r9 as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(10u32)) as u32 as u64; rsi = ((((rsi as u32)) & (0x3ffu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((((rdi as u32)) & (0x3ffu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            4 => { rbp = (0xffffffffu32) as u32 as u64; bb = 5; continue; }
            5 => { rsi = (0xffffffffu32) as u32 as u64; bb = 6; continue; }
            6 => { rdi = (0xffffffffu32) as u32 as u64; bb = 7; continue; }
            7 => { rax = (core::ptr::read_unaligned((rbx.wrapping_add(0x28u64)) as *const u64)) as u64; rax = ((rax).wrapping_sub(core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x18u64) as u64 as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = (core::ptr::read_unaligned((rbx.wrapping_add(0x48u64)) as *const u32)) as u32 as u64; rcx = (rbx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            10 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rax = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (r15) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x30u64)) as *mut u32, (0x6u32) as u32); core::ptr::write_unaligned((rax.wrapping_add(0x10u64)) as *mut u32, (0x0u32) as u32); r9 = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; rax = ((r9.wrapping_add(0x18u64)) as u64) as u64; core::ptr::write_unaligned((rbx.wrapping_add(0x8u64)) as *mut u64, (rax) as u64); rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = ((core::ptr::read_unaligned((rsp.wrapping_add(0x80u64)) as *const u8)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; fl_l = (core::ptr::read_unaligned((rax.wrapping_sub(0x8u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0xffffffffffffffe8u64)) as u64; r9 = (rdx) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; fl_l = (core::ptr::read_unaligned((rax.wrapping_sub(0x8u64)) as *const u32)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u64)) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0xffffffffffffffe8u64)) as u64; r9 = (rdx) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x60u64)) as *const u64)) as u64; rbp = (core::ptr::read_unaligned((rsp.wrapping_add(0x68u64)) as *const u64)) as u64; rsi = (core::ptr::read_unaligned((rsp.wrapping_add(0x70u64)) as *const u64)) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

pub unsafe fn diff_ak_1417c0690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u32, u32, i32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_fbc0(_p1: usize, _p2: usize, out: usize) {
        for i in 0..12 {
            wr::<u64>(out + i * 8, 0);
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_417b_fbc0, mock_fbc0 as *const ())?;

    let run = |f: F, n_frames: u32| -> (u64, [u32; 8]) {
        let st: [f32; 16] = [0.5; 16];
        let p2: [u64; 4] = [0; 4];
        let inp: [f32; 16] = [0.25, -0.5, 0.75, 0.125, -0.25, 0.5, -0.75, 0.375, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let mut out = [0.0f32; 16];
        let ret = f(
            st.as_ptr() as usize,
            p2.as_ptr() as usize,
            inp.as_ptr() as usize,
            out.as_mut_ptr() as usize,
            n_frames,
            4,
            4,
        );
        let mut res = [0u32; 8];
        for i in 0..8 {
            res[i] = out[i].to_bits();
        }
        (ret, res)
    };

    for n_frames in [1u32, 2u32] {
        let o = run(f_o, n_frames);
        let n = run(f_n, n_frames);
        crate::diff_eq!(o, n, "1417c0690 n_frames={}", n_frames);
    }
    Ok(2)
}

pub unsafe fn diff_bigfish_141a9a450(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_D3B0: u32 = 0;
    unsafe extern "win64" fn mock_d3b0(_a: u64, _b: u64, _c: u32) {
        CALLED_D3B0 += 1;
    }
    let _m1 = crate::difftest::Mock::new(0x1_4020_d3b0, mock_d3b0 as *const ())?;

    let run = |f: F, cur_val: f32| -> (u32, u32) {
        CALLED_D3B0 = 0;
        let mut b0 = [0u8; 0x40];
        wr::<f32>(b0.as_mut_ptr() as usize + 0x14, 1.0);
        let b1: [u64; 4] = [0x1234, 0, 0, 0];
        let mut b2: [f32; 4] = [cur_val, 0.0, 0.0, 0.0];
        let p1: [u64; 6] = [
            b0.as_ptr() as u64,
            b1.as_ptr() as u64,
            b2.as_mut_ptr() as u64,
            0,
            0,
            0,
        ];
        let mut p2 = vec![0u8; 0x58100];
        let arr: [u64; 4] = [0x9999; 4];
        wr::<usize>(p2.as_mut_ptr() as usize + 0x580e0, arr.as_ptr() as usize);
        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (CALLED_D3B0, b2[0].to_bits())
    };

    for cur_val in [0.5f32, 1.5f32] {
        let o = run(f_o, cur_val);
        let n = run(f_n, cur_val);
        crate::diff_eq!(o, n, "141a9a450 cur_val={}", cur_val);
    }
    Ok(2)
}

pub unsafe fn diff_coreshared_142039910(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, search_key: u64| -> (usize, u32) {
        let mut sentinel = [0u8; 0x40];
        let sp = sentinel.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x00, sp);
        wr::<usize>(sp + 0x08, sp);
        wr::<usize>(sp + 0x10, sp);
        wr::<u8>(sp + 0x19, 1);

        let mut mgr = [0u8; 0xd0];
        wr::<usize>(mgr.as_mut_ptr() as usize + 0xc0, sp);
        let p2: usize = mgr.as_ptr() as usize;
        let mut out_u32: u32 = 0xdead_beef;
        let out_p = (&mut out_u32 as *mut u32) as usize;
        let ret = f(out_p, (&p2 as *const usize) as usize, (&search_key as *const u64) as usize);
        (ret.wrapping_sub(out_p), out_u32)
    };

    for k in [10u64, 20u64] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "142039910 k={}", k);
    }
    Ok(2)
}

pub unsafe fn diff_coreshared_1420472c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, target_k: u64| -> u64 {
        let hdr: [u16; 4] = [0, 0, 0, 2]; // base=0, key_off=0, count=2
        let keys: [u64; 2] = [100, 200];
        let vals: [u64; 4] = [0xaaaa, 0xbbbb, 0xcccc, 0xdddd];
        let mut inner = [0u64; 8];
        inner[0] = hdr.as_ptr() as u64;
        inner[4] = keys.as_ptr() as u64;
        inner[6] = vals.as_ptr() as u64;
        let outer: usize = inner.as_ptr() as usize;
        let p2: [u16; 2] = [0, 1];
        f((&outer as *const usize) as usize, p2.as_ptr() as usize, target_k)
    };

    for target_k in [200u64, 999u64] {
        let o = run(f_o, target_k);
        let n = run(f_n, target_k);
        crate::diff_eq!(o, n, "1420472c0 k={}", target_k);
    }
    Ok(2)
}

pub unsafe fn diff_luau_1420ec940(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_SLOT: [u64; 3] = [0; 3];
    unsafe extern "win64" fn mock_d0a0(_tbl: usize, key: u64) -> usize {
        FOUND_SLOT = [key + 10, 0, 3];
        core::ptr::addr_of_mut!(FOUND_SLOT) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_420f_d0a0, mock_d0a0 as *const ())?;

    let run = |f: F, key: u64| -> [u64; 3] {
        let mut l_st = [0u8; 0x80];
        let mut tbl = [0u8; 0x40];
        let p2: [u64; 3] = [tbl.as_mut_ptr() as u64, 0, 6];
        let mut out_tv = [0u64; 3];
        let _ = f(
            l_st.as_mut_ptr() as usize,
            p2.as_ptr() as usize,
            key,
            out_tv.as_mut_ptr() as usize,
        );
        out_tv
    };

    for key in [42u64, 99u64] {
        let o = run(f_o, key);
        let n = run(f_n, key);
        crate::diff_eq!(o, n, "1420ec940 key={}", key);
    }
    Ok(2)
}

pub unsafe fn diff_luau_1420edd00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_CNT_C940: u32 = 0;
    unsafe extern "win64" fn mock_c940(_l: usize, _t: usize, _k: usize, out: usize) -> usize {
        CALL_CNT_C940 += 1;
        wr::<u64>(out, 0x7777);
        wr::<u32>(out + 0x10, 6);
        out
    }
    let _m1 = crate::difftest::Mock::new(0x1_420e_c940, mock_c940 as *const ())?;

    let run = |f: F, n_parts: u64| -> (u32, usize, u64, u32) {
        CALL_CNT_C940 = 0;
        let mut stk = [0u8; 0x100];
        let sp = stk.as_mut_ptr() as usize;
        let mut l_st = [0u8; 0x60];
        let lp = l_st.as_mut_ptr() as usize;
        wr::<usize>(lp + 0x08, sp);
        wr::<usize>(lp + 0x28, sp + 0x80);
        let k_arr = [0u8; 0x80];
        let imp = (n_parts << 30) | (1 << 20) | (2 << 10) | 3;
        f(lp, 0x1234, k_arr.as_ptr() as usize, imp, 0);
        (
            CALL_CNT_C940,
            rd::<usize>(lp + 0x08).wrapping_sub(sp),
            rd::<u64>(sp),
            rd::<u32>(sp + 0x10),
        )
    };

    for n_parts in [1u64, 2u64] {
        let o = run(f_o, n_parts);
        let n = run(f_n, n_parts);
        crate::diff_eq!(o, n, "1420edd00 n_parts={}", n_parts);
    }
    Ok(2)
}
