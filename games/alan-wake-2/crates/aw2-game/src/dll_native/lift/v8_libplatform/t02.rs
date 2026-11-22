#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eea404(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001eef900; const P1: u64 = 0x7001eef980; const P2: u64 = 0x7001eefa10; const P3: u64 = 0x7001efa428; const P4: u64 = 0x7001f01020;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (rcx) as u64;
    rax = ((rax).wrapping_sub(rdx)) as u64;
    rdx = ((P0) as u64) as u64;
    rax = ((rax).wrapping_sub(rdx)) as u64;
    rax = ((rax).wrapping_add(0xffffffffffffff80u64)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64;
    if (((fl_l as u64)) <= ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    r8 = ((P1) as u64) as u64;
    rdx = (rcx) as u64;
    rax = ((P2) as u64) as u64;
    rdx = ((rdx).wrapping_sub(r8)) as u64;
    rax = ((rax).wrapping_sub(r8)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if (((fl_l as u64)) > ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(P3)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(P3)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001ed1660 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eea4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001eef560; const P1: u64 = 0x7001eea7e2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eea4f8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ef9800; const P1: u64 = 0x7001efa434; const P2: u64 = 0x7001ef97f8; const P3: u64 = 0x7001ef97fc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r8 = ((rcx as u32)) as u32 as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = ((rbx as u32)) as u32 as u64; r8 = ((((r8 as u32)) ^ (0x6c65746eu32))) as u32 as u64; r9 = ((((r9 as u32)) ^ (0x756e6547u32))) as u32 as u64; r10 = ((rdx as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = ((((r9 as u32)) | ((r8 as u32)))) as u32 as u64; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r10 = ((((r10 as u32)) ^ (0x49656e69u32))) as u32 as u64; crate::simd_ops::wu32(rsp, ((rax as u32)) as u32); fl_l = ((((r9 as u32)) | ((r10 as u32)))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((rbx as u32)) as u32); rdi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_or_u64(P0, (0xffffffffffffffffu64) as u64); rax = ((((rax as u32)) & (0xfff3ff0u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x106c0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20660u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20670u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(0xfffcf9b0u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (0x100010001u64) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = ((((r8 as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); bb = 8; continue; }
            7 => { r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (0x7u32) as u32 as u64; r9 = ((rax.wrapping_sub(0x5u64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp, ((rax as u32)) as u32); r11 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((rdx as u32)) as u32); fl_l = ((((rbx as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((((r8 as u32)) | ((r9 as u32)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(P2, (0x1u32) as u32); crate::simd_ops::wu32(P3, ((r9 as u32)) as u32); fl_l = ((((rdi as u32)) >> (20u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(P2, ((r9 as u32)) as u32); rbx = (0x6u32) as u32 as u64; crate::simd_ops::wu32(P3, ((rbx as u32)) as u32); fl_l = ((((rdi as u32)) >> (27u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = ((((rdi as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = 7; rdx = 0; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = (((rdx) | (rax))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rbx as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(P2, (0x3u32) as u32); crate::simd_ops::wu32(P3, ((rax as u32)) as u32); fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(P2, (0x5u32) as u32); crate::simd_ops::wu32(P3, ((rax as u32)) as u32); rax = (0xd0030000u32) as u32 as u64; r11 = ((((r11 as u32)) & ((rax as u32)))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xe0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xe0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_or_u32(P3, (0x40u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eea704(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x1u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eea70c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001ef9820;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eebea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eebec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001f01020;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001ed1660 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eecece(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ee9e18;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eeceea(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ee9e18;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64));
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eecf03(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ee9c30;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eecf22(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ee8ba0; const P1: u64 = 0x7001ee9b7c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32);
    r9 = ((P0) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001eecf65(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xc0000005u32) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rax = ((rcx as u32)) as u32 as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}
