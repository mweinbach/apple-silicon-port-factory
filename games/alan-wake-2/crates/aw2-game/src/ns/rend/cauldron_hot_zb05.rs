//! Decompiled `rend` hot functions shard.
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens, static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(never)]
pub unsafe extern "win64" fn ak_141512b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rbp) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x18u64)) as *mut u64, (rsi) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (core::ptr::read_unaligned((rcx) as *const u32)) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32))) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdi = (core::ptr::read_unaligned((rcx.wrapping_add(0x4u64)) as *const u32)) as u32 as u64; r11 = (rdx) as u64; rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; core::ptr::write_unaligned((rsp) as *mut u128, (xmm7) as u128); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm7 = (xmm3) as u128; r9 = (rcx) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r10 as u32)) as u32 as u64; bb = 19; continue; }
            2 => { xmm1 = ((core::ptr::read_unaligned((rcx.wrapping_add(0x1cu64)) as *const u32)) as u32) as u128; xmm5 = 0u128; core::ptr::write_unaligned((rsp.wrapping_add(0x30u64)) as *mut u64, (rbx) as u64); rbx = (((r10 as u16)) as u32) as u32 as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u128, (xmm6) as u128); xmm6 = ((core::ptr::read_unaligned((P0) as *const u32)) as u32) as u128; bb = 3; continue; }
            3 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm4 = ((core::ptr::read_unaligned((r9.wrapping_add(0x18u64)) as *const u32)) as u32) as u128; xmm3 = ((core::ptr::read_unaligned((r11) as *const u32)) as u32) as u128; bb = 7; continue; }
            5 => { rax = (((rsi as u16)) as u32) as u32 as u64; rdx = (((rcx as u16)) as u32) as u32 as u64; r8 = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 6; continue; }
            6 => { xmm3 = ((core::ptr::read_unaligned((r11.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64)) as *const u32)) as u32) as u128; rax = (((rcx as u16)) as u32) as u32 as u64; xmm4 = ((core::ptr::read_unaligned((r11.wrapping_add(rax.wrapping_mul(4))) as *const u32)) as u32) as u128; bb = 7; continue; }
            7 => { rdx = (((rbx as u16)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; xmm0 = 0u128; rcx = ((rcx as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            10 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { rax = ((rdi as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); rbx = (rbx & !0xffffu64) | (((((rbx as u16)).wrapping_add(1)) as u16 as u64)); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); core::ptr::write_unaligned((r14.wrapping_add(rax.wrapping_mul(4))) as *mut u32, ((xmm2 as u32)) as u32); rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (core::ptr::read_unaligned((r9) as *const u32)) as u32 as u64; rdi = ((r10 as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; bb = 3; continue; }
            12 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; core::ptr::write_unaligned((r9.wrapping_add(0x18u64)) as *mut u32, ((xmm4 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rax = (((rcx as u16)) as u32) as u32 as u64; r10 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 17; continue; }
            14 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (((rcx as u16)) as u32) as u32 as u64; rcx = (core::ptr::read_unaligned((r11.wrapping_add(rax.wrapping_mul(4))) as *const u32)) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; core::ptr::write_unaligned((r9.wrapping_add(0x18u64)) as *mut u32, ((rcx as u32)) as u32); bb = 16; continue; }
            16 => { r10 = (((rsi as u16)) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; bb = 17; continue; }
            17 => { xmm0 = (((rax as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 18; continue; }
            18 => { fl_l = ((core::ptr::read_unaligned((r9.wrapping_add(0xcu64)) as *const u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; core::ptr::write_unaligned((r9.wrapping_add(0xcu64)) as *mut u32, ((fl_l as u32)) as u32); xmm6 = (core::ptr::read_unaligned((rsp.wrapping_add(0x10u64)) as *const u128)) as u128; rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u64)) as u64; core::ptr::write_unaligned((r9.wrapping_add(0x1cu64)) as *mut u32, ((xmm1 as u32)) as u32); core::ptr::write_unaligned((r9.wrapping_add(0x4u64)) as *mut u32, ((rdi as u32)) as u32); rax = (((r10 as u16)) as u32) as u32 as u64; bb = 19; continue; }
            19 => { rbp = (core::ptr::read_unaligned((rsp.wrapping_add(0x38u64)) as *const u64)) as u64; rsi = (core::ptr::read_unaligned((rsp.wrapping_add(0x40u64)) as *const u64)) as u64; rdi = (core::ptr::read_unaligned((rsp.wrapping_add(0x48u64)) as *const u64)) as u64; xmm7 = (core::ptr::read_unaligned((rsp) as *const u128)) as u128; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn ak_14159a5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u64, (rbp) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x18u64)) as *mut u64, (rsi) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (core::ptr::read_unaligned((r8) as *const u32)) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = (r9) as u64; r14 = (r8) as u64; r12 = (rdx) as u64; rsi = (rcx) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r15 = ((rdi as u32)) as u32 as u64; rbp = (0xffffffffu32) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x40u64)) as *mut u64, (rbx) as u64); core::ptr::write_unaligned((r8) as *mut u32, ((rdi as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P0) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (core::ptr::read_unaligned((rsi.wrapping_add(0x8u64)) as *const u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 3; continue; }
            3 => { rcx = (core::ptr::read_unaligned((rsi) as *const u64)) as u64; rdx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (core::ptr::read_unaligned((rcx.wrapping_add(rax.wrapping_mul(8))) as *const u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 10; continue; }
            7 => { fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { rax = ((r9.wrapping_add(0x1fu64)) as u32) as u32 as u64; bb = 22; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 10; continue; }
            10 => { fl_l = (core::ptr::read_unaligned((rdx.wrapping_add(0x28u64)) as *const u64)) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rbp = (((rbp as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (core::ptr::read_unaligned((r14) as *const u32)) as u32 as u64; rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x4cu64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((r15.wrapping_add(rcx.wrapping_mul(4))) as *mut u32, ((rax as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((core::ptr::read_unaligned((r14) as *const u32)).wrapping_add(1)) as u32 as u64; core::ptr::write_unaligned((r14) as *mut u32, ((fl_l as u32)) as u32); bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = (core::ptr::read_unaligned((rdx.wrapping_add(0x68u64)) as *const u64)) as u64; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { r8 = (core::ptr::read_unaligned((rsi.wrapping_add(0x8u64)) as *const u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rax as u32)) as u32 as u64; rax = (core::ptr::read_unaligned((rsi) as *const u64)) as u64; rdx = (core::ptr::read_unaligned((rax.wrapping_add(rcx.wrapping_mul(8))) as *const u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 10; continue; }
            20 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 21; continue; }
            21 => { rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P1) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x40u64)) as *const u64)) as u64; rax = (0x1u32) as u32 as u64; bb = 22; continue; }
            22 => { rbp = (core::ptr::read_unaligned((rsp.wrapping_add(0x48u64)) as *const u64)) as u64; rsi = (core::ptr::read_unaligned((rsp.wrapping_add(0x50u64)) as *const u64)) as u64; rdi = (core::ptr::read_unaligned((rsp.wrapping_add(0x58u64)) as *const u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn ak_1415b16f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x143931880; const P2: u64 = 0x143931830; const P3: u64 = 0x1426452b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P0) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (core::ptr::read_unaligned((P1) as *const u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P3) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn ak_1415b1830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645790; const P1: u64 = 0x143931880; const P2: u64 = 0x143931830; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P0) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = (core::ptr::read_unaligned((P1) as *const u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = ((P2) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((P3) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn unknown_1415eee30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1415ef1b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = (core::ptr::read_unaligned((rdx.wrapping_add(P0)) as *const u32)) as u32 as u64;
    rdx = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn ak_14165fa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x60;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0; let mut xmm4: u128 = 0; let mut ymm4_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rbp = (((rbp) & (0xffffffffffffffe0u64))) as u64; r10 = (rcx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r9 as u32)) as u32 as u64; let bc_v: u128 = crate::simd_ops::vbroadcastss((xmm2 as u32)); let (y_lo, y_hi): (u128, u128) = (bc_v, bc_v); xmm4 = y_lo; ymm4_hi = y_hi; fl_l = ((((rcx as u32)) & (0xffffffe0u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(4)); xmm3 = core::ptr::read_unaligned(y_addr as *const u128); ymm3_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0); xmm2 = core::ptr::read_unaligned(y_addr as *const u128); ymm2_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vfmadd_ps(xmm4, xmm3, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4))) as *const u128), false, false), crate::simd_ops::vfmadd_ps(ymm4_hi, ymm3_hi, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4))).wrapping_add(16) as *const u128), false, false)); xmm3 = y_lo; ymm3_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vfmadd_ps(xmm4, xmm2, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0)) as *const u128), false, false), crate::simd_ops::vfmadd_ps(ymm4_hi, ymm2_hi, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0)).wrapping_add(16) as *const u128), false, false)); xmm2 = y_lo; ymm2_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vfmadd_ps(xmm4, xmm1, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1)) as *const u128), false, false), crate::simd_ops::vfmadd_ps(ymm4_hi, ymm1_hi, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1)).wrapping_add(16) as *const u128), false, false)); xmm1 = y_lo; ymm1_hi = y_hi; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vfmadd_ps(xmm4, xmm0, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2)) as *const u128), false, false), crate::simd_ops::vfmadd_ps(ymm4_hi, ymm0_hi, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2)).wrapping_add(16) as *const u128), false, false)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = r10.wrapping_add(rax.wrapping_mul(4)); core::ptr::write_unaligned(y_addr as *mut u128, xmm3); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm3_hi); let y_addr: u64 = r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0); core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); let y_addr: u64 = r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); let y_addr: u64 = r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); rax = (((rax as u32)).wrapping_add(0x20u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(4)); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vfmadd_ps(xmm4, xmm0, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4))) as *const u128), false, false), crate::simd_ops::vfmadd_ps(ymm4_hi, ymm0_hi, core::ptr::read_unaligned((r10.wrapping_add(rax.wrapping_mul(4))).wrapping_add(16) as *const u128), false, false)); xmm0 = y_lo; ymm0_hi = y_hi; let y_addr: u64 = r10.wrapping_add(rax.wrapping_mul(4)); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); rax = (((rax as u32)).wrapping_add(0x8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

pub unsafe fn diff_ak_141512b30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64;
    let f_n: F = core::mem::transmute(new);

    let run = |is_orig: bool, step: f32| -> (u64, [u32; 8], u32, u32, u32) {
        let mut ring = [0.0f32; 16];
        let mut st = [0u32; 8];
        st[0] = 16;
        st[1] = 0;
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>((st.as_mut_ptr() as usize) + 0x10, rp);
        st[6] = 0.25f32.to_bits();
        st[7] = 0.5f32.to_bits();
        let inp: [f32; 8] = [0.1, 0.2, 0.4, 0.8, -0.5, 0.25, 0.75, 1.0];
        let gpr = [
            st.as_mut_ptr() as u64,
            inp.as_ptr() as u64,
            4,
            step.to_bits() as u64,
            0, 0, 0, 0,
        ];
        let xmm = [0u128, 0u128, 0u128, step.to_bits() as u128];
        let ret = if is_orig {
            crate::emu::emu_run_full((orig & !(1usize << 46)) as u64, gpr, xmm, [0; 8]).0
        } else {
            crate::simd_ops::set_tls_xmm_in(xmm);
            f_n(gpr[0], gpr[1], gpr[2], gpr[3], 0, 0, 0, 0)
        };
        let mut out_r = [0u32; 8];
        for i in 0..8 {
            out_r[i] = ring[i].to_bits();
        }
        (ret, out_r, st[1], st[3], st[7])
    };

    for step in [0.5f32, 0.75f32] {
        let o = run(true, step);
        let n = run(false, step);
        crate::diff_eq!(o, n, "141512b30 step={}", step);
    }
    Ok(2)
}

pub unsafe fn diff_ak_14159a5d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, key: u64| -> (u32, u32, u32) {
        let mut node = [0u8; 0x80];
        let np = node.as_mut_ptr() as usize;
        wr::<u64>(np + 0x28, 0x1234);
        wr::<u32>(np + 0x4c, 0x99);

        let buckets: [usize; 2] = [0, np];
        let mut tbl = [0u8; 0x60];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<usize>(tp, buckets.as_ptr() as usize);
        wr::<u32>(tp + 0x08, 2);
        wr::<i32>(tp + 0x20, -1);

        let mut count: u32 = 4;
        let mut out_val: u32 = 0;
        let ret = f(tp, key, (&mut count as *mut u32) as usize, (&mut out_val as *mut u32) as usize);
        (ret, count, out_val)
    };

    for key in [0x1234u64, 0x5678u64] {
        let o = run(f_o, key);
        let n = run(f_n, key);
        crate::diff_eq!(o, n, "14159a5d0 key={}", key);
    }
    Ok(2)
}

