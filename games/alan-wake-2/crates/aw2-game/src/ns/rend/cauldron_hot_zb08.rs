//! Decompiled `rend` hot functions shard.
#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens, static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(never)]
pub unsafe extern "win64" fn luau_142103170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (rdx) as u64; r10 = (rcx) as u64; rbx = ((r11 as u32)) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(0xffffffffffffffecu64)) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; rax = (((rdx) as i64).wrapping_mul((0xfffffffffffffff4u64) as i64) as u64) as u64; fl_l = ((r9).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_add(core::ptr::read_unaligned((r10.wrapping_add(P0)) as *const u32))) as u32 as u64; xmm0 = ((core::ptr::read_unaligned((r10) as *const u64)) as u64) as u128; rcx = ((r8 as u32)) as u32 as u64; r10 = ((r10).wrapping_add(0xcu64)) as u64; rcx = (((rcx as u32)).rotate_right(14u32)) as u32 as u64; core::ptr::write_unaligned((rsp) as *mut u64, ((xmm0 as u64)) as u64); r11 = (((r11 as u32)).wrapping_add(core::ptr::read_unaligned((rsp) as *const u32))) as u32 as u64; rax = (core::ptr::read_unaligned((rsp) as *const u64)) as u64; r11 = ((((r11 as u32)) ^ ((r8 as u32)))) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; rcx = (((rcx as u32)).rotate_right(11u32)) as u32 as u64; rbx = ((((rbx as u32)) ^ ((r11 as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rbx as u32)))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(25u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rax as u32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rcx = ((core::ptr::read_unaligned((r10.wrapping_add(r9).wrapping_add(P1)) as *const u8)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn unknown_14213bb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0x90; const P2: u64 = 0x48; const P3: u64 = 0x50; const P4: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (core::ptr::read_unaligned((rcx.wrapping_add(P0)) as *const u64)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rbx = (rcx) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { core::ptr::write_unaligned((rcx.wrapping_add(P1)) as *mut u8, ((rdx as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = (core::ptr::read_unaligned((rcx.wrapping_add(P1)) as *const u8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((rax.wrapping_add(P2)) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (core::ptr::read_unaligned((rbx.wrapping_add(P0)) as *const u64)) as u64; rcx = (rax) as u64; r8 = (core::ptr::read_unaligned((rax) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((r8.wrapping_add(P3)) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); core::ptr::write_unaligned((rbx.wrapping_add(P1)) as *mut u8, (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((rax.wrapping_add(P2)) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (core::ptr::read_unaligned((rbx.wrapping_add(P0)) as *const u64)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rax) as u64; r9 = (core::ptr::read_unaligned((rax) as *const u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((r9.wrapping_add(P4)) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); core::ptr::write_unaligned((rbx.wrapping_add(P1)) as *mut u8, (0x0u8) as u8); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn unknown_1421be190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434281d0; const P1: u64 = 0x20; const P2: u64 = 0x2; const P3: u64 = 0xffffffffffffffe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r11 = (rdx) as u64;
    r10 = (rcx) as u64;
    xmm0 = (core::ptr::read_unaligned((P0) as *const u128)) as u128; ymm0_hi = 0;
    xmm1 = 0u128;
    core::ptr::write_unaligned((rsp) as *mut u128, (xmm0) as u128);
    core::ptr::write_unaligned((rsp.wrapping_add(0x10u64)) as *mut u128, (xmm1) as u128);
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, (rbx) as u64);
    rbx = (0xffffu32) as u32 as u64;
    r9 = ((r9 as u32)) as u32 as u64;
    loop { rcx = ((core::ptr::read_unaligned((r8) as *const u16)) as u32) as u32 as u64; rax = ((rsp) as u64) as u64; rdx = ((rcx as u32)) as u32 as u64; r10 = ((r10.wrapping_add(P1)) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; r8 = ((r8.wrapping_add(P2)) as u64) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; rax = (if (((fl_l as u16)) != ((fl_r as u16))) { rdx } else { rax }) as u64; let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = r10.wrapping_add(P3); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (core::ptr::read_unaligned((rsp.wrapping_add(0x20u64)) as *const u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn std_14236d960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (core::ptr::read_unaligned((rcx.wrapping_add(P0)) as *const u64)) as u64;
    rdx = (core::ptr::read_unaligned((rdx) as *const u64)) as u64;
    rax = (core::ptr::read_unaligned((rcx) as *const u64)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (core::ptr::read_unaligned((rax) as *const u64)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn unknown_1423bdc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x143426780; const P2: u64 = 0x143bbdde0; const P3: u64 = 0x143bbddd8; const P4: u64 = 0x14024ae10; const P5: u64 = 0x14384170c; const P6: u64 = 0x143426460; const P7: u64 = 0x14183ad8e; const P8: u64 = 0x143a3395c; const P9: u64 = 0x1425476d0; const P10: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x98u64)) as u64; xmm3 = ((core::ptr::read_unaligned((rcx) as *const u64)) as u64) as u128; xmm4 = ((core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u64)) as u64) as u128; xmm5 = ((core::ptr::read_unaligned((P0) as *const u32)) as u32) as u128; core::ptr::write_unaligned((rax.wrapping_sub(0x18u64)) as *mut u128, (xmm6) as u128); r8 = (rcx) as u64; core::ptr::write_unaligned((rax.wrapping_sub(0x28u64)) as *mut u128, (xmm7) as u128); core::ptr::write_unaligned((rax.wrapping_sub(0x38u64)) as *mut u128, (xmm8) as u128); xmm8 = ((core::ptr::read_unaligned((P1) as *const u32)) as u32) as u128; core::ptr::write_unaligned((rax.wrapping_sub(0x48u64)) as *mut u128, (xmm9) as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); core::ptr::write_unaligned((rax.wrapping_sub(0x58u64)) as *mut u128, (xmm10) as u128); core::ptr::write_unaligned((rax.wrapping_sub(0x78u64)) as *mut u64, ((xmm3 as u64)) as u64); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm7 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); core::ptr::write_unaligned((rax.wrapping_sub(0x68u64)) as *mut u128, (xmm11) as u128); xmm11 = (xmm1) as u128; xmm1 = crate::simd_ops::vshufps(xmm3, xmm3, 0x55u8); xmm3 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm4, xmm4, 0x55u8); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm9 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32))).to_bits() as u128); rcx = (core::ptr::read_unaligned((P2) as *const u32)) as u32 as u64; core::ptr::write_unaligned((rax.wrapping_sub(0x78u64)) as *mut u64, ((xmm4 as u64)) as u64); rax = (core::ptr::read_unaligned((P3) as *const u64)) as u64; xmm10 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; xmm6 = (xmm5) as u128; rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { xmm0 = ((core::ptr::read_unaligned((rax) as *const u32)) as u32) as u128; xmm1 = ((core::ptr::read_unaligned((rax.wrapping_add(0x4u64)) as *const u32)) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = ((core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u32)) as u32) as u128; xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((core::ptr::read_unaligned((rax.wrapping_add(0xcu64)) as *const u32)) as u32) as u128; xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32)).sqrt().to_bits() as u128); xmm1 = crate::simd_ops::vcmpss(xmm0, (xmm0 as u32), (xmm2 as u32), 0x1u8); xmm1 = crate::simd_ops::vblendvps(xmm2, xmm5, xmm1); xmm2 = ((core::ptr::read_unaligned((rax.wrapping_add(0x10u64)) as *const u32)) as u32) as u128; xmm0 = 0u128; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm1 = 0u128; xmm1 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); rax = ((rax).wrapping_add(0x14u64)) as u64; xmm2 = (xmm6) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((core::ptr::read_unaligned((rax.wrapping_add(0x4u64)) as *const u32)) as u32) as u128; xmm2 = ((core::ptr::read_unaligned((rax) as *const u32)) as u32) as u128; xmm3 = ((core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32)).sqrt().to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm7 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P6) as *const u32)) }).to_bits() as u128); xmm6 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((core::ptr::read_unaligned((P8) as *const u32)) as i32) as f32).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm11 as u32)).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (core::ptr::read_unaligned((rsp.wrapping_add(0x80u64)) as *const u128)) as u128; xmm7 = (core::ptr::read_unaligned((rsp.wrapping_add(0x70u64)) as *const u128)) as u128; xmm9 = (core::ptr::read_unaligned((rsp.wrapping_add(0x50u64)) as *const u128)) as u128; xmm10 = (core::ptr::read_unaligned((rsp.wrapping_add(0x40u64)) as *const u128)) as u128; xmm11 = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u128)) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(core::ptr::read_unaligned((P10) as *const u32))).to_bits() as u128); xmm8 = (core::ptr::read_unaligned((rsp.wrapping_add(0x60u64)) as *const u128)) as u128; rsp = ((rsp).wrapping_add(0x98u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn r_1424b8b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x14024ae10; const P2: u64 = 0x143428720; const P3: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; xmm0 = ((core::ptr::read_unaligned((rcx) as *const u32)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u32)) as u32 as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x50u64)) as *mut u128, (xmm6) as u128); r8 = (rdx) as u64; core::ptr::write_unaligned((rsp.wrapping_add(0x40u64)) as *mut u128, (xmm7) as u128); core::ptr::write_unaligned((rsp.wrapping_add(0x30u64)) as *mut u128, (xmm8) as u128); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = ((core::ptr::read_unaligned((rcx.wrapping_add(0x4u64)) as *const u32)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (core::ptr::read_unaligned((rcx.wrapping_add(0x10u64)) as *const u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { xmm4 = ((core::ptr::read_unaligned((P0) as *const u32)) as u32) as u128; xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rcx.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); xmm7 = ((core::ptr::read_unaligned((rcx.wrapping_add(0xcu64)) as *const u64)) as u64) as u128; xmm5 = ((core::ptr::read_unaligned((rcx) as *const u64)) as u64) as u128; xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(core::ptr::read_unaligned((rcx.wrapping_add(0x14u64)) as *const u32))).to_bits() as u128); xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm7, xmm7, 0x55u8); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = crate::simd_ops::vshufps(xmm5, xmm5, 0x55u8); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, ((xmm5 as u64)) as u64); core::ptr::write_unaligned((rsp.wrapping_add(0x20u64)) as *mut u64, ((xmm7 as u64)) as u64); xmm7 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = crate::simd_ops::vblendps(xmm7, xmm1, 0x1u8); xmm7 = crate::simd_ops::vinsertps(xmm7, xmm2, 0x10u8); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; xmm7 = crate::simd_ops::vinsertps(xmm7, xmm6, 0x20u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (((core::ptr::read_unaligned((r8.wrapping_add(0x180u64)) as *const u32)) as i32 as i64) as u64) as u64; xmm8 = ((core::ptr::read_unaligned((rax) as *const u64)) as u64) as u128; xmm8 = crate::simd_ops::vinsertps(xmm8, ((core::ptr::read_unaligned((rax.wrapping_add(0x8u64)) as *const u32)) as u128), 0x20u8); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm5 = (core::ptr::read_unaligned((P2) as *const u128)) as u128; xmm6 = (core::ptr::read_unaligned((P3) as *const u128)) as u128; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { xmm4 = (core::ptr::read_unaligned((r8) as *const u128)) as u128; xmm0 = (xmm4) & (xmm5); xmm1 = crate::simd_ops::vdpps(xmm0, xmm8, 0x7fu8); xmm2 = crate::simd_ops::vshufps(xmm4, xmm4, 0xffu8); xmm3 = (xmm1) ^ (xmm6); xmm0 = crate::simd_ops::vdpps(xmm4, xmm7, 0x7fu8); xmm1 = crate::simd_ops::vaddps(xmm2, xmm0); xmm2 = crate::simd_ops::vcmpps(xmm1, xmm3, 0x1u8); rax = (crate::simd_ops::vmovmskps(xmm2) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = 1u64; bb = 8; continue; }
            8 => { xmm6 = (core::ptr::read_unaligned((rsp.wrapping_add(0x50u64)) as *const u128)) as u128; xmm7 = (core::ptr::read_unaligned((rsp.wrapping_add(0x40u64)) as *const u128)) as u128; xmm8 = (core::ptr::read_unaligned((rsp.wrapping_add(0x30u64)) as *const u128)) as u128; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

pub unsafe fn diff_luau_142103170(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let buf: [u8; 48] = *b"AlanWake2_CauldronLake_LuauStringHashBenchmark!!";
    for len in [12u64, 44u64] {
        let o = f_o(buf.as_ptr() as usize, len);
        let n = f_n(buf.as_ptr() as usize, len);
        crate::diff_eq!(o, n, "142103170 len={}", len);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14213bb50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, flag: u8| -> u8 {
        let mut obj = [0u64; 24];
        obj[0x15] = 0;
        let op = obj.as_mut_ptr() as usize;
        f(op, flag);
        rd::<u8>(op + 0x90)
    };

    for flag in [0u8, 1u8] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        crate::diff_eq!(o, n, "14213bb50 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1421be190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, idx0: u16| -> [u64; 8] {
        let mut dst = [0u64; 8];
        let src: [u64; 8] = [11, 22, 33, 44, 55, 66, 77, 88];
        let indices: [u16; 2] = [idx0, 1];
        f(
            dst.as_mut_ptr() as usize,
            src.as_ptr() as usize,
            indices.as_ptr() as usize,
            2,
        );
        dst
    };

    for idx0 in [0u16, 0xffffu16] {
        let o = run(f_o, idx0);
        let n = run(f_n, idx0);
        crate::diff_eq!(o, n, "1421be190 idx0={}", idx0);
    }
    Ok(2)
}

pub unsafe fn diff_std_14236d960(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VFUNC_CALLED: (usize, u64) = (0, 0);
    unsafe extern "win64" fn mock_v0(this: usize, arg: u64) {
        VFUNC_CALLED = (this, arg);
    }

    let run = |f: F, arg_val: u64| -> (usize, u64) {
        VFUNC_CALLED = (0, 0);
        let vt: [usize; 2] = [mock_v0 as *const () as usize, 0];
        let target_obj: usize = vt.as_ptr() as usize;
        let callable: [usize; 2] = [0, (&target_obj as *const usize) as usize];
        f(callable.as_ptr() as usize, (&arg_val as *const u64) as usize);
        (VFUNC_CALLED.0.wrapping_sub( callable[1] ), VFUNC_CALLED.1)
    };

    for arg_val in [0x1234u64, 0x5678u64] {
        let o = run(f_o, arg_val);
        let n = run(f_n, arg_val);
        crate::diff_eq!(o, n, "14236d960 arg={}", arg_val);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423bdc80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let f_n: F = core::mem::transmute(new);

    static mut EXTENTS_OUT: [f32; 4] = [2.0, 2.0, 2.0, 0.0];
    unsafe extern "win64" fn mock_ae10(_p1: u64, _p2: usize) -> usize {
        core::ptr::addr_of_mut!(EXTENTS_OUT) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_4024_ae10, mock_ae10 as *const ())?;

    let g_ptr = exe::ptr::<usize>(0x1_43bb_ddd8) as usize;
    let g_cnt = exe::ptr::<u32>(0x1_43bb_dde0) as usize;
    let g_fov = exe::ptr::<f32>(0x1_4384_170c) as usize;
    let g_res = exe::ptr::<i32>(0x1_43a3_395c) as usize;
    let saved = (rd::<usize>(g_ptr), rd::<u32>(g_cnt), rd::<u32>(g_fov), rd::<i32>(g_res));

    let run = |is_orig: bool, scale: f32| -> u32 {
        let pts: [f32; 5] = [0.0, 0.0, 10.0, 100.0, 1.0];
        wr::<usize>(g_ptr, pts.as_ptr() as usize);
        wr::<u32>(g_cnt, 1);
        wr::<f32>(g_fov, 1.0);
        wr::<i32>(g_res, 1080);

        let aabb: [f32; 6] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let gpr = [aabb.as_ptr() as u64, scale.to_bits() as u64, 0, 0, 0, 0, 0, 0];
        let xmm = [0u128, scale.to_bits() as u128, 0u128, 0u128];
        if is_orig {
            crate::emu::emu_run_full((orig & !(1usize << 46)) as u64, gpr, xmm, [0; 8]).1 as u32
        } else {
            crate::simd_ops::set_tls_xmm_in(xmm);
            let _ = f_n(gpr[0], gpr[1], 0, 0);
            crate::simd_ops::take_tls_xmm_out(0) as u32
        }
    };

    for scale in [1.0f32, 4.0f32] {
        let o = run(true, scale);
        let n = run(false, scale);
        crate::diff_eq!(o, n, "1423bdc80 scale={}", scale);
    }

    wr::<usize>(g_ptr, saved.0);
    wr::<u32>(g_cnt, saved.1);
    wr::<u32>(g_fov, saved.2);
    wr::<i32>(g_res, saved.3);
    Ok(2)
}

pub unsafe fn diff_r_1424b8b00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut EXTENTS_B00: [f32; 4] = [1.0, 1.0, 1.0, 0.0];
    unsafe extern "win64" fn mock_ae10_b(_p1: u64, _p2: usize) -> usize {
        core::ptr::addr_of_mut!(EXTENTS_B00) as usize
    }
    let _m1 = crate::difftest::Mock::new(0x1_4024_ae10, mock_ae10_b as *const ())?;

    let run = |f: F, min_x: f32, max_x: f32| -> u8 {
        let aabb: [f32; 6] = [min_x, -1.0, -1.0, max_x, 1.0, 1.0];
        let mut frustum = [0u8; 0x190];
        wr::<i32>(frustum.as_mut_ptr() as usize + 0x180, 0);
        f(aabb.as_ptr() as usize, frustum.as_ptr() as usize)
    };

    for (min_x, max_x) in [(-1.0f32, 1.0f32), (5.0f32, 2.0f32)] {
        let o = run(f_o, min_x, max_x);
        let n = run(f_n, min_x, max_x);
        crate::diff_eq!(o, n, "1424b8b00 min={} max={}", min_x, max_x);
    }
    Ok(2)
}