pub unsafe fn diff_ak_1415b16f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_tid = exe::ptr::<u32>(0x1_4393_1880) as usize;
    let saved = rd::<u32>(g_tid);
    let cur_tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    wr::<u32>(g_tid, cur_tid);

    for v in [0x1111u64, 0x2222u64] {
        let o = f_o(v);
        let n = f_n(v);
        crate::diff_eq!(o, n, "1415b16f0 v={}", v);
    }
    wr::<u32>(g_tid, saved);
    Ok(2)
}

pub unsafe fn diff_ak_1415b1830(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_tid = exe::ptr::<u32>(0x1_4393_1880) as usize;
    let saved = rd::<u32>(g_tid);
    let cur_tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    wr::<u32>(g_tid, cur_tid);

    for _ in 0..2 {
        f_o();
        f_n();
    }
    wr::<u32>(g_tid, saved);
    Ok(2)
}

pub unsafe fn diff_unknown_1415eee30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GOT_F1B0: (u64, u64, u64, u32) = (0, 0, 0, 0);
    unsafe extern "win64" fn mock_f1b0(a1: u64, a2: u64, a3: u64, a4: u32) {
        GOT_F1B0 = (a1, a2, a3, a4);
    }
    let _m1 = crate::difftest::Mock::new(0x1_415e_f1b0, mock_f1b0 as *const ())?;

    let run = |f: F, v: u32| -> (u64, u64, u64, u32) {
        GOT_F1B0 = (0, 0, 0, 0);
        let p2: [u64; 3] = [0xaaaa, 0, v as u64];
        f(0x1111, p2.as_ptr() as usize, 0x2222);
        GOT_F1B0
    };

    for v in [7u32, 42u32] {
        let o = run(f_o, v);
        let n = run(f_n, v);
        crate::diff_eq!(o, n, "1415eee30 v={}", v);
    }
    Ok(2)
}

pub unsafe fn diff_ak_14165fa50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let f_n: F = core::mem::transmute(new);

    let run = |is_orig: bool, scale: f32| -> [u32; 16] {
        let mut dst = [0.25f32; 64];
        let mut src = [0.5f32; 64];
        for i in 0..64 {
            dst[i] = (i as f32) * 0.125;
            src[i] = ((64 - i) as f32) * 0.0625;
        }
        let gpr = [
            dst.as_mut_ptr() as u64,
            src.as_ptr() as u64,
            scale.to_bits() as u64,
            64,
            0, 0, 0, 0,
        ];
        let xmm = [0u128, 0u128, scale.to_bits() as u128, 0u128];
        if is_orig {
            let _ = crate::emu::emu_run_full((orig & !(1usize << 46)) as u64, gpr, xmm, [0; 8]);
        } else {
            crate::simd_ops::set_tls_xmm_in(xmm);
            let _ = f_n(gpr[0], gpr[1], gpr[2], gpr[3]);
        }
        let mut out = [0u32; 16];
        for i in 0..16 {
            out[i] = dst[i].to_bits();
        }
        out
    };

    for scale in [0.5f32, -1.5f32] {
        let o = run(true, scale);
        let n = run(false, scale);
        crate::diff_eq!(o, n, "14165fa50 scale={}", scale);
    }
    Ok(2)
}
