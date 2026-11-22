#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddcde0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001dc6090; const P2: u64 = 0x7001dc6b60; const P3: u64 = 0x7001dc5e60; const P4: u64 = 0x7001dc7310;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 85>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x2a0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x260u64), (rax) as u64); rdi = (rcx) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbx) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (r12) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (r13) as u64); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = ((r9 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; r8 = ((r8.wrapping_add(0x4u64)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rax).wrapping_sub(0x4u64), ((r9 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r13 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x210u64), (r9) as u64); fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x13u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x298u64), (rbp) as u64); rax = (0xaf286bcbu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r12 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x290u64), (rsi) as u64); rax = ((r12 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x278u64), (r14) as u64); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (r9) as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (0x13u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x270u64), (r15) as u64); rax = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64; r13 = ((((rax as u32)) as i32).wrapping_mul((0x13u32) as i32) as u32) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; r12 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { r8 = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (r9) as u64; rdx = (rsi) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru8(rdx.wrapping_add(r14))) as i8 as i32) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rbx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = (0x13u32) as u32 as u64; rcx = (rdi) as u64; rbp = (((rbp as u32)).wrapping_add(0x13u32)) as u32 as u64; rsi = ((rsi).wrapping_add(0x13u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x250u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((r8 as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; rax = ((rax.wrapping_add(0x4u64)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx).wrapping_sub(0x4u64), ((r8 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x250u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x254u64), ((r8 as u32)) as u32); rbx = ((rbx).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rcx = ((rbx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; fl_l = ((rbx).wrapping_shr(28u32)) as u64 as u64; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (0x3u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbx as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x250u64), ((rcx as u32)) as u32); bb = 13; continue; }
            13 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x250u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x254u64))) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (rdi) as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x254u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { r9 = (0x0u32) as u32 as u64; fl_l = (r15) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x270u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x278u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x290u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x298u64))) as u64; bb = 18; continue; }
            18 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rax = ((r12.wrapping_add(r13)) as u32) as u32 as u64; r8 = ((((r13 as u32)) as i32 as i64) as u64) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; rbx = (r9) as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 21; continue; } bb = 19; continue; }
            19 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rax = (((crate::simd_ops::ru8(r8.wrapping_add(r10))) as i8 as i32) as u32) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rbx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 20; continue; } bb = 21; continue; }
            21 => { rdx = ((r12 as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x288u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2c0u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; rdx = ((rcx as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x210u64), ((rcx as u32)) as u32); rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 23; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; bb = 26; continue; }
            26 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rdi.wrapping_add(0x214u64), (0x0u32) as u32); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x2a0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6d58; const P1: u64 = 0x7001dc7cd0; const P2: u64 = 0x7001ddd34b;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = (rdx) as u64; r11 = (rcx) as u64; r9 = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x200u64))) as u64; r8 = ((r8.wrapping_add(0x4u64)) as u64) as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rax).wrapping_sub(0x4u64), ((r15 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (0x92492493u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x210u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rsi) as u64); rcx = ((r8.wrapping_mul(4)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); let prod = (rax as u32 as i32 as i64).wrapping_mul(((rcx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rbx = ((rcx.wrapping_add(rdx)) as u32) as u32 as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(4u32) as u32)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 4; continue; }
            4 => { rdi = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(r14)) as u64; r10 = (r15) as u64; r8 = ((((rdi as u32)) as i32 as i64) as u64) as u64; rbp = ((((rbx as u32)) as i32 as i64) as u64) as u64; bb = 6; continue; }
            6 => { r9 = ((r15 as u32)) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (((crate::simd_ops::ru8(rsi.wrapping_add(r8))) as i8 as i32) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; bb = 13; continue; }
            9 => { rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rcx.wrapping_sub(0x57u64)) as u32) as u32 as u64; bb = 13; continue; }
            11 => { rax = ((rcx.wrapping_sub(0x41u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 34; continue; } bb = 12; continue; }
            12 => { rax = ((rcx.wrapping_sub(0x37u64)) as u32) as u32 as u64; bb = 13; continue; }
            13 => { rcx = ((rdx as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; r9 = (((r9 as u32)).wrapping_add((rax as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x4u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1cu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(r10.wrapping_mul(4)), ((r9 as u32)) as u32); r10 = ((r10).wrapping_add(1)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(r11.wrapping_add(0x210u64), ((rbx as u32)) as u32); rdx = ((r15 as u32)) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 16; continue; }
            16 => { r10 = (crate::simd_ops::ru64(r14)) as u64; r8 = (r15) as u64; r9 = ((((rdi as u32)) as i32 as i64) as u64) as u64; bb = 17; continue; }
            17 => { rcx = (((crate::simd_ops::ru8(r10.wrapping_add(r8))) as i8 as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(4u32)) as u32 as u64; rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (((rdx as u32)).wrapping_add(0xffffffd0u32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 23; continue; }
            19 => { rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (((rdx as u32)).wrapping_add(0xffffffa9u32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 23; continue; }
            21 => { rax = ((rcx.wrapping_sub(0x41u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 34; continue; } bb = 22; continue; }
            22 => { fl_l = (((rdx as u32)).wrapping_add(0xffffffc9u32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 24; continue; }
            24 => { rcx = ((rbx as u32)) as u32 as u64; bb = 25; continue; }
            25 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(r11.wrapping_add(0x210u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 27; continue; }
            27 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; bb = 29; continue; }
            29 => { rax = ((rbx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x210u64), ((rcx as u32)) as u32); rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 29; continue; } bb = 31; continue; }
            31 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu32(r11.wrapping_add(0x214u64), ((r15 as u32)) as u32); bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            34 => { rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc78f0; const P1: u64 = 0x7001dc5f70; const P2: u64 = 0x7001dc6bd0; const P3: u64 = 0x7001dc5710; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001ddd5cf;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; r9 = (((rdx as u16)) as u32) as u32 as u64; rbx = (rcx) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r8 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; rdx = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_sub(0x4u64), ((r8 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x214u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rax, (0x1u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x210u64), (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); rcx = ((r8 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((r8 as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; rdx = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_sub(0x4u64), ((r8 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x210u64), (r8) as u64); rsi = ((r8 as u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { r9 = (r9 & !0xffffu64) | (((((r9 as u16)).wrapping_shr(1u32)) as u16 as u64)); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { r14 = (((r9 as u16)) as u32) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 13; continue; }
            13 => { rcx = ((r8 as u32)) as u32 as u64; rax = (0x92492493u32) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rcx as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(4u32) as u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(31u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x2u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); rdi = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdi = (((rdi as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 17; continue; }
            17 => { rdi = (((((rdi as u32)) as i32).wrapping_shr(2u32) as u32)) as u32 as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (((r9 as u16)) as u32) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { rcx = (0x40u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = (0xffffffffu32) as u32 as u64; bb = 19; continue; }
            19 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 20; continue; }
            20 => { rdx = (((rdx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = (0x1u32) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; rax = ((rax).wrapping_sub(1)) as u64; rax = (!(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 24; continue; }
            23 => { r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 24; continue; }
            24 => { fl_l = (((((rdi as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 19; continue; } bb = 25; continue; }
            25 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = ((r14 as u32)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 27; continue; }
            27 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = ((r14 as u32)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 30; continue; }
            30 => { fl_l = (((((rdi as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = ((((rsi as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { rax = (0x92492493u32) as u32 as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rsi as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(4u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x214u64), ((rdx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = ((((rdx as u32)) as i32).wrapping_mul((0x1cu32) as i32) as u32) as u32 as u64; rcx = (rbx) as u64; rsi = (((rsi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0x4; const P3: u64 = 0xfffffffffffffffc;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rcx) as u64; r9 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = ((r10 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rcx = ((rcx.wrapping_add(P2)) as u64) as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P3), ((r10 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r8.wrapping_add(P0), (r10) as u64); fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rcx = (((rdx as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(P0), (0x1u32) as u32); bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0x4; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x8; const P5: u64 = 0xffffffffffffffff; const P6: u64 = 0x214;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (rdx) as u64; r9 = ((r11 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = ((r11 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = ((r8.wrapping_add(P2)) as u64) as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rax).wrapping_add(P3), ((r11 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), (r11) as u64); fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((((rdx as u32)) & (0xfffffffu32))) as u32 as u64; r10 = ((r10).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); rdx = ((r10 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((((rdx as u32)) & (0xfffffffu32))) as u32 as u64; fl_l = ((r10).wrapping_shr(28u32)) as u64 as u64; r10 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdx as u32)) as u32); rdx = (0x3u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((r10 as u32)) as u32); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); bb = 5; continue; }
            5 => { r8 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rdx = ((r8.wrapping_add(P5)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P3))) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P6), ((r11 as u32)) as u32); bb = 9; continue; }
            9 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x214; const P2: u64 = 0x200;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (((r8 as u32)).wrapping_add((r9 as u32))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; return rax; }
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0x4; const P3: u64 = 0xfffffffffffffffc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; r9 = (rcx) as u64; r11 = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rbx = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rdi = (0x1cu32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rbx = ((rbx.wrapping_add(P2)) as u64) as u64; rcx = ((rsi as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(rbx).wrapping_add(P3))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(((rcx as u32) & 31))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_add(P3), ((rax as u32)) as u32); rax = (((crate::simd_ops::ru32(r9.wrapping_add(P0))) as i32 as i64) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rax) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r8 as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0xfffffffffffffffc; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x214;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = (rcx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; bb = 5; continue; }
            5 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(r8.wrapping_add(P4), (0x0u32) as u32); bb = 7; continue; }
            7 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddd920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 2>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); r8 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x214u64))) as i32 as i64) as u64) as u64; r11 = (rdx) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; rbx = (rcx) as u64; r9 = (((crate::simd_ops::ru32(rdx.wrapping_add(0x214u64))) as i32 as i64) as u64) as u64; r10 = (((r10 as u32)).wrapping_add((r8 as u32))) as u32 as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x210u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 1; continue; }
            1 => { if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 2; continue; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rsi = (rcx) as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; rsi = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { r11 } else { rsi }) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 3; continue; }
            3 => { rbp = (r9) as u64; r14 = (r8) as u64; r8 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rax = (rbp) as u64; r9 = (r8) as u64; r9 = ((r9).wrapping_sub(rbp)) as u64; r9 = ((r9).wrapping_shl(2u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r15 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            6 => { fl_l = (r8) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; fl_l = ((rcx).wrapping_add(r15)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(r9))) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            11 => { fl_l = (r8) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(rax))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((r8).wrapping_sub(1)) as u64; r9 = ((r9).wrapping_sub(0x4u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x214u64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 16; continue; }
            16 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 19; continue; }
            17 => { rax = (0x1u32) as u32 as u64; bb = 19; continue; }
            18 => { rax = (0xffffffffu32) as u32 as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddda70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc7f80; const P1: u64 = 0x7001dc6f60; const P2: u64 = 0x7001dc5540; const P3: u64 = 0x7001dc8090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x214u64))) as u32 as u64; rsi = (rdx) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x210u64))) as u32 as u64; rdi = (rcx) as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(0x210u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x214u64)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((rbx as u16)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x214u64))) as u32 as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x214u64))) as u32 as u64; rbp = (((rbx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x210u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as i32 as i64) as u64) as u64; rdx = (rsi) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_add(crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64)))) as u16 as u64)); r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x214u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x214u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x210u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x200u64))) as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { let d = ((r15 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rcx = ((rax as u32)) as u32 as u64; rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_add((rax as u16))) as u16 as u64)); rcx = ((((rcx as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_sub(0x4u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; rdx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x210u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x214u64), ((rbx as u32)) as u32); bb = 17; continue; }
            13 => { rcx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rdx = (rsi) as u64; rcx = (rdi) as u64; r8 = ((rax as u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_add((rax as u16))) as u16 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rdi) as u64; rcx = (rsi) as u64; rbp = (rbp & !0xffffu64) | (((((rbp as u16)).wrapping_add(1)) as u16 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 16; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (((rbp as u16)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dddca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rdx = (rax) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dddcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001dc7e30; const P2: u64 = 0x7001dc5f70; const P3: u64 = 0x7001dc5710; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001dddeed;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x5u32) as u32); rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x19u32) as u32); rbx = (rcx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x7du32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x271u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0xc35u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x3d09u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1312du32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x5f5e1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1dcd65u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x9502f9u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2e90eddu32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0xe8d4a51u32) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rdi) as u64); rdi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1bu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rsi) as u64); rax = (0x2f684bdbu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rbp as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (r14) as u64); rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r14 = (0x6765c793fa10079du64) as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(4u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1bu32) as i32) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 6; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; r9 = ((r9.wrapping_add(0x4u64)) as u64) as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9).wrapping_sub(0x4u64))) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x48c27395u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (rdx) as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(r8.wrapping_add(r9).wrapping_sub(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 12; continue; }
            12 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; r8 = ((((r8 as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x210u64), (1) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rdi = (((rdi as u32)).wrapping_sub(0xdu32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rcx = (rbx) as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1cu64))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (0x92492493u32) as u32 as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((rbp as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbp as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(4u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x214u64), ((rdx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 20; continue; }
            20 => { rax = ((((rdx as u32)) as i32).wrapping_mul((0x1cu32) as i32) as u32) as u32 as u64; rcx = (rbx) as u64; rbp = (((rbp as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dddf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0x4; const P3: u64 = 0xfffffffffffffffc; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001dde068;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r8 = ((r8.wrapping_add(P2)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P3), ((rax as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r9.wrapping_add(P0), (rax) as u64); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { r11 = ((rdx as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r10).wrapping_add(P3))) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r10).wrapping_add(P3), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r8 = ((((r8 as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(r9.wrapping_add(P0), (1) as u32); rcx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            16 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6d58; const P1: u64 = 0x7001dc7cd0; const P2: u64 = 0x7001dde1e2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r11 = (rdx) as u64; r10 = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as u32 as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x200u64))) as u64; r8 = ((r8.wrapping_add(0x4u64)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_sub(0x4u64), ((rax as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r10.wrapping_add(0x210u64), (rax) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); rsi = ((r11 as u32)) as u32 as u64; r11 = ((r11).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rbx = (rax) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x200u64))) as u64; rbx = ((rbx.wrapping_add(0x4u64)) as u64) as u64; rcx = (rax) as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(rbx).wrapping_sub(0x4u64))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rsi) as i64) as u64) as u64; r9 = (((r9) as i64).wrapping_mul((r11) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = ((r8).wrapping_shr(28u32)) as u64; rcx = ((((rcx as u32)) & (0xfffffffu32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_sub(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x210u64))) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x200u64))) as u64; r8 = ((((r8 as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((r8 as u32)) as u32); crate::simd_ops::lk_add_u32(r10.wrapping_add(0x210u64), (1) as u32); rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x210u64))) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            15 => { rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x214u64))) as u32 as u64; r15 = (r8) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x214u64))) as u32 as u64; r10 = (rcx) as u64; r9 = (((r9 as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(0x210u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x210u64)))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (r10) as u64; r10 = (rdx) as u64; rdx = (rax) as u64; rcx = (crate::simd_ops::ru32(r10.wrapping_add(0x214u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x214u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(r10.wrapping_add(0x210u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(0x210u64)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (((crate::simd_ops::ru32(r10.wrapping_add(0x214u64))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru32(r10.wrapping_add(0x210u64))) as u32 as u64; rsi = (((crate::simd_ops::ru32(r8.wrapping_add(0x214u64))) as i32 as i64) as u64) as u64; r14 = (((r14 as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x210u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rdi) as u64); rbp = (((rbp as u32)).wrapping_add((rsi as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); rax = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r13) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 3; continue; }
            3 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 30; continue; } bb = 4; continue; }
            4 => { rdi = (((crate::simd_ops::ru32(rdx.wrapping_add(0x214u64))) as i32 as i64) as u64) as u64; r12 = (crate::simd_ops::ru32(rdx.wrapping_add(0x210u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 6; continue; }
            6 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); r8 = ((rsp.wrapping_add(0x24u64)) as u64) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x2cu64)) as u64) as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { r8 } else { rcx }) as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru32(rcx)) as i32 as i64) as u64) as u64; r11 = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 29; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rax) as u64); r13 = (rdi) as u64; rcx = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rdi = (rsi) as u64; r8 = ((((r11 as u32)) as i32 as i64) as u64) as u64; rax = (r13) as u64; crate::simd_ops::wu64(rsp, (rsi) as u64); rax = ((rax).wrapping_sub(rcx)) as u64; rbx = (r8) as u64; rbx = ((rbx).wrapping_sub(r13)) as u64; rbx = ((rbx).wrapping_shl(2u32)) as u64; rsi = ((rax.wrapping_mul(4)) as u64) as u64; rax = (r13) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rax = ((rax.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rax) as u64); bb = 10; continue; }
            10 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            12 => { fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            14 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x200u64))) as u64; rcx = ((rsi.wrapping_add(rbx)) as u64) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            17 => { fl_l = (r8) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x200u64))) as u64; rdi = (crate::simd_ops::ru32(rbx.wrapping_add(rax))) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 25; continue; }
            22 => { fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp)) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 25; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x200u64))) as u64; fl_l = ((rcx).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; bb = 25; continue; }
            25 => { rdi = (((rdi as u32)).wrapping_add((rsi as u32))) as u32 as u64; rax = ((rcx.wrapping_add(r9)) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = (((rcx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r11 = (((r11 as u32)).wrapping_sub(1)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((r8).wrapping_sub(1)) as u64; r9 = (((r9 as u32)).wrapping_shl(28u32)) as u32 as u64; rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 28; continue; }
            28 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 32; continue; }
            30 => { rax = (0x1u32) as u32 as u64; bb = 32; continue; }
            31 => { rax = (0xffffffffu32) as u32 as u64; bb = 32; continue; }
            32 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x1; const P2: u64 = 0x214; const P3: u64 = 0x7001dc5710; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001dde55c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x92492493u32) as u32 as u64; let prod = (rax as u32 as i32 as i64).wrapping_mul(((r8 as u32)) as i32 as i64) as u64; rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(4u32) as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), ((rdx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((((rdx as u32)) as i32).wrapping_mul((0x1cu32) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            3 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6d58; const P1: u64 = 0x7001dc7cd0; const P2: u64 = 0x7001dde77c; const P3: u64 = 0x7001df6d90; const P4: u64 = 0x7001dde789;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbx = (((crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as i32 as i64) as u64) as u64; r11 = (rcx) as u64; rsi = ((rbx.wrapping_add(rbx)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x100u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbp) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r12) as u64); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r14) as u64); r12 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rdx = ((r9.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9.wrapping_add(0x4u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(r8).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { r9 = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rcx = ((r12.wrapping_mul(4)) as u64) as u64; r8 = ((rcx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; fl_l = ((r8).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; r10 = ((rcx.wrapping_add(rdx)) as u64) as u64; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = ((r8.wrapping_sub(0x4u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; r10 = ((r10.wrapping_add(0x4u64)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdi.wrapping_mul(4)), ((rdx as u32)) as u32); rdi = ((rdi).wrapping_add(1)) as u64; rbx = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 11; continue; }
            11 => { r14 = ((((rbx as u32)) as i32 as i64) as u64) as u64; r15 = ((((rsi as u32)) as i32 as i64) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; rdi = ((rbx as u32)) as u32 as u64; rdx = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rcx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rdx = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r9 = ((r8.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rdx.wrapping_add(r12)) as u64) as u64; r10 = ((r8.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r8 = ((rbp as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru32(r9)) as u32 as u64; r9 = ((r9.wrapping_add(0x4u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; r10 = ((r10.wrapping_sub(0x4u64)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xfffffffu32))) as u32 as u64; rax = ((rax).wrapping_shr(28u32)) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r14.wrapping_mul(4)), ((rdx as u32)) as u32); r14 = ((r14).wrapping_add(1)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(0x214u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(r11.wrapping_add(0x210u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_add(0x214u64), ((rax as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rcx = ((rsi as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rsi = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rsi.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; rcx = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(r11.wrapping_add(0x210u64), ((rcx as u32)) as u32); rsi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; bb = 21; continue; }
            21 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(r11.wrapping_add(0x214u64), (0x0u32) as u32); bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            24 => { rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); return rax; }
            25 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc7f80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rdi = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x214u64))) as u32 as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x214u64)))) as u32 as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r14) as u64); fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; r10 = ((rsi.wrapping_mul(4)) as u64) as u64; r14 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x200u64))) as u64; rcx = ((r14.wrapping_add(r10)) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; r10 = ((r10.wrapping_add(0x4u64)) as u64) as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(r10).wrapping_sub(0x4u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(rax)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(31u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(r10).wrapping_sub(0x4u64), ((rax as u32)) as u32); fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x210u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((((r11 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdx).wrapping_shl(2u32)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; rdx = ((rdx.wrapping_add(0x4u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(31u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_sub(0x4u64), ((rax as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; rdx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x210u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0x214u64), (0x0u32) as u32); bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001dde990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc8090;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 19; continue; } bb = 2; continue; }
            2 => { rdi = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rdi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r10 = (crate::simd_ops::ru32(rdx.wrapping_add(0x214u64))) as u32 as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (((r10 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x214u64)))) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); rbp = ((((r8 as u32)) as i32 as i64) as u64) as u64; r11 = ((rax.wrapping_mul(4)) as u64) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; r14 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x200u64))) as u64; rcx = ((r14.wrapping_add(r11)) as u64) as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r11 = ((r11.wrapping_add(0x4u64)) as u64) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; rax = ((r9 as u32)) as u32 as u64; r8 = (((r8) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r11).wrapping_sub(0x4u64))) as u32 as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = ((r8 as u32)) as u32 as u64; r9 = (r8) as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r9 = ((r9).wrapping_shr(28u32)) as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(31u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r11).wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; r8 = ((rax.wrapping_add(r10)) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; r10 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 11; continue; }
            11 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r10))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(31u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffu32))) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r10), ((rax as u32)) as u32); r10 = ((r10).wrapping_add(0x4u64)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x200u64))) as u64; bb = 15; continue; }
            15 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x210u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_add(0x214u64), ((rax as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddeb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); r10 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x210u64))) as i32 as i64) as u64) as u64; rdi = ((r8 as u32)) as u32 as u64; r11 = (rcx) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu16(rdx, (0x30u16) as u16); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x200u64))) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = ((rbx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(r10.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (((r8 as u32)).wrapping_shr(4u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x214u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r10 as u32))) as u32 as u64; r10 = ((((rcx as u32)) as i32).wrapping_mul((0x7u32) as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(0xfffffffau32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            7 => { r10 = (((r10 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = ((rbx as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rdx), ((rbx as u8)) as u8); fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x214u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(0x7u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_sub(0x3u64), (0x30303030u32) as u32); crate::simd_ops::wu16(rax.wrapping_add(rdx).wrapping_sub(0x5u64), (0x3030u16) as u16); crate::simd_ops::wu8(rdx.wrapping_add(rax).wrapping_sub(0x6u64), (0x30u8) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(0x214u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; rsi = (0x37u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdi = (rbx) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rdi = ((rdi.wrapping_add(0x4u64)) as u64) as u64; r8 = ((((r10 as u32)) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(rax).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (0x30u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx), ((rax as u8)) as u8); rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx).wrapping_sub(0x1u64), ((rax as u8)) as u8); rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx).wrapping_sub(0x2u64), ((rax as u8)) as u8); rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx).wrapping_sub(0x3u64), ((rax as u8)) as u8); rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx).wrapping_sub(0x4u64), ((rax as u8)) as u8); rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = ((((r9 as u32)) & (0xfu32))) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx).wrapping_sub(0x5u64), ((rax as u8)) as u8); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (0x30u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(0x7u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((r9 as u8))) as u8 as u64)); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r8).wrapping_sub(0x6u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x210u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x200u64))) as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { r9 = ((((r10 as u32)) as i32 as i64) as u64) as u64; fl_l = ((r9).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rcx = ((r8 as u32)) as u32 as u64; r9 = ((r9.wrapping_sub(0x1u64)) as u64) as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; rax = (0x30u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(4u32)) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rcx as u8))) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(0x1u64), ((rax as u8)) as u8); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddee30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x200; const P2: u64 = 0x4; const P3: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = ((r9 as u32)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { crate::simd_ops::wu64(rcx.wrapping_add(P0), (r9) as u64); return rax; }
    r8 = ((r9 as u32)) as u32 as u64;
    loop { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = ((r8.wrapping_add(P2)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rax).wrapping_add(P3), ((r9 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { break; } }
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddee80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3f; const P1: u64 = 0x7001df6d80; const P2: u64 = 0x7001defd90; const P3: u64 = 0x7001df6dd0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = (r9) as u64; xmm0 = (((rax as u32)) as u32) as u128; rdi = (r8) as u64; xmm0 = crate::simd_ops::vcvtdq2pd(xmm0); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x15bu32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((P3) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; xmm0 = (xmm1) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rcx = ((xmm0 as u64)) as u64; rax = (rcx) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = (rax as u16 as i16 as i32 as u32) as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((((rcx as u16)) as i16 as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddef30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x15c; const P1: u64 = 0x7001df6dc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64;
    r9 = (rdx) as u64;
    rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64;
    rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add((rdx as u32))) as u32 as u64;
    rax = (((((rax as u32)) as i32).wrapping_shr(3u32) as u32)) as u32 as u64;
    rcx = ((((rax as u32)) as i32 as i64) as u64) as u64;
    rax = ((P1) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128;
    crate::simd_ops::wu64(rsp, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8);
    rcx = ((xmm0 as u64)) as u64;
    rax = ((((rcx as u16)) as i16 as i32) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    rcx = ((rcx).wrapping_shr(16u32)) as u64;
    rax = ((((rcx as u16)) as i16 as i32) as u32) as u32 as u64;
    crate::simd_ops::wu128(r9, (xmm0) as u128);
    crate::simd_ops::wu32(r8, ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddefa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (rcx) as u64; r8 = ((rax as u32)) as u32 as u64; rdi = (rax) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r11 = (rax) as u64; r9 = ((rax as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdx = (0x80000000u32) as u32 as u64; rdi = ((rdi).wrapping_shr(32u32)) as u64; rax = (((rax as u32)).wrapping_add(0x40u32)) as u32 as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), ((rax as u32)) as u32); r10 = (((r10) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) as i64).wrapping_mul((r8) as i64) as u64) as u64; r11 = ((r11).wrapping_shr(32u32)) as u64; rbx = (r11) as u64; rax = ((r10 as u32)) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rcx = ((rbx as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; r9 = ((r9).wrapping_shr(32u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r10 = ((r10).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; fl_l = ((rdx).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi, (rdx) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddf050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6bf0; const P1: u64 = 0x7001dc65b0; const P2: u64 = 0x7001dc5210; const P3: u64 = 0x7001dc6900; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001ddf1fc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); rcx = (0x8000000000000000u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (r9) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r15 = ((r8 as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; xmm6 = (xmm0) as u128; fl_l = (((rax) & (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm6 = (xmm6) ^ (crate::simd_ops::ru128(P0)); bb = 2; continue; }
            2 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rdi) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; xmm0 = 0u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 3; continue; }
            3 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9)) as u64; rdi = (0x1u32) as u32 as u64; crate::simd_ops::wu16(rax, (0x30u16) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); bb = 23; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbp) as u64); rdi = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r14) as u64); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu8(rax, ((r15 as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rax, ((r15 as u32)) as u32); bb = 22; continue; }
            8 => { rcx = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { rdx = ((rdi as u32)) as u32 as u64; bb = 14; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = (rbp) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((r15 as u32)) as u32 as u64; xmm0 = (xmm6) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            13 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = (xmm6) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rdi = (0x2u32) as u32 as u64; bb = 21; continue; }
            20 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rdi as u32)) as u32 as u64; xmm0 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (((crate::simd_ops::ru32(rbp)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x0u8) as u8); bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddf270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc4000; const P1: u64 = 0x7001df6d58; const P2: u64 = 0x7001dc7cd0; const P3: u64 = 0x7001ddf3c5;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r10 = ((P0) as u64) as u64; rax = ((crate::simd_ops::ru8(r10.wrapping_add(rax).wrapping_add(0x1b3f4u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1b3c8u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r10)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x3b9aca00u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(r8, (0x3b9aca00u32) as u32); crate::simd_ops::wu32(r9, (0x9u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x5f5e100u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r8, (0x5f5e100u32) as u32); crate::simd_ops::wu32(r9, (0x8u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x989680u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r8, (0x989680u32) as u32); crate::simd_ops::wu32(r9, (0x7u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xf4240u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r8, (0xf4240u32) as u32); crate::simd_ops::wu32(r9, (0x6u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x186a0u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(r8, (0x186a0u32) as u32); crate::simd_ops::wu32(r9, (0x5u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x2710u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(r8, (0x2710u32) as u32); crate::simd_ops::wu32(r9, (0x4u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x3e8u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(r8, (0x3e8u32) as u32); crate::simd_ops::wu32(r9, (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x64u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(r8, (0x64u32) as u32); crate::simd_ops::wu32(r9, (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(r8, (0xau32) as u32); crate::simd_ops::wu32(r9, (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r8, (0x1u32) as u32); crate::simd_ops::wu32(r9, ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            22 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); crate::simd_ops::wu32(r9, (0xffffffffu32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            23 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((P1) as u64) as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); crate::simd_ops::wu32(r9, ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x7001ddf270u64) { 0xd => { bb = 1; continue; } 0x2d => { bb = 2; continue; } 0x35 => { bb = 3; continue; } 0x48 => { bb = 4; continue; } 0x50 => { bb = 5; continue; } 0x63 => { bb = 6; continue; } 0x6b => { bb = 7; continue; } 0x7e => { bb = 8; continue; } 0x86 => { bb = 9; continue; } 0x99 => { bb = 10; continue; } 0xa1 => { bb = 11; continue; } 0xb4 => { bb = 12; continue; } 0xbc => { bb = 13; continue; } 0xcf => { bb = 14; continue; } 0xd7 => { bb = 15; continue; } 0xea => { bb = 16; continue; } 0xef => { bb = 17; continue; } 0x102 => { bb = 18; continue; } 0x107 => { bb = 19; continue; } 0x11a => { bb = 20; continue; } 0x11f => { bb = 21; continue; } 0x130 => { bb = 22; continue; } 0x141 => { bb = 23; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddf480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ddf270;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (rax) as u64; r13 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rdx as u32)) as u32); rsi = (r9) as u64; r12 = (r8) as u64; r13 = (0u32.wrapping_sub((r13 as u32))) as u32 as u64; r9 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((r13 as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x78u64)) as u64) as u64; r15 = ((r15).wrapping_shr(((rcx as u32) & 63))) as u64; rdi = (0x1u32) as u32 as u64; r14 = ((rdi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x40u32)) as u32 as u64; r14 = ((r14).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((r15 as u32)) as u32 as u64; rbx = ((r14.wrapping_sub(0x1u64)) as u64) as u64; rbx = (((rbx) & (rax))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu32(rsi, (0x0u32) as u32); fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(r12)) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add(0x30u8)) as u8 as u64)); r15 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(r10), ((rax as u8)) as u8); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; rcx = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 3; continue; }
            3 => { rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; bb = 5; continue; }
            5 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 38; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r10 = ((r14.wrapping_sub(0x1u64)) as u64) as u64; bb = 7; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r12)) as u64; rbx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rcx = ((r8 as u32)) as u32 as u64; rdx = (rbx) as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = ((rdx).wrapping_shr(((rcx as u32) & 63))) as u64; rbp = (((rbp as u32)).wrapping_sub(1)) as u32 as u64; rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(0x30u8)) as u8 as u64)); rbx = (((rbx) & (r10))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdx as u8)) as u8); crate::simd_ops::lk_add_u32(rsi, (1) as u32); crate::simd_ops::lk_sub_u32(r9, (1) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (rdi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 38; continue; } bb = 11; continue; }
            11 => { rax = (r14) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 12; continue; }
            12 => { rcx = (r14) as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rbx.wrapping_add(rbx)) as u64) as u64; r14 = ((r14).wrapping_sub(rax)) as u64; rax = ((rdi.wrapping_add(rdi)) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_sub(rdi)) as u64; rax = ((rdi.wrapping_add(rcx)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 38; continue; } bb = 16; continue; }
            16 => { rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::lk_add_u8(rcx.wrapping_add(r10).wrapping_sub(0x1u64), (1) as u8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(r10)) as u64; r8 = (r10) as u64; fl_l = 0; fl_r = (r8) as u64 as u64; r8 = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; bb = 18; continue; }
            18 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu8(rax, (0x30u8) as u8); rax = ((rax).wrapping_sub(1)) as u64; crate::simd_ops::lk_add_u8(rax, (1) as u8); rcx = ((r8.wrapping_add(rax)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(r10)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu8(r10, (0x31u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(r9, (1) as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 39; continue; }
            23 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 25; continue; }
            25 => { xmm0 = (crate::simd_ops::ru128(r12)) as u128; rcx = ((r13 as u32)) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; r10 = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; r10 = ((r10).wrapping_shl(((rcx as u32) & 63))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (r10) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 26; continue; }
            26 => { rax = ((r10.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 27; continue; }
            27 => { rcx = (r10) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rdx.wrapping_add(rdx)) as u64) as u64; r10 = ((r10).wrapping_sub(rax)) as u64; fl_l = (r10) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 29; continue; }
            29 => { fl_l = (rdx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 38; continue; } bb = 30; continue; }
            30 => { rcx = ((rcx).wrapping_add(1)) as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 38; continue; } bb = 31; continue; }
            31 => { rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::lk_add_u8(rcx.wrapping_add(r10).wrapping_sub(0x1u64), (1) as u8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r10))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(r10)) as u64) as u64; r8 = (r10) as u64; fl_l = 0; fl_r = (r8) as u64 as u64; r8 = (0u64.wrapping_sub(fl_r as u64)) as u64; bb = 33; continue; }
            33 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu8(rcx, (0x30u8) as u8); rcx = ((rcx).wrapping_sub(1)) as u64; crate::simd_ops::lk_add_u8(rcx, (1) as u8); rax = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 33; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru8(r10)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu8(r10, (0x31u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(r9, (1) as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 39; continue; }
            38 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddf7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ddee80; const P1: u64 = 0x7001dc7760; const P2: u64 = 0x7001ddf480; const P3: u64 = 0x7001ddf9e0; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001ddf971;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (r9) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((r10 as u32)) as u32); rsi = ((r8 as u32)) as u32 as u64; xmm1 = (xmm0) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = (0xfffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = (0x10000000000000u64) as u64; rcx = (rax) as u64; r8 = (0x7ff0000000000000u64) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = (rax) as u64; fl_l = (((rdx) & (r8))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; r8 = ((rcx.wrapping_add(r9)) as u64) as u64; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { r8 }) as u64; rax = ((rax).wrapping_shr(52u32)) as u64; rax = ((((rax as u32)) & (0x7ffu32))) as u32 as u64; rcx = (0xfffffbceu32) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x433u32)) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(r8)) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_sub(0xbu64)) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(11u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (0xffffff8fu32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r8) as u64); rdx = (0xffffffabu32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r10) as u64); rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r10 as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((rsp.wrapping_add(0x98u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r9 = (rbx) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r9 = ((rsp.wrapping_add(0x98u64)) as u64) as u64; r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = (xmm1) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rbx))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); rdx = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), (0x0u8) as u8); bb = 8; continue; }
            8 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddf9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001ddee80; const P1: u64 = 0x7001dc7760; const P2: u64 = 0x7001ddf270; const P3: u64 = 0x7001ddfe20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=4
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; r11 = (0x10000000000000u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r10 = (0x7ff0000000000000u64) as u64; rcx = (rax) as u64; r9 = (rax) as u64; rbx = (rdx) as u64; rdi = (0xfffffffffffffu64) as u64; rcx = (((rcx) & (rdi))) as u64; rsi = (0xfffffbceu32) as u32 as u64; fl_l = (((r9) & (r10))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; r15 = (r8) as u64; rdx = ((rcx.wrapping_add(r11)) as u64) as u64; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rdx }) as u64; rax = ((rax).wrapping_shr(52u32)) as u64; rax = ((((rax as u32)) & (0x7ffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x433u32)) as u32 as u64; fl_l = (r10) as u64 as u64; fl_r = (r9) as u64 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (r11) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (r11) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_sub(0xbu64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(11u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rdx) as u64); fl_l = (((r8) & (r10))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; bb = 5; continue; }
            4 => { rdx = (rcx) as u64; rdx = ((rdx).wrapping_shr(52u32)) as u64; rdx = ((((rdx as u32)) & (0x7ffu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x433u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (((rcx) & (rdi))) as u64; r10 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r10 as u32)) as u32); r8 = ((r10 as u32)) as u32 as u64; rdi = (0xffc0000000000000u64) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { r9 }) as u64; rcx = ((r9.wrapping_mul(2).wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_shl(10u32)) as u64; r8 = (((r8 as u32)).wrapping_sub(0xau32)) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) as i64) >= 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (rcx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r8 as u32)) as u32); fl_l = (r9) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((r9.wrapping_mul(4).wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); r10 = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rdx = ((r9.wrapping_mul(2).wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdx) as u64); bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r10 as u32)) as u32); r9 = ((rbp.wrapping_sub(0x75u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r10 = (((r10 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm0) as u128); rcx = (((r10 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), ((crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))).wrapping_shl(((rcx as u32) & 63))) as u64); rdx = (0xffffffabu32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (0xffffff8fu32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; r9 = ((rbp.wrapping_add(0x77u64)) as u64) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; r8 = ((rbp.wrapping_sub(0x79u64)) as u64) as u64; r13 = ((r13).wrapping_add(1)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm1 = crate::simd_ops::vpsrldq(xmm1, 0x8u8); r14 = (0x1u32) as u32 as u64; rax = ((xmm1 as u32)) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; r12 = (r13) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); rdi = (r13) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (r13) as u64); rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64)))) as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; rdi = ((rdi).wrapping_add(1)) as u64; r12 = ((r12).wrapping_shr(((rcx as u32) & 63))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (rdx) as u64); rcx = ((rdx.wrapping_sub(0x1u64)) as u64) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rcx) as u64); rcx = ((r12 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; rbx = (((rbx) & (r13))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r15, (0x0u32) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x79u64))) as u32 as u64; fl_l = 0; fl_r = ((r9 as u32)) as u32 as u64; r9 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rcx = (((crate::simd_ops::ru32(r15)) as i32 as i64) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add(0x30u8)) as u8 as u64)); r12 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(r11), ((rax as u8)) as u8); rcx = ((r9 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; crate::simd_ops::wu32(r15, ((r10 as u32)) as u32); fl_l = (rdx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; fl_l = 0; fl_r = ((r13 as u32)) as u32 as u64; r13 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (((crate::simd_ops::ru32(r15)) as i32 as i64) as u64) as u64; rbx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r14 = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; rdx = (rbx) as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rcx = ((r13 as u32)) as u32 as u64; r14 = ((r14).wrapping_add(r14)) as u64; rdx = ((rdx).wrapping_shr(((rcx as u32) & 63))) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(0x30u8)) as u8 as u64)); rbx = (((rbx) & (r8))) as u64; crate::simd_ops::wu8(rax.wrapping_add(r11), ((rdx as u8)) as u8); rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r15, ((rdx as u32)) as u32); fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_add(0x7u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); r8 = (((r8) as i64).wrapping_mul((r14) as i64) as u64) as u64; bb = 20; continue; }
            20 => { r9 = (rdi) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rsi = (((rsi as u32)).wrapping_sub(crate::simd_ops::ru32(rbp.wrapping_sub(0x75u64)))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rsi as u32)) as u32); rsp = ((rsp).wrapping_add(0xe8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = (r13) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64 as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); r8 = (r13) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); rdx = ((r10 as u32)) as u32 as u64; bb = 20; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddfe20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r10 = (r8) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r10 = ((r10).wrapping_sub(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); r11 = (r9) as u64; rsi = (rcx) as u64; rbx = ((r8.wrapping_add(r14)) as u64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rdi = ((((rdx as u32)) as i32 as i64) as u64) as u64; bb = 2; continue; }
            2 => { rcx = (r11) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { r9 = ((rax.wrapping_add(r8)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (rax) as u64; rcx = (r10) as u64; rdx = ((rdx).wrapping_sub(r10)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_sub_u8(rax.wrapping_add(rdi).wrapping_sub(0x1u64), (1) as u8); rax = (r9) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (r11) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(r8)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (rax) as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rbx = ((rbx).wrapping_sub(rax)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = ((r14.wrapping_add(r14)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = ((r14.wrapping_mul(4)) as u64) as u64; r11 = ((r11).wrapping_sub(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001ddff30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de02c0; const P1: u64 = 0x7001de09d0; const P2: u64 = 0x7001de0390; const P3: u64 = 0x7001de0de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); r12 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = (0xfffffffffffffu64) as u64; rcx = (((rcx) & (rdx))) as u64; rbp = (0x10000000000000u64) as u64; rdx = (0x7ff0000000000000u64) as u64; rsi = (r9) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; r15 = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = ((rcx.wrapping_add(rbp)) as u64) as u64; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbp }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (0xfffffbceu32) as u32 as u64; bb = 3; continue; }
            2 => { rbx = ((rbx).wrapping_shr(52u32)) as u64; rbx = ((((rbx as u32)) & (0x7ffu32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub(0x433u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x14u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 3; continue; }
            3 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x14u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); rax = ((rbx.wrapping_add(0x35u64)) as u32) as u32 as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (r14) as u64); crate::simd_ops::wu32(r9, ((rdi as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xb1a2bc2ec5u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x11u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_sub(0x11u64)) as u32) as u32 as u64; rbp = ((rbp).wrapping_shl(((rcx as u32) & 63))) as u64; rax = (rbp) as u64; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbp = (rdx) as u64; fl_l = ((rbp).wrapping_shl(17u32)) as u64 as u64; rbp = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { rax = (rbp) as u64; rcx = (0x11u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r8 = ((r8).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((rbx as u32)) as u32 as u64; let d = (r8) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbp = (rdx) as u64; fl_l = ((rbp).wrapping_shl(((rcx as u32) & 63))) as u64 as u64; rbp = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(r15)) as u128; r8 = (rsi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rsi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (0x11u32) as u32 as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 19; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; rcx = ((rbx as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rbp = ((rbp).wrapping_shl(((rcx as u32) & 63))) as u64; r8 = (rsi) as u64; rcx = (rbp) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 19; continue; }
            11 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xffffffcbu32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; r9 = (rbp) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rbx as u32)) as u32 as u64; r8 = (rsi) as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((r9).wrapping_shr(((rcx as u32) & 63))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; rbp = ((rbp).wrapping_sub(rax)) as u64; rax = (0xffffffffu32) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 18; continue; }
            14 => { rcx = ((r9 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 18; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xffffff80u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rax, ((rdi as u8)) as u8); rax = ((r12 as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(r9, ((rdi as u32)) as u32); crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 19; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu32(r14, ((rdi as u32)) as u32); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(r15)) as u128; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 20; continue; }
            20 => { r8 = ((rcx as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rax = ((r8 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx).wrapping_sub(0x1u64))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 21; continue; } bb = 23; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (rdx) as u64; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 27; continue; }
            27 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { r9 = ((rdi as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 29; continue; }
            29 => { r8 = ((((rdi as u32)) as i32 as i64) as u64) as u64; fl_l = ((r8).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); rdx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 30; continue; } bb = 31; continue; }
            31 => { rcx = (((rcx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = ((((rcx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x0u8) as u8); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = 0; fl_r = ((r12 as u32)) as u32 as u64; r12 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); bb = 34; continue; }
            34 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de02c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = (r8) as u64; rbx = (rdx) as u64; r9 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(r11)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add(0x30u8)) as u8 as u64)); rax = (rax as u32 as i32 as i64) as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(r8), ((rcx as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = (((crate::simd_ops::ru32(r11)) as i32 as i64) as u64) as u64; r8 = (rcx) as u64; rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r9))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(r9))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(r9), ((rcx as u8)) as u8); r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu8(rax.wrapping_add(r9), ((rdx as u8)) as u8); rax = ((rax).wrapping_sub(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_add_u32(r11, ((r10 as u32)) as u32)) as u32 as u64; fl_r = fl_l; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = (((rcx as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r11, ((rcx as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de0390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de02c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; rbp = (rdx) as u64; rsi = (r8) as u64; r8 = (0xd6bf94d5e57a42bdu64) as u64; rax = (r8) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (r8) as u64; rdi = (rdx) as u64; rdi = ((rdi).wrapping_shr(23u32)) as u64; let prod = (rax as u128).wrapping_mul((rdi) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((((rdi as u32)) as i32).wrapping_mul((0x989680u32) as i32) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(23u32)) as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x989680u32) as i32) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (rsi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (crate::simd_ops::ru64(rbp)) as u64; rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rdi as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x6u64), ((rdi as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x5u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x4u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x3u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); r11 = (r10) as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10), ((r8 as u8)) as u8); r8 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rax = ((r8.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rbx as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r10).wrapping_add(0xdu64), ((rbx as u8)) as u8); r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x5u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x4u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x3u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r10 = ((rdx as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r10)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r11).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r10 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rcx = (((r9 as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); r8 = ((rcx.wrapping_add(r9)) as u32) as u32 as u64; rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add((r8 as u8))) as u8 as u64)); r10 = (r10 & !0xffu64) | (((((r10 as u8)).wrapping_sub((r8 as u8))) as u8 as u64)); r10 = (r10 & !0xffu64) | (((((r10 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(r11).wrapping_add(0x1u64), ((r10 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r11), ((r9 as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi, (0x7u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 18; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rdi as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add(0x30u8)) as u8 as u64)); rax = (rax as u32 as i32 as i64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(r10), ((rcx as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rcx) as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r10))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r10), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r9.wrapping_add(r10), ((rcx as u8)) as u8); r9 = ((r9).wrapping_sub(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 9; continue; }
            9 => { r10 = (crate::simd_ops::ru64(rbp)) as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsi, ((r8 as u32)) as u32); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rbx as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x6u64), ((rbx as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x5u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x4u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x3u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10), ((r8 as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi, (0x7u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 18; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rbx as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add(0x30u8)) as u8 as u64)); rax = (rax as u32 as i32 as i64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(r10), ((rcx as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 13; continue; }
            13 => { rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rcx) as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(r10))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(r10), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r9.wrapping_add(r10), ((rcx as u8)) as u8); r9 = ((r9).wrapping_sub(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 17; continue; }
            17 => { rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de09d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsi = (r9) as u64; rdi = (rcx) as u64; rbx = (r8) as u64; rcx = (0xd6bf94d5e57a42bdu64) as u64; rax = (rcx) as u64; let prod = (rax as u128).wrapping_mul((rdi) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (rcx) as u64; r11 = (rdx) as u64; r11 = ((r11).wrapping_shr(23u32)) as u64; let prod = (rax as u128).wrapping_mul((r11) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r10 = ((((r11 as u32)) as i32).wrapping_mul((0x989680u32) as i32) as u32) as u32 as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(23u32)) as u64; rax = ((((r8 as u32)) as i32).wrapping_mul((0x989680u32) as i32) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r10 = (crate::simd_ops::ru64(rbx)) as u64; r11 = (((r11 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10), ((r8 as u8)) as u8); r8 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rax = ((r8.wrapping_add(0x3u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r11 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r11 = (r11 & !0xffu64) | (((((r11 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r11 = (r11 & !0xffu64) | (((((r11 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r10).wrapping_add(0x9u64), ((r11 as u8)) as u8); r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x5u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; r11 = (r10) as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x4u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x3u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10), ((r8 as u8)) as u8); r8 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rax = ((r8.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((rdi as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r10).wrapping_add(0xdu64), ((rdi as u8)) as u8); r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x5u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r9)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x4u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r8 = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r8)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r10).wrapping_add(0x3u64), ((r9 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; r10 = ((rdx as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rcx = ((rax.wrapping_add(r10)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_sub((rcx as u8))) as u8 as u64)); r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(r11).wrapping_add(0x2u64), ((r8 as u8)) as u8); rax = (0xcccccccdu32) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r10 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rax = (0xcccccccdu32) as u32 as u64; r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(3u32)) as u32 as u64; let prod = (rax as u32 as u64).wrapping_mul(((r9 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rcx = (((r9 as u8)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(2u32)) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rax as u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); r8 = ((rcx.wrapping_add(r9)) as u32) as u32 as u64; rcx = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add((r8 as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub((rdx as u8))) as u8 as u64)); r10 = (r10 & !0xffu64) | (((((r10 as u8)).wrapping_sub((r8 as u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); r10 = (r10 & !0xffu64) | (((((r10 as u8)).wrapping_add(0x30u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(r11).wrapping_add(0x1u64), ((r10 as u8)) as u8); rax = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rax.wrapping_add(r11), ((r9 as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u32(rsi, (0x7u32) as u32)) as u32 as u64; fl_r = fl_l; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de0de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = ((rdx as u32)) as u32 as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r10 = (0u32.wrapping_sub((r10 as u32))) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; r14 = (r9) as u64; r8 = ((rdx as u32)) as u32 as u64; rax = (rcx) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r10 = (((r10 as u32)).wrapping_sub(1)) as u32 as u64; r9 = (rax) as u64; rcx = ((r10 as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_shr(((rcx as u32) & 63))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r9.wrapping_add(0x30u64)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((r8 as u8)) as u8); rcx = ((r10 as u32)) as u32 as u64; crate::simd_ops::lk_add_u32(rbx, (1) as u32); rdx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 41; continue; } bb = 5; continue; }
            5 => { rcx = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax).wrapping_shr(((rcx as u32) & 63))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::lk_add_u8(rax.wrapping_add(rdx).wrapping_sub(0x1u64), (1) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 39; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rcx.wrapping_add(rdx), (0x30u8) as u8); crate::simd_ops::lk_add_u8(rcx.wrapping_add(rdx).wrapping_sub(0x1u64), (1) as u8); rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 39; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); r10 = (r11) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r12) as u64); r12 = (0xffffffc0u32) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (r11) as u64; bb = 20; continue; }
            15 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r10 = (rax) as u64; rax = (r11) as u64; bb = 20; continue; }
            17 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; fl_l = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            19 => { r10 = (rax) as u64; rcx = (0x40u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r10 = ((r10).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rax).wrapping_shr(((rcx as u32) & 63))) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (0x80u32) as u32 as u64; rsi = ((r11 as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 29; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r13) as u64); r13 = (0x500000000u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (r15) as u64); r15 = (0xffffffff00000000u64) as u64; bb = 22; continue; }
            22 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = ((r10 as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; r10 = ((r10).wrapping_shr(32u32)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; r9 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rcx = (r8) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; r9 = ((r9).wrapping_add(rcx)) as u64; rcx = ((r8 as u32)) as u32 as u64; r10 = (r9) as u64; r9 = ((r9).wrapping_shr(32u32)) as u64; r10 = ((r10).wrapping_shl(32u32)) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax) as i64).wrapping_mul((r13) as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; rcx = (rdx) as u64; rcx = (((rcx) & (r15))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; r15 = (rax) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = ((rdi.wrapping_sub(0x40u64)) as u32) as u32 as u64; rax = ((rax).wrapping_shr(((rcx as u32) & 63))) as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (r15) as u64; rdx = (r9) as u64; rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 27; continue; }
            26 => { rcx = ((rdi as u32)) as u32 as u64; r8 = (r10) as u64; r8 = ((r8).wrapping_shr(((rcx as u32) & 63))) as u64; r9 = (r15) as u64; rdx = (r8) as u64; rax = (r11) as u64; rdx = ((rdx).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((r12 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(((rcx as u32) & 63))) as u64; r10 = ((r10).wrapping_sub(rdx)) as u64; fl_l = (((r9 as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 27; continue; }
            27 => { rdx = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_add(0x30u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(r14)) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r15 = (0xffffffff00000000u64) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((r9 as u8)) as u8); crate::simd_ops::lk_add_u32(rbx, (1) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 28; continue; }
            28 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = (((rcx as u32)).wrapping_add(0xffffffc0u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 32; continue; }
            31 => { rax = (r10) as u64; bb = 32; continue; }
            32 => { rax = ((rax).wrapping_shr(((rcx as u32) & 63))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 33; continue; }
            33 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = (((crate::simd_ops::ru32(rbx)) as i32 as i64) as u64) as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu8(rax, (0x31u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rax, (0x1u32) as u32); crate::simd_ops::wu32(rbx, (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rdx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::lk_add_u8(rax.wrapping_add(rdx).wrapping_sub(0x1u64), (1) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu8(rcx.wrapping_add(rdx), (0x30u8) as u8); crate::simd_ops::lk_add_u8(rcx.wrapping_add(rdx).wrapping_sub(0x1u64), (1) as u8); rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rdx, (0x31u8) as u8); fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 41; continue; }
            41 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de1150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6dac; const P1: u64 = 0x7001ddef30; const P2: u64 = 0x7001dc4000; const P3: u64 = 0x7001dc7760; const P4: u64 = 0x7001df6d58; const P5: u64 = 0x7001dc7cd0; const P6: u64 = 0x7001de14c9;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = (r8) as u64; rsi = ((rdx as u32)) as u32 as u64; r14 = (rcx) as u64; xmm1 = (xmm0) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm1 = crate::simd_ops::vpsrldq(xmm1, 0x8u8); r9 = ((rbx as u32)) as u32 as u64; r8 = ((xmm1 as u32)) as u32 as u64; r10 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; rdi = (0x1999999999999998u64) as u64; bb = 2; continue; }
            2 => { fl_l = (r9) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rdx.wrapping_add(r11))) as i8 as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r9 = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r9) as u64); r8 = ((rbx as u32)) as u32 as u64; bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rdx = ((r9.wrapping_add(0x1u64)) as u64) as u64; rcx = ((((r10 as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x35u8) as u8 as u64; rdx = (if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { r9 } else { rdx }) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rdx) as u64); bb = 7; continue; }
            7 => { rsi = (((rsi as u32)).wrapping_add((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rbx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; fl_l = 0; fl_r = ((r8 as u32)) as u32 as u64; r8 = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); r12 = (0xffc0000000000000u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rdi = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rdi = ((((rdi as u32)) & (0x4u32))) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; fl_l = (r12) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_shl(10u32)) as u64; rcx = (((rcx as u32)).wrapping_sub(0xau32)) as u32 as u64; fl_l = (r12) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if ((((fl_l as u64)) as i64) >= 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rcx as u32)) as u32); rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rdi = ((rdi).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r15, (rbx) as u64); bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            14 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { r8 = ((rbp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rbx) as u64); rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), ((rbx as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 16; continue; }
            16 => { rsi = (((rsi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rsi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 53; continue; } bb = 17; continue; }
            17 => { rdx = ((P2) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1d4ccu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            18 => { rax = (0xa000000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffc4u32) as u32); bb = 25; continue; }
            19 => { rax = (0xc800000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffc7u32) as u32); bb = 25; continue; }
            20 => { rax = (0xfa00000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffcau32) as u32); bb = 25; continue; }
            21 => { rax = (0x9c40000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffceu32) as u32); bb = 25; continue; }
            22 => { rax = (0xc350000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffd1u32) as u32); bb = 25; continue; }
            23 => { rax = (0xf424000000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffd4u32) as u32); bb = 25; continue; }
            24 => { rax = (0x9896800000000000u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xffffffd8u32) as u32); bb = 25; continue; }
            25 => { rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x13u32) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(r14.wrapping_add(0x8u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((rdi).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; rax = (rbx) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rax).wrapping_add(0x8u64)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx).wrapping_shl(10u32)) as u64; r8 = (((r8 as u32)).wrapping_sub(0xau32)) as u32 as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 29; continue; }
            29 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) as i64) >= 0) { bb = 30; continue; } bb = 31; continue; }
            31 => { rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x40u64)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffc03u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 33; continue; } bb = 32; continue; }
            32 => { r9 = (0xbu32) as u32 as u64; bb = 37; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffbceu32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 34; continue; }
            34 => { r9 = (0x40u32) as u32 as u64; bb = 36; continue; }
            35 => { r9 = (0xfffffc0eu32) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((r9.wrapping_add(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x40u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = ((r9.wrapping_sub(0x3cu64)) as u32) as u32 as u64; r9 = (0x3cu32) as u32 as u64; rdx = ((rdx).wrapping_shr(((rcx as u32) & 63))) as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdi = ((((rdi) as i64).wrapping_shr(((rcx as u32) & 63)) as u64)) as u64; fl_l = ((rdi).wrapping_add(0x9u64)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 37; continue; }
            37 => { rcx = ((r9 as u32)) as u32 as u64; rax = ((r8.wrapping_add(r9)) as u32) as u32 as u64; r10 = (0x1u32) as u32 as u64; r11 = ((r10 as u32)) as u32 as u64; r11 = ((r11).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = ((r10).wrapping_shl(((rcx as u32) & 63))) as u64; r11 = ((r11).wrapping_sub(1)) as u64; r11 = (((r11) & (rdx))) as u64; r10 = ((r10).wrapping_shl(3u32)) as u64; rcx = ((r9 as u32)) as u32 as u64; r11 = ((r11).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_shr(((rcx as u32) & 63))) as u64; rsi = ((r10.wrapping_add(rdi)) as u64) as u64; fl_l = (r11) as u64 as u64; fl_r = (rsi) as u64 as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rcx }) as u64; rdx = (0x1fffffffffffffu64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 40; continue; } bb = 38; continue; }
            38 => { bb = 39; continue; }
            39 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 39; continue; } bb = 40; continue; }
            40 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3ccu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 42; continue; } bb = 41; continue; }
            41 => { rbx = (0x7ff0000000000000u64) as u64; bb = 50; continue; }
            42 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffbceu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 50; continue; } bb = 43; continue; }
            43 => { if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 46; continue; } bb = 44; continue; }
            44 => { fl_l = (((rcx) >> (52u32)) & 1) as u64; if ((fl_l as u64) != 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffbceu32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 44; continue; } bb = 46; continue; }
            46 => { if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { fl_l = (((rcx) >> (52u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = (((rax as u32)).wrapping_add(0x433u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; bb = 49; continue; }
            49 => { rax = (0xfffffffffffffu64) as u64; rbx = ((rbx).wrapping_shl(52u32)) as u64; rcx = (((rcx) & (rax))) as u64; fl_l = (((rbx) | (rcx))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 50; continue; }
            50 => { r10 = ((r10).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(r15, (rbx) as u64); fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 51; continue; }
            51 => { fl_l = (r11) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 52; continue; }
            52 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            53 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x7001de1150u64) { 0x46 => { bb = 1; continue; } 0x56 => { bb = 2; continue; } 0x5b => { bb = 3; continue; } 0x79 => { bb = 4; continue; } 0x82 => { bb = 5; continue; } 0x8b => { bb = 6; continue; } 0xa5 => { bb = 7; continue; } 0xd2 => { bb = 8; continue; } 0xde => { bb = 9; continue; } 0xe3 => { bb = 10; continue; } 0xea => { bb = 11; continue; } 0xfe => { bb = 12; continue; } 0x101 => { bb = 13; continue; } 0x103 => { bb = 14; continue; } 0x11c => { bb = 15; continue; } 0x13d => { bb = 16; continue; } 0x14b => { bb = 17; continue; } 0x160 => { bb = 18; continue; } 0x173 => { bb = 19; continue; } 0x186 => { bb = 20; continue; } 0x199 => { bb = 21; continue; } 0x1ac => { bb = 22; continue; } 0x1bf => { bb = 23; continue; } 0x1d2 => { bb = 24; continue; } 0x1e3 => { bb = 25; continue; } 0x201 => { bb = 26; continue; } 0x205 => { bb = 27; continue; } 0x231 => { bb = 28; continue; } 0x23e => { bb = 29; continue; } 0x243 => { bb = 30; continue; } 0x24b => { bb = 31; continue; } 0x25f => { bb = 32; continue; } 0x267 => { bb = 33; continue; } 0x26e => { bb = 34; continue; } 0x276 => { bb = 35; continue; } 0x288 => { bb = 36; continue; } 0x29f => { bb = 37; continue; } 0x2eb => { bb = 38; continue; } 0x2f0 => { bb = 39; continue; } 0x2fa => { bb = 40; continue; } 0x301 => { bb = 41; continue; } 0x30d => { bb = 42; continue; } 0x314 => { bb = 43; continue; } 0x316 => { bb = 44; continue; } 0x31d => { bb = 45; continue; } 0x329 => { bb = 46; continue; } 0x32b => { bb = 47; continue; } 0x332 => { bb = 48; continue; } 0x33a => { bb = 49; continue; } 0x34e => { bb = 50; continue; } 0x35d => { bb = 51; continue; } 0x366 => { bb = 52; continue; } 0x36d => { bb = 53; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de15d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7450;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r9 = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbp = (r8) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x17u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rbx = ((xmm0 as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; r10 = ((rdx as u32)) as u32 as u64; r8 = (0x1999999999999998u64) as u64; bb = 5; continue; }
            5 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(r10.wrapping_add(rdi))) as i8 as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rdx) as i64) as f64).to_bits() as u128); bb = 10; continue; }
            9 => { rax = (rdx) as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 10; continue; }
            10 => { rax = (r9) as u64; rcx = ((P0) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(crate::simd_ops::ru64(rcx))).to_bits() as u128); crate::simd_ops::wu64(rbp, ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x17u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rbx = ((xmm0 as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; r10 = ((rdx as u32)) as u32 as u64; r8 = (0x1999999999999998u64) as u64; bb = 14; continue; }
            14 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (((crate::simd_ops::ru8(r10.wrapping_add(rdi))) as i8 as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 16; continue; }
            16 => { xmm0 = 0u128; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rdx) as i64) as f64).to_bits() as u128); bb = 19; continue; }
            18 => { rax = (rdx) as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 19; continue; }
            19 => { rcx = ((P0) as u64) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8))))).to_bits() as u128); crate::simd_ops::wu64(rbp, ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rbx = (0xfu32) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 31; continue; } bb = 21; continue; }
            21 => { r9 = (((r9 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x17u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rdi = ((xmm0 as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); r10 = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp)) as u64; r8 = (0x1999999999999998u64) as u64; bb = 24; continue; }
            24 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (((crate::simd_ops::ru8(r10.wrapping_add(rsi))) as i8 as i32) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 27; continue; }
            27 => { xmm0 = 0u128; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rdx) as i64) as f64).to_bits() as u128); bb = 30; continue; }
            29 => { rax = (rdx) as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 30; continue; }
            30 => { rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; rcx = ((P0) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8))))).to_bits() as u128); rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8))))).to_bits() as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rbp, ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de18c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df6ac0; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (0x7ff0000000000000u64) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0)) { rax = (0xfffffffffffffu64) as u64; rcx = (rdx) as u64; rcx = (((rcx) & (rax))) as u64; rax = (0x10000000000000u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; rax = ((rcx.wrapping_add(rax)) as u64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; } rax = ((rdx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rax) as u64);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de1950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001dc55b0; const P2: u64 = 0x7001df6ac0; const P3: u64 = 0x7001de15d0; const P4: u64 = 0x7001de1150; const P5: u64 = 0x7001dc6b60; const P6: u64 = 0x7001dc5be0; const P7: u64 = 0x7001dc78f0; const P8: u64 = 0x7001dc6090; const P9: u64 = 0x7001dc5630; const P10: u64 = 0x7001dc5540; const P11: u64 = 0x7001de18c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 185>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x4c0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x5c0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x470u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rdx = ((xmm0 as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (r8) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (r9) as u64; rcx = ((r9 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x5e0u64), (rsi) as u64); rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { xmm0 = 0u128; bb = 46; continue; }
            10 => { rax = (rax as u32 as i32 as i64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = ((rdx as u32)) as u32 as u64; bb = 5; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; r10 = ((((rcx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r10) as u64); rsi = ((rdx.wrapping_add(r11)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x30cu32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = ((r8.wrapping_add(0x30au64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = ((rbp.wrapping_add(0x46au64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = ((rbp.wrapping_add(0x160u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 16; continue; }
            16 => { rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r8))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); fl_l = (r9) as u64 as u64; fl_r = (0x30bu64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 16; continue; } bb = 17; continue; }
            17 => { bb = 21; continue; }
            18 => { rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; rax = (0x6u32) as u32 as u64; bb = 19; continue; }
            19 => { rcx = ((rcx.wrapping_add(0x80u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; r8 = ((r8.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu128(rcx.wrapping_sub(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm1) as u128); fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = ((crate::simd_ops::ru16(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0xau64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0xau64), ((rax as u8)) as u8); bb = 21; continue; }
            21 => { rdx = ((r10.wrapping_sub(0x30cu64)) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x46bu64), (0x31u8) as u8); rax = ((rbp.wrapping_add(0x160u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x30cu64) as u64); rdx = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 46; continue; }
            22 => { rax = ((rcx.wrapping_add(rsi)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x135u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; bb = 46; continue; }
            24 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffebcu32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu128(rsp.wrapping_add(0x580u64), (xmm7) as u128); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 47; continue; } bb = 26; continue; }
            26 => { r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); rdx = ((rsi as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 47; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu128(rsp.wrapping_add(0x590u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; fl_l = ((xmm6 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P2)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 29; continue; } bb = 28; continue; }
            28 => { if (f64::from_bits(fl_l) == f64::from_bits(fl_r) || f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 44; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rsp.wrapping_add(0x5d0u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x5b8u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x5b0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x5a8u64), (r14) as u64); r14 = (0x7ff0000000000000u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x5a0u64), (r15) as u64); fl_l = (((rdx) & (r14))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdi = (0xfffffbcdu32) as u32 as u64; bb = 32; continue; }
            31 => { rdi = (rax) as u64; rdi = ((rdi).wrapping_shr(52u32)) as u64; rdi = ((((rdi as u32)) & (0x7ffu32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x434u32) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 32; continue; }
            32 => { r15 = (0xfffffffffffffu64) as u64; r12 = (0x10000000000000u64) as u64; rax = (((rax) & (r15))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; rcx = ((rax.wrapping_add(r12)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rbx = ((rcx.wrapping_mul(2).wrapping_add(0x1u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rdx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rbp.wrapping_add(0x160u64)) as u64) as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { rax } else { rcx }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { rdx } else { rcx }) as u64; rdx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x160u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x5b8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x5d0u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 43; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 41; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); xmm6 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 43; continue; }
            36 => { fl_l = (((rax) >> (63u32)) & 1) as u64; if ((fl_l as u64) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { rdx = (rax) as u64; rdx = (((rdx) & (r15))) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; rcx = ((rdx.wrapping_add(r12)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); xmm6 = (xmm0 as u64) as u128; bb = 43; continue; }
            39 => { fl_l = ((rax).wrapping_sub(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); xmm6 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 43; continue; }
            40 => { fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); xmm6 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 43; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (xmm0) as u128; bb = 43; continue; }
            43 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x5a8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x5b0u64))) as u64; bb = 44; continue; }
            44 => { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x590u64))) as u128; bb = 45; continue; }
            45 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x580u64))) as u128; bb = 46; continue; }
            46 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x5e0u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x470u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x5c0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            47 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; bb = 45; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de1ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x7001e0a060; const P4: u64 = 0x7001deded0; const P5: u64 = 0x7001de1f6d;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 6; continue; }
            2 => { rcx = (0x1u32) as u32 as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rcx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rbx, fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx, (0x2u8) as u8); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 6; continue; }
            6 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 10; continue; }
            10 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de1f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df7530; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de1fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7530; const P1: u64 = 0x7001dc5b10; const P2: u64 = 0x8; const P3: u64 = 0x7001dc7360; const P4: u64 = 0x10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x7001e074c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); } rax = ((rbx.wrapping_add(P0)) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de20c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e074c8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e074c8; const P1: u64 = 0x7001de2117; const P2: u64 = 0x7001de2118; const P3: u64 = 0x7001de2119; const P4: u64 = 0x7001de211a; const P5: u64 = 0x7001de211b; const P6: u64 = 0x7001de211c; const P7: u64 = 0x7001de211d; const P8: u64 = 0x7001de211e; const P9: u64 = 0x7001de211f; const P10: u64 = 0x7001de2127; const P11: u64 = 0x7001de2128; const P12: u64 = 0x7001de2129; const P13: u64 = 0x7001de212a; const P14: u64 = 0x7001de212b; const P15: u64 = 0x7001de212c; const P16: u64 = 0x7001de212d; const P17: u64 = 0x7001de212e; const P18: u64 = 0x7001de212f;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            10 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            17 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            18 => { crate::lift_ops::trap(0x80000003, P17, rcx, 0); return rax; }
            19 => { crate::lift_ops::trap(0x80000003, P18, rcx, 0); return rax; }
            20 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e074b0; const P1: u64 = 0x7001e0a628;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            2 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x7001de2140u64) { 0xc => { bb = 1; continue; } 0xf => { bb = 2; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e074a0; const P1: u64 = 0x7001e0a610;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            2 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x7001de2160u64) { 0xc => { bb = 1; continue; } 0xf => { bb = 2; continue; } _ => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc6310;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    r10 = (r9) as u64;
    r11 = (r8) as u64;
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = (rdx) as u64;
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64;
    r8 = (r10) as u64;
    rdx = (r11) as u64;
    r9 = (if (((fl_l as u32)) != ((fl_r as u32))) { crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)) } else { (r9 as u32) }) as u32 as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc53e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (rdx) as u64;
    rdx = (r8) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc5430; const P1: u64 = 0x7001de222b; const P2: u64 = 0x7001de222c; const P3: u64 = 0x7001de222d; const P4: u64 = 0x7001de222e; const P5: u64 = 0x7001de222f; const P6: u64 = 0x7001dc7dc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (rdx) as u64; rdx = (r8) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            6 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e074d8; const P1: u64 = 0x7001e074a0; const P2: u64 = 0x7001e074b0; const P3: u64 = 0x7001e074f0; const P4: u64 = 0x7001e074c8; const P5: u64 = 0x7001e074c0; const P6: u64 = 0x7001e074d0; const P7: u64 = 0x7001e074e0; const P8: u64 = 0x7001e074e8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(P0, (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(P2, (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(P3, (rax) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(P4, (rax) as u64);
    crate::simd_ops::wu64(P5, (rcx) as u64);
    crate::simd_ops::wu64(P6, (rdx) as u64);
    crate::simd_ops::wu64(P7, (r8) as u64);
    crate::simd_ops::wu64(P8, (r9) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de22d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc6630;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64;
    r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rcx = ((rdx.wrapping_add(r9)) as u64) as u64;
    rdx = (r8) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc57d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (r8) as u64;
    rcx = (rdx) as u64;
    rdx = (rax) as u64;
    r8 = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc7e10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (rdx) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de25a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de25b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (((r8) as i64).wrapping_mul((crate::simd_ops::ru64(rcx)) as i64) as u64) as u64;
    rax = (rdx) as u64;
    crate::simd_ops::wu64(rdx, (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r8)) as u64 as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    rax = (rdx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = (rdx) as u64;
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;
    if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }
    crate::simd_ops::wu64(r9, (rax) as u64);
    rax = (r9) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (rdx) as u64;
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    let d = (crate::simd_ops::ru64(r8)) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;
    if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de27a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de27e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (((rdx) as i64).wrapping_mul((crate::simd_ops::ru64(rcx)) as i64) as u64) as u64;
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (rax) as u64)) as u64; fl_r = fl_l;
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de29b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64;
    crate::simd_ops::wu64(rcx, ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de29c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (rdx) as u64;
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;
    if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de29e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdx = (0x141dd76000u64) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df75f0; const P1: u64 = 0x7001df75e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64;
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    rdx = (rdx & !0xffu64) | (((if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64;
    rax = (rax & !0xffu64) | (((if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; } rax = (0x8000000000000000u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r8 } else { rax }) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdx = (0xd693a400u32) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((rdx) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdx = (((rax) as i64).wrapping_mul((0x3938700u64) as i64) as u64) as u64;
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (0x20c49ba5e353f7cfu64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rdx).wrapping_add(rax)) as u64 as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (((rdx) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2e91; const P1: u64 = 0x7001de2e92; const P2: u64 = 0x7001de2e93; const P3: u64 = 0x7001de2e94; const P4: u64 = 0x7001de2e95; const P5: u64 = 0x7001de2e96; const P6: u64 = 0x7001de2e97; const P7: u64 = 0x7001de2e98; const P8: u64 = 0x7001de2e99; const P9: u64 = 0x7001de2e9a; const P10: u64 = 0x7001de2e9b; const P11: u64 = 0x7001de2e9c; const P12: u64 = 0x7001de2e9d; const P13: u64 = 0x7001de2e9e; const P14: u64 = 0x7001de2e9f; const P15: u64 = 0x7001e0a078; const P16: u64 = 0x7001de2ea7; const P17: u64 = 0x7001de2ea8; const P18: u64 = 0x7001de2ea9; const P19: u64 = 0x7001de2eaa; const P20: u64 = 0x7001de2eab; const P21: u64 = 0x7001de2eac; const P22: u64 = 0x7001de2ead; const P23: u64 = 0x7001de2eae; const P24: u64 = 0x7001de2eaf; const P25: u64 = 0x7001e0a070;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (0x8000000000000000u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P0, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            16 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P15)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            18 => { crate::lift_ops::trap(0x80000003, P17, rcx, 0); return rax; }
            19 => { crate::lift_ops::trap(0x80000003, P18, rcx, 0); return rax; }
            20 => { crate::lift_ops::trap(0x80000003, P19, rcx, 0); return rax; }
            21 => { crate::lift_ops::trap(0x80000003, P20, rcx, 0); return rax; }
            22 => { crate::lift_ops::trap(0x80000003, P21, rcx, 0); return rax; }
            23 => { crate::lift_ops::trap(0x80000003, P22, rcx, 0); return rax; }
            24 => { crate::lift_ops::trap(0x80000003, P23, rcx, 0); return rax; }
            25 => { crate::lift_ops::trap(0x80000003, P24, rcx, 0); return rax; }
            26 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P25)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df75b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    xmm1 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rcx)) as i64) as f64).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rdx)) as i64) as f64).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P0))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    xmm1 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rcx)) as i64) as f64).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rdx)) as i64) as f64).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = (0xffffffffu32) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de2f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc6ae0; const P1: u64 = 0x7001e0a080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rcx) as u64;
    rbx = (rdx) as u64;
    rcx = (r8) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = ((rax as u32)) as u32 as u64;
    rdx = (rbx) as u64;
    rcx = (rdi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de3063; const P1: u64 = 0x7001de3064; const P2: u64 = 0x7001de3065; const P3: u64 = 0x7001de3066; const P4: u64 = 0x7001de3067; const P5: u64 = 0x7001de3068; const P6: u64 = 0x7001de3069; const P7: u64 = 0x7001de306a; const P8: u64 = 0x7001de306b; const P9: u64 = 0x7001de306c; const P10: u64 = 0x7001de306d; const P11: u64 = 0x7001de306e; const P12: u64 = 0x7001de306f; const P13: u64 = 0x7001e0a0d8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P0, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            14 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de3083; const P1: u64 = 0x7001de3084; const P2: u64 = 0x7001de3085; const P3: u64 = 0x7001de3086; const P4: u64 = 0x7001de3087; const P5: u64 = 0x7001de3088; const P6: u64 = 0x7001de3089; const P7: u64 = 0x7001de308a; const P8: u64 = 0x7001de308b; const P9: u64 = 0x7001de308c; const P10: u64 = 0x7001de308d; const P11: u64 = 0x7001de308e; const P12: u64 = 0x7001de308f; const P13: u64 = 0x7001e0a098; const P14: u64 = 0x7001de3097; const P15: u64 = 0x7001de3098; const P16: u64 = 0x7001de3099; const P17: u64 = 0x7001de309a; const P18: u64 = 0x7001de309b; const P19: u64 = 0x7001de309c; const P20: u64 = 0x7001de309d; const P21: u64 = 0x7001de309e; const P22: u64 = 0x7001de309f; const P23: u64 = 0x7001e0a0c0; const P24: u64 = 0x7001de30a7; const P25: u64 = 0x7001de30a8; const P26: u64 = 0x7001de30a9; const P27: u64 = 0x7001de30aa; const P28: u64 = 0x7001de30ab; const P29: u64 = 0x7001de30ac; const P30: u64 = 0x7001de30ad; const P31: u64 = 0x7001de30ae; const P32: u64 = 0x7001de30af; const P33: u64 = 0x7001de30b7; const P34: u64 = 0x7001de30b8; const P35: u64 = 0x7001de30b9; const P36: u64 = 0x7001de30ba; const P37: u64 = 0x7001de30bb; const P38: u64 = 0x7001de30bc; const P39: u64 = 0x7001de30bd; const P40: u64 = 0x7001de30be; const P41: u64 = 0x7001de30bf; const P42: u64 = 0x7001e0a0a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P0, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            14 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            17 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            18 => { crate::lift_ops::trap(0x80000003, P17, rcx, 0); return rax; }
            19 => { crate::lift_ops::trap(0x80000003, P18, rcx, 0); return rax; }
            20 => { crate::lift_ops::trap(0x80000003, P19, rcx, 0); return rax; }
            21 => { crate::lift_ops::trap(0x80000003, P20, rcx, 0); return rax; }
            22 => { crate::lift_ops::trap(0x80000003, P21, rcx, 0); return rax; }
            23 => { crate::lift_ops::trap(0x80000003, P22, rcx, 0); return rax; }
            24 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P23)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { crate::lift_ops::trap(0x80000003, P24, rcx, 0); return rax; }
            26 => { crate::lift_ops::trap(0x80000003, P25, rcx, 0); return rax; }
            27 => { crate::lift_ops::trap(0x80000003, P26, rcx, 0); return rax; }
            28 => { crate::lift_ops::trap(0x80000003, P27, rcx, 0); return rax; }
            29 => { crate::lift_ops::trap(0x80000003, P28, rcx, 0); return rax; }
            30 => { crate::lift_ops::trap(0x80000003, P29, rcx, 0); return rax; }
            31 => { crate::lift_ops::trap(0x80000003, P30, rcx, 0); return rax; }
            32 => { crate::lift_ops::trap(0x80000003, P31, rcx, 0); return rax; }
            33 => { crate::lift_ops::trap(0x80000003, P32, rcx, 0); return rax; }
            34 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            35 => { crate::lift_ops::trap(0x80000003, P33, rcx, 0); return rax; }
            36 => { crate::lift_ops::trap(0x80000003, P34, rcx, 0); return rax; }
            37 => { crate::lift_ops::trap(0x80000003, P35, rcx, 0); return rax; }
            38 => { crate::lift_ops::trap(0x80000003, P36, rcx, 0); return rax; }
            39 => { crate::lift_ops::trap(0x80000003, P37, rcx, 0); return rax; }
            40 => { crate::lift_ops::trap(0x80000003, P38, rcx, 0); return rax; }
            41 => { crate::lift_ops::trap(0x80000003, P39, rcx, 0); return rax; }
            42 => { crate::lift_ops::trap(0x80000003, P40, rcx, 0); return rax; }
            43 => { crate::lift_ops::trap(0x80000003, P41, rcx, 0); return rax; }
            44 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P42)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a0f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    r9 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    r8 = (0x7fffffffu32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rbx, (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de31c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a0e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de31d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a0e8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    rdx = (0x1u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a0f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (0xffffffffu32) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc67b0; const P1: u64 = 0x7001dc6500; const P2: u64 = 0x7001dc6ae0; const P3: u64 = 0x7001e0a0f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; rbx = (rdx) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (0xffffffffu32) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0xfffffffeu32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x102u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a450; const P1: u64 = 0x7001df7610; const P2: u64 = 0x7001df75c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { xmm1 = 0u128; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
    rdx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { xmm1 = ((crate::simd_ops::ru64(P1)) as u64) as u128; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128);
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de33a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rdx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx, (0x0u64) as u64); rax = (rcx) as u64; return rax; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (rdx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (r8) as u64; return rax; }
            6 => { rcx = (rdx) as u64; rax = (0xffffffff00000000u64) as u64; rcx = (((rcx) & (rax))) as u64; rax = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (0xcccccccccccccccdu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (0x295e9648864000u64) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (r8) as u64; crate::simd_ops::wu64(r8, (rdx) as u64); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df7610; const P1: u64 = 0x7001df75c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((xmm1 as u64)) as u64; fl_r = (crate::simd_ops::ru64(P0)) as u64;
    if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; return rax; }
    if !((f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan())) { rax = (0x7fffffffffffffffu64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (rcx) as u64; return rax; }
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07770; const P1: u64 = 0x7001e07524; const P2: u64 = 0x7001e07508; const P3: u64 = 0x7001dee2a0; const P4: u64 = 0x7001e07090; const P5: u64 = 0x7001e0a128; const P6: u64 = 0x7001e07510; const P7: u64 = 0x7001dc5e80; const P8: u64 = 0x7001e07518; const P9: u64 = 0x7001dc8070; const P10: u64 = 0x7001dee220;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rdx = (0x104u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P2) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(P2, (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P2) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a130; const P1: u64 = 0x7001dc7a50; const P2: u64 = 0x7001df75d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (rdx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rbx = (rcx) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm2 = (xmm0) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 1; continue; }
            1 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); bb = 6; continue; }
            5 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (0x7fffffffu32) as u32 as u64; return rax; }
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    rcx = (0x141dd76000u64) as u64;
    let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128;
    if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (0x7fffffffu32) as u32 as u64; return rax; }
    rax = (0x98b5bf2c03e529c5u64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((rdx).wrapping_add(r8)) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(31u32) as u64)) as u64;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de36c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de36e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (0x20c49ba5e353f7cfu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = ((rcx).wrapping_add(0x3e7u64)) as u64; rax = (0x20c49ba5e353f7cfu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de37b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (0x7fffffffu32) as u32 as u64; return rax; }
    rax = (0x8f2a633943a6d729u64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((rdx).wrapping_add(r8)) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(25u32) as u64)) as u64;
    rax = (rdx) as u64;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (((rdx) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (0x431bde82d7b634dbu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(18u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de38a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de38e0; const P1: u64 = 0x7001e07090;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (rbx) as u64;
    crate::simd_ops::wu64(rbx, (rdx) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de38e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001e0a108; const P2: u64 = 0x7001dc7bf0; const P3: u64 = 0x7001de3df0; const P4: u64 = 0x7001de3ea0; const P5: u64 = 0x7001e07528; const P6: u64 = 0x7001e07090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((P3) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x8bu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((P4) as u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P5, (rax) as u64); crate::simd_ops::wu64(P6, (rcx) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de39a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07090; const P1: u64 = 0x7001de38a0; const P2: u64 = 0x7001de38e0; const P3: u64 = 0x7001de3df0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = ((P1) as u64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(P0)) as u64; }
    rcx = ((P3) as u64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001e07770; const P2: u64 = 0x7001e07534; const P3: u64 = 0x7001e07531; const P4: u64 = 0x7001dee2a0; const P5: u64 = 0x7001dc7bf0; const P6: u64 = 0x5b; const P7: u64 = 0x7001dee220;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rax) as u64); rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = (0x104u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x98u64)) as u64; return rax; }
            2 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((P2) as u64) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(P3, ((rdx as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a110; const P1: u64 = 0x7001e0a130; const P2: u64 = 0x7001dc7a50; const P3: u64 = 0x7001df75d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = 0u128; xmm2 = (xmm0) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 1; continue; }
            1 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); bb = 6; continue; }
            5 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P3))).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de3480; const P1: u64 = 0x7001dc5060; const P2: u64 = 0x7001e07090; const P3: u64 = 0x7001e0a128; const P4: u64 = 0x7001dc6500; const P5: u64 = 0x7001dc5350;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rcx = ((rax.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x7fffffffffffffffu64) as u64; bb = 7; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (0xcccccccccccccccdu64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (0x295e9648864000u64) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (rbx) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x3938700u64) as u64 as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(r14, (rax) as u64); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rsi, (rbx) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(r14, (rdx) as u64); bb = 11; continue; }
            11 => { rcx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (r14) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de3480; const P1: u64 = 0x10; const P2: u64 = 0x7001dc5060; const P3: u64 = 0x7001e07090; const P4: u64 = 0x7001e0a128; const P5: u64 = 0x8; const P6: u64 = 0x7001dc5350;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rcx = ((rax.wrapping_add(P1)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsi, (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rdx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (rdx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (0x7fffffffffffffffu64) as u64; bb = 7; continue; }
            6 => { rax = ((rdx as u32)) as u32 as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (0xcccccccccccccccdu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rax = (0x295e9648864000u64) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P5), (rdx) as u64); crate::simd_ops::wu64(rdi, (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rdi) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a100; const P1: u64 = 0x7001e07528; const P2: u64 = 0x7001dc6500;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (0x8637bd05af7u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((r10) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (crate::simd_ops::ru64(P1)) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } bb = 3; continue; }
            2 => { r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (r10) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; rcx = (r8) as u64; let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r9 = (rax) as u64; r10 = ((r10).wrapping_sub(rcx)) as u64; rax = (((r10) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; rcx = (((r9) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (r8) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx, (rax) as u64); rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07504; const P1: u64 = 0x7001e07080; const P2: u64 = 0x7001dc6500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = (rcx) as u64; bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = (((rcx as u16)).wrapping_add(1)) as u16 as u64; rcx = (rcx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x32u64), ((rcx as u16)) as u16); bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rbx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(P0, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } rbx = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x32u64))) as u32) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rcx = (((rcx) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de3f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07538; const P1: u64 = 0x7001e0a110; const P2: u64 = 0x7001e0a120; const P3: u64 = 0x7001e0a118; const P4: u64 = 0x7001e07770; const P5: u64 = 0x7001e07554; const P6: u64 = 0x7001e07564; const P7: u64 = 0x7001e0a100; const P8: u64 = 0x7001e0a108; const P9: u64 = 0x7001e07558; const P10: u64 = 0x7001df7600; const P11: u64 = 0x7001e07548; const P12: u64 = 0x7001dee2a0; const P13: u64 = 0x7001dee220;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 1; continue; }
            1 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0x2u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru32(P4)) as u32 as u64; rbp = (0x104u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(rbp))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rbp))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 4; continue; }
            4 => { { let t = crate::emu::guest_rdtsc(); rax = t & 0xffff_ffff; rdx = t >> 32; } rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rax = (((rax) | (rdx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((rdi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm2 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P9)) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); bb = 7; continue; }
            6 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); bb = 7; continue; }
            7 => { xmm1 = 0u128; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as i64) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru64(P10)) as u64) as u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P11)) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); bb = 11; continue; }
            10 => { rax = (rbx) as u64; rbx = ((((rbx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rbx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm2 as u64)).to_bits() as u128); crate::simd_ops::wu64(P0, ((xmm0 as u64)) as u64); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            13 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rcx = ((P5) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 15; continue; }
            15 => { { let t = crate::emu::guest_rdtsc(); rax = t & 0xffff_ffff; rdx = t >> 32; } rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = ((P5) as u64) as u64; rax = (((rax) | (rdx))) as u64; crate::simd_ops::wu64(P11, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            16 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 17; continue; }
            17 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = ((P6) as u64) as u64; crate::simd_ops::wu64(P9, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de41b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rdx, (rax) as u64); rax = (rdx) as u64; return rax; }
    rcx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); rax = (rdx) as u64; return rax; }
    rcx = (0x295e9648864000u64) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (rdx) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32);
    fl_l = ((rcx).wrapping_shr(32u32)) as u64 as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df7610; const P1: u64 = 0x7001df75c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07770; const P1: u64 = 0x7001df7600; const P2: u64 = 0x7001e07538; const P3: u64 = 0x7001e0a110; const P4: u64 = 0x7001e0a120; const P5: u64 = 0x7001e0a118; const P6: u64 = 0x7001e07554; const P7: u64 = 0x7001e07564; const P8: u64 = 0x7001e0a100; const P9: u64 = 0x7001e0a108; const P10: u64 = 0x7001e07558; const P11: u64 = 0x7001e07548; const P12: u64 = 0x7001e0a060; const P13: u64 = 0x7001dee2a0; const P14: u64 = 0x7001dee220;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rbp = (crate::simd_ops::ru32(P0)) as u32 as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = ((crate::exe::tls_pointer() as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(P1)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); r12 = (0x104u32) as u32 as u64; bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 16; continue; } bb = 2; continue; }
            2 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 13; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0x2u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(r14.wrapping_add(rbp.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(r12.wrapping_add(rdi))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(rdi))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 5; continue; }
            5 => { { let t = crate::emu::guest_rdtsc(); rax = t & 0xffff_ffff; rdx = t >> 32; } rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; rax = (((rax) | (rdx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r15) as u64); rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x90u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(P10)) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; xmm1 = 0u128; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rdi) as i64) as f64).to_bits() as u128); bb = 8; continue; }
            7 => { rax = (rdi) as u64; rdi = ((((rdi as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdi))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm7 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P11)) as u64 as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; xmm0 = 0u128; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) < 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); bb = 12; continue; }
            11 => { rax = (rbx) as u64; rbx = ((((rbx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rbx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 12; continue; }
            12 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128); crate::simd_ops::wu64(P2, ((xmm0 as u64)) as u64); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; bb = 13; continue; }
            13 => { if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 16; continue; } bb = 14; continue; }
            14 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (0xau32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 1; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 18; continue; }
            18 => { { let t = crate::emu::guest_rdtsc(); rax = t & 0xffff_ffff; rdx = t >> 32; } rdx = ((rdx).wrapping_shl(32u32)) as u64; rcx = ((P6) as u64) as u64; rax = (((rax) | (rdx))) as u64; crate::simd_ops::wu64(P11, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            19 => { rcx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 20; continue; }
            20 => { rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((P7) as u64) as u64; crate::simd_ops::wu64(P10, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x19; const P2: u64 = 0x10; const P3: u64 = 0x7001de45d0; const P4: u64 = 0x7001de2050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rcx) as u64;
    rsi = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rdx = (0x28u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; rdx = (0x28u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = (0x28u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de45d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x7001de45d0; const P3: u64 = 0x7001de2050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = (r8) as u64;
    rdi = (rdx) as u64;
    rsi = (rcx) as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; rdx = (0x28u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (rdx) as u64; r9 = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = 0; fl_r = (r8) as u64 as u64; r8 = (0u64.wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 13; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = (rcx) as u64; bb = 13; continue; }
            10 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r9, (rax) as u64); return rax; }
            15 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 27; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 21; continue; }
            21 => { rax = (rcx) as u64; bb = 25; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rdx) as u64; rdx = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 25; continue; }
            25 => { r8 = ((r8).wrapping_sub(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 17; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r9, (rax) as u64); return rax; }
            27 => { crate::simd_ops::wu64(rcx, (rax) as u64); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001de60c0; const P2: u64 = 0x7001de64c0; const P3: u64 = 0x7001de487e;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r11 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r14 = (r8) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); rbx = (rdx) as u64; rsi = (rcx) as u64; r9 = (rdi) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8)) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((r10).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); bb = 5; continue; }
            4 => { r9 = (rax) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rax)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            9 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (0x28u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = (rax) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (rsi) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(r8, (rdi) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rax) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (rbx) as u64; crate::simd_ops::wu64(rbx, (r9) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de48d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7798; const P1: u64 = 0x7001df77a8; const P2: u64 = 0x7001de2010; const P3: u64 = 0x7001de60c0; const P4: u64 = 0x7001dc7850; const P5: u64 = 0x7001de64c0; const P6: u64 = 0x7001de4b16; const P7: u64 = 0x7001df7650; const P8: u64 = 0x7001df5ca0; const P9: u64 = 0x7001dc7cd0; const P10: u64 = 0x7001de4b2a; const P11: u64 = 0x7001df7638; const P12: u64 = 0x7001de4b3e; const P13: u64 = 0x7001df7680; const P14: u64 = 0x7001de4b52; const P15: u64 = 0x7001df76a8; const P16: u64 = 0x7001de4b66;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); rax = (r8) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r8) as u64); let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rbp as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((r8) as i64) as f64).to_bits() as u128); bb = 3; continue; }
            2 => { rax = (r8) as u64; r8 = ((((r8 as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (r8))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm0 as u64)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P0))).to_bits() as u128); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm1 = ((crate::simd_ops::ru64(P1)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm1 as u64)).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0x8000000000000000u64) as u64; rcx = (rax) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), (rbp) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (r9) as u64); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (0x28u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (rbp) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (0x28u32) as u32 as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), (rbp) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rbp) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 8; continue; }
            8 => { rax = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 9; continue; }
            9 => { fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 10; continue; }
            10 => { fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); rcx = (0x18u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); bb = 14; continue; }
            13 => { rbx = (rbp) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((r9).wrapping_add(crate::simd_ops::ru64(rbx))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); bb = 19; continue; }
            18 => { r8 = (rdx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = (0x28u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = (rax) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax, (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((rbp as u16)) as u16); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rdi) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rdx = ((P7) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rdx = ((P11) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = ((P13) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rdx = ((P15) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x19; const P2: u64 = 0x20; const P3: u64 = 0x7001de2030; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x48; const P7: u64 = 0x7001de4540;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rbx = (rax) as u64; bb = 11; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 12; continue; }
            12 => { rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rsi.wrapping_add(P0)) as u64) as u64; rcx = ((rsi.wrapping_add(P0)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x7001dc8180; const P3: u64 = 0x18; const P4: u64 = 0x30; const P5: u64 = 0x7001dc5990; const P6: u64 = 0x7001dc79c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); bb = 2; continue; }
            2 => { r8 = (0x8u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let d = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r9 = (0x2u32) as u32 as u64; r8 = (rbp) as u64; rsi = (rdx) as u64; rcx = (rdi) as u64; rsi = (((rsi) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(P4))) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (rbp) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rsi) as u64; bb = 5; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x10; const P5: u64 = 0x7001dc6c20; const P6: u64 = 0x7001dc74c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = (rdx) as u64; rdi = (rcx) as u64; r9 = (rbx) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 4; continue; }
            3 => { rbx = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (rbx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (0xffffffffffffffffu64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc6c20; const P1: u64 = 0x7001dc74c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rbp = (r8) as u64; rax = (rdx) as u64; r14 = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r8 = (rdx) as u64; rdi = (rcx) as u64; rsi = ((rdx.wrapping_add(rbp)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rbx = (r10) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 7; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = (rdi) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = (rbp) as u64; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            14 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r14 as u32)) as u32); bb = 16; continue; }
            15 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de4fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r9 = (rdx) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; r8 = (r10) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; return rax; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (r8) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r10 = (rdx) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdx, (r9) as u64); rax = (rdx) as u64; return rax; }
            2 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r10, (r9) as u64); rax = (r10) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001de60c0; const P2: u64 = 0x7001de64c0; const P3: u64 = 0x7001de518a;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rsi = ((rcx.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x28u64), (rax) as u64); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rsi)) as u64; rbx = (rdx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); r9 = (rdi) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (rax) as u64 as u64; bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); bb = 7; continue; }
            6 => { r9 = (rcx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64 as u64; fl_r = (rax) as u64 as u64; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (0x28u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = (rax) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax, (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de51d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (r8) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 4; continue; }
            3 => { r8 = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; return rax; }
            7 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x19; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x10; const P5: u64 = 0x28; const P6: u64 = 0x7001de5c40; const P7: u64 = 0x7001de2050;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (rdx) as u64; rbx = (rcx) as u64; r8 = (r11) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r10)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (rcx) as u64 as u64; bb = 4; continue; }
            4 => { rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 7; continue; }
            6 => { r8 = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(r10)) as u64 as u64; fl_r = (rax) as u64 as u64; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r8 = (r11) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::lk_sub_u64(rbx.wrapping_add(P5), (rcx) as u64); rdx = (r8) as u64; rcx = ((rbx.wrapping_add(P0)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0x28u32) as u32 as u64; rcx = (rax) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de52f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc80b0; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x19; const P4: u64 = 0x20; const P5: u64 = 0x10; const P6: u64 = 0x7001df76d0; const P7: u64 = 0x7001df5ca0; const P8: u64 = 0x7001dc7cd0; const P9: u64 = 0x7001de53bd;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; r8 = (r9) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = (r8) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rax))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rbx.wrapping_add(rsi)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rdx = ((P6) as u64) as u64; rcx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x7001de5c40; const P3: u64 = 0x7001de2050; const P4: u64 = 0x7001de2030;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = (r8) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::lk_add_u64(r9.wrapping_add(P1), (rax) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (0x28u32) as u32 as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7634; const P1: u64 = 0x7001dd6e00; const P2: u64 = 0x7001e0a430; const P3: u64 = 0x7001df771c; const P4: u64 = 0x7001df7710; const P5: u64 = 0x7001df7704; const P6: u64 = 0x7001df76f8; const P7: u64 = 0x7001df76ec; const P8: u64 = 0x7001df6d58; const P9: u64 = 0x7001dc7cd0; const P10: u64 = 0x7001de5558;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = (rdx) as u64; r14 = (rcx) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(rdx).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx).wrapping_add(0x18u64), (0x808u32) as u32); rdx = ((P0) as u64) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (r15) as u64; rdi = ((rbx.wrapping_add(rsi)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbx = ((P5) as u64) as u64; bb = 6; continue; }
            4 => { rbx = ((P6) as u64) as u64; bb = 6; continue; }
            5 => { rbx = ((P7) as u64) as u64; bb = 6; continue; }
            6 => { rdx = ((P3) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(r15)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r15).wrapping_add(0x18u64), ((rdx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de55a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7720; const P1: u64 = 0x7001dd6e00; const P2: u64 = 0x7001e0a430; const P3: u64 = 0x7001df771c; const P4: u64 = 0x7001df6d10; const P5: u64 = 0x7001df7738; const P6: u64 = 0x7001df7748; const P7: u64 = 0x7001df7758; const P8: u64 = 0x7001df7768; const P9: u64 = 0x7001df777c; const P10: u64 = 0x7001de5450; const P11: u64 = 0x7001df7780; const P12: u64 = 0x7001df6d20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(rdx).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx).wrapping_add(0x18u64), (0x808u32) as u32); rdx = ((P0) as u64) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (rsi) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P5) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P6) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P8) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((P9) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rbx = (rax) as u64; bb = 10; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rdx = ((P11) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((P9) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rbx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 16; continue; }
            16 => { rbx = (rax) as u64; bb = 20; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 21; continue; }
            21 => { rdx = ((P12) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rsi).wrapping_add(0x18u64), ((rdx as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x7001de2010; const P2: u64 = 0x8; const P3: u64 = 0x7001dc74c0; const P4: u64 = 0x38; const P5: u64 = 0x7001de4760; const P6: u64 = 0x7001dc7850;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rsi = (rcx) as u64; rcx = (0x18u32) as u32 as u64; rdi = (r8) as u64; rbx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r9).wrapping_sub(rdi)) as u64; fl_l = ((rdx).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), (r9) as u64); crate::simd_ops::wu64(rax, (rdx) as u64); crate::simd_ops::wu32(rax.wrapping_add(P0), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), (rdi) as u64); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            4 => { rcx = ((rsi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), (rdi) as u64); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc6c20; const P1: u64 = 0x7001dc74c0; const P2: u64 = 0x7001de5c40; const P3: u64 = 0x7001de2050; const P4: u64 = 0x7001de2030; const P5: u64 = 0x7001de4640; const P6: u64 = 0x7001dc7850;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r12 = (r8) as u64; r9 = (rdx) as u64; r15 = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rdi = (r10) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (rax) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 7; continue; }
            7 => { fl_l = (rdi) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbp) as u64); fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 11; continue; }
            11 => { rdx = (r14) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdi = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rdi = (rax) as u64; bb = 19; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14)) as u64; bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rax).wrapping_add(rbp)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rsi) as u64); fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (rdi) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 23; continue; }
            23 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 25; continue; }
            25 => { bb = 28; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = ((r15.wrapping_add(0x38u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x8u64), (rax) as u64); rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0x28u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 30; continue; }
            30 => { fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = (crate::simd_ops::ru64(r14)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rdx = ((r12.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = ((r15.wrapping_add(0x38u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(r8.wrapping_add(0x8u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((r12.wrapping_add(0x28u64)) as u32) as u32 as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; bb = 34; continue; }
            34 => { rdx = (r14) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = (rbp) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de5c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de6450; const P1: u64 = 0x7001de63e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r8 = ((rdx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rsi = (rcx) as u64; r9 = (rdx) as u64; r14 = (rdx) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 8; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rdx) as u64; rdx = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdi = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = (rcx) as u64; bb = 12; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rbx) as u64); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rdi) as u64); bb = 19; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx, (rdi) as u64); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = (rbx) as u64; bb = 25; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (rdi) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rcx) as u64; rcx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(r8, (rdx) as u64); bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rbx) as u64); rcx = (rbx) as u64; rdx = ((crate::simd_ops::ru8(r9.wrapping_add(0x18u64))) as u32) as u32 as u64; bb = 45; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (rdi) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { bb = 31; continue; }
            31 => { rcx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rcx) as u64); bb = 33; continue; }
            33 => { rdx = ((crate::simd_ops::ru8(r9.wrapping_add(0x18u64))) as u32) as u32 as u64; bb = 45; continue; }
            34 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r8)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rbx = (rax) as u64; bb = 39; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rbx) as u64); bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); bb = 44; continue; }
            41 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 43; continue; } bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rcx, (rax) as u64); bb = 44; continue; }
            43 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), ((rcx as u8)) as u8); crate::simd_ops::wu8(r14.wrapping_add(0x18u64), ((rdx as u8)) as u8); bb = 45; continue; }
            45 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 84; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 83; continue; } bb = 47; continue; }
            47 => { bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; r8 = (rbx) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 83; continue; } bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 65; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 59; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 58; continue; }
            55 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(rax, (rcx) as u64); bb = 58; continue; }
            57 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 59; continue; }
            59 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 78; continue; } bb = 60; continue; }
            60 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 77; continue; } bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 64; continue; } bb = 63; continue; }
            63 => { crate::simd_ops::wu8(rdx.wrapping_add(0x18u64), (0x1u8) as u8); rdx = (rcx) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 64; continue; }
            64 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; rdx = (rbx) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = (rsi) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 83; continue; }
            65 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 74; continue; } bb = 66; continue; }
            66 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 68; continue; } bb = 67; continue; }
            67 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 69; continue; }
            69 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 73; continue; }
            70 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 71; continue; }
            71 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); bb = 73; continue; }
            72 => { crate::simd_ops::wu64(rax, (rcx) as u64); bb = 73; continue; }
            73 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 74; continue; }
            74 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 78; continue; } bb = 75; continue; }
            75 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 80; continue; } bb = 76; continue; }
            76 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 80; continue; } bb = 77; continue; }
            77 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); bb = 78; continue; }
            78 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = (r8) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 79; continue; }
            79 => { bb = 83; continue; }
            80 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 82; continue; } bb = 81; continue; }
            81 => { crate::simd_ops::wu8(rdx.wrapping_add(0x18u64), (0x1u8) as u8); rdx = (rcx) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 82; continue; }
            82 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; rdx = (rbx) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (rsi) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 83; continue; }
            83 => { crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), (0x1u8) as u8); bb = 84; continue; }
            84 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (r14) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 85; continue; }
            85 => { fl_l = ((rcx).wrapping_sub(1)) as u64 as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rcx) as u64); bb = 86; continue; }
            86 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de60c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x8u64), (1) as u64); r11 = (r8) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi, (r8) as u64); rax = (r8) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r8) as u64); crate::simd_ops::wu8(r8.wrapping_add(0x18u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r11) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r11) as u64); bb = 7; continue; }
            5 => { crate::simd_ops::wu64(rax, (r11) as u64); fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, (r11) as u64); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (r11) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r9 = ((rdx.wrapping_add(0x8u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = ((rcx.wrapping_add(0x8u64)) as u64) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); r9 = (r10) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); crate::simd_ops::wu64(r10, (r8) as u64); bb = 20; continue; }
            16 => { rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rax, (r8) as u64); r9 = (r10) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); crate::simd_ops::wu64(r10, (r8) as u64); bb = 20; continue; }
            18 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r8) as u64); r9 = (r10) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); crate::simd_ops::wu64(r10, (r8) as u64); bb = 20; continue; }
            19 => { r8 = (rcx) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu8(r8.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rcx) as u64); bb = 46; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r8) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rcx) as u64); bb = 46; continue; }
            26 => { crate::simd_ops::wu64(rax, (r8) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), (rcx) as u64); bb = 46; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x1u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 47; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 30; continue; }
            30 => { rdx = (rcx) as u64; rcx = (rax) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rdx) as u64); bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 37; continue; }
            34 => { rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); bb = 37; continue; }
            36 => { crate::simd_ops::wu64(rax, (rcx) as u64); bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rdx) as u64); r9 = (r10) as u64; crate::simd_ops::wu64(r10, (rcx) as u64); bb = 38; continue; }
            38 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); bb = 45; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rax, (r8) as u64); bb = 45; continue; }
            44 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r8) as u64); bb = 45; continue; }
            45 => { crate::simd_ops::wu64(r8, (rcx) as u64); bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r8) as u64); bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (r11) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de63e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x19; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (rax) as u64);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { crate::simd_ops::wu64(rax.wrapping_add(P2), (rdx) as u64); }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rax.wrapping_add(P2), (r8) as u64); crate::simd_ops::wu64(r8, (rdx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64); return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rax, (r8) as u64); crate::simd_ops::wu64(r8, (rdx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64); return rax; }
    crate::simd_ops::wu64(rax.wrapping_add(P0), (r8) as u64);
    crate::simd_ops::wu64(r8, (rdx) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x19; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, (rax) as u64);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { crate::simd_ops::wu64(rax.wrapping_add(P2), (rdx) as u64); }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rax.wrapping_add(P2), (r8) as u64); crate::simd_ops::wu64(r8.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64); return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rax.wrapping_add(P0), (r8) as u64); crate::simd_ops::wu64(r8.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64); return rax; }
    crate::simd_ops::wu64(rax, (r8) as u64);
    crate::simd_ops::wu64(r8.wrapping_add(P0), (rdx) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), (r8) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de64e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df77b8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64);
    rax = (rcx) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001df77b8; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rdx) as u64);
    rax = ((P1) as u64) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    rbx = (rcx) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = (rdx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64);
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = (rdx) as u64;
    rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    crate::simd_ops::wu64(rbx, (rcx) as u64);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de66e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001dc7430;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5, 32>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    // stackargs: A=32 max=4
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    r8 = (rdx) as u64;
    r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64;
    xmm0 = (xmm1) as u128;
    rcx = ((xmm1 as u64)) as u64;
    xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8);
    rdx = ((xmm0 as u32)) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e070b8; const P1: u64 = 0x7001e0a138;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x78u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64);
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x40u32) as u32);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (0x7fffffffffffffffu64) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64;
    rax = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { rcx } else { rax }) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rsp = ((rsp).wrapping_add(0x78u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e0a140;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = 0u128;
    rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001defd10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    rdi = (r8) as u64;
    rbx = ((rbx).wrapping_sub(rcx)) as u64;
    rdx = (rcx) as u64;
    r8 = (rbx) as u64;
    rcx = (rdi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = ((rbx.wrapping_add(rdi)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de69c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001de8930; const P2: u64 = 0x7001df78d0; const P3: u64 = 0x7001deddc0; const P4: u64 = 0x7001de6c2f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = (rcx) as u64; rcx = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; rbx = (((rbx) ^ (rax))) as u64; r14 = (rdx) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r15 = (r8) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x6u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x7u64))) as u32) as u32 as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rdx = (rbx) as u64; rdx = (((rdx) & (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x0u8) as u8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (r14) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rbp = (rax) as u64; bb = 7; continue; }
            7 => { rax = (0xaaaaaaaaaaaaaaau64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 8; continue; }
            8 => { rcx = (0x18u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r15)) as u64; rdi = (rax) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 11; continue; }
            10 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 14; continue; }
            13 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (rbx) as u64; rax = (((rax) & (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 21; continue; }
            17 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rdi, (rbp) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rcx) as u64); crate::simd_ops::wu64(rcx, (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rdi) as u64); rbx = (((rbx) & (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rax.wrapping_add(rbx.wrapping_mul(8)), (rdi) as u64); bb = 28; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 21; continue; }
            25 => { fl_l = (rdx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rax.wrapping_add(rbx.wrapping_mul(8)), (rdi) as u64); crate::simd_ops::wu64(r14, (rdi) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x1u8) as u8); bb = 5; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64), (rdi) as u64); bb = 29; continue; }
            29 => { crate::simd_ops::wu64(r14, (rdi) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(0x8u64), (0x1u8) as u8); bb = 5; continue; }
            30 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r10 = (rdx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_sub(rcx)) as u64; r9 = (rcx) as u64; rdx = ((rdx).wrapping_add(0x7u64)) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rdx) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = ((rcx).wrapping_add(0xfffffffffffffff8u64)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp, (rdi) as u64); rdx = (((rdx) & (0xfffffffffffffffeu64))) as u64; rdi = (r9) as u64; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(3u32)) as u64; while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u64, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } fl_l = ((r9).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; rdi = (crate::simd_ops::ru64(rsp)) as u64; bb = 5; continue; }
            5 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9, (rax) as u64); r9 = ((r9).wrapping_add(0x8u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07568; const P1: u64 = 0x7001e07570; const P2: u64 = 0x7001dc54b0; const P3: u64 = 0x7001df7910; const P4: u64 = 0x7001dc7c30; const P5: u64 = 0x7001dc5060; const P6: u64 = 0x7001e07580; const P7: u64 = 0x7001dc6eb0; const P8: u64 = 0x7001dc5350; const P9: u64 = 0x7001e0a980;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rdi = ((P1) as u64) as u64; rbx = (rcx) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P2) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (rdi) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (rax) as u64); rdx = ((r11.wrapping_sub(0x48u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (rax) as u64); rcx = ((P0) as u64) as u64; rax = ((r11.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (0x8u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(32u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x7001de2050; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x7001e0a720; const P7: u64 = 0x7001de6f93;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (r8) as u64; bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P0), (rdx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P1), (rdx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P4), (rdx) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdx = (0x18u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de6fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de8ad0; const P1: u64 = 0x7001de8a40; const P2: u64 = 0x7001defd10; const P3: u64 = 0x7001de2050; const P4: u64 = 0x7001e0a720; const P5: u64 = 0x7001de7223; const P6: u64 = 0x7001de8a20; const P7: u64 = 0x7001de7229;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (r8) as u64; crate::simd_ops::wu64(rcx, (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); r13 = (r8) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rdi) as u64); rbx = (rcx) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { r9 = (0x100000001b3u64) as u64; r8 = (0x1fffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rax = (0xcbf29ce484222325u64) as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rax = (rdi) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((rax).wrapping_shr(8u32)) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(40u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rdi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(56u32)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = (((rcx) & (rdx))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsi, (rdi) as u64); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 23; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbp = (rsi) as u64; rbp = ((rbp).wrapping_sub(rax)) as u64; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_sub(rax)) as u64; r12 = ((rbp.wrapping_add(0x1u64)) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = (r8) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r14 = (r8) as u64; bb = 14; continue; }
            13 => { r14 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = (r14) as u64 as u64; fl_r = (r12) as u64 as u64; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r14 }) as u64; bb = 14; continue; }
            14 => { rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r15 = (rax) as u64; rcx = (rax) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbp.wrapping_mul(8)), (rdi) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 17; continue; }
            16 => { r8 = (rsi) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rbp.wrapping_add(0x1u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rsi) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = (rsi) as u64; bb = 17; continue; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rcx = (r8) as u64; bb = 21; continue; }
            21 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            22 => { rax = ((r15.wrapping_add(r12.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx, (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); r8 = (0x1fffffffffffffffu64) as u64; rax = ((r15.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; r9 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 24; continue; }
            24 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 25; continue; }
            25 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            26 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de72e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (rcx) as u64;
    rax = ((rax).wrapping_shr(33u32)) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rcx = (0xff51afd7ed558ccdu64) as u64;
    rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64;
    rcx = (rax) as u64;
    rcx = ((rcx).wrapping_shr(33u32)) as u64;
    rcx = (((rcx) ^ (rax))) as u64;
    rax = (0xc4ceb9fe1a85ec53u64) as u64;
    rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64;
    rax = (rcx) as u64;
    rax = ((rax).wrapping_shr(33u32)) as u64;
    fl_l = (((rax) ^ (rcx))) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de7330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (r8) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (r9) as u64);
    rax = ((rax).wrapping_shl(23u32)) as u64;
    r8 = (((r8) ^ (rax))) as u64;
    rax = (r9) as u64;
    rax = ((rax).wrapping_shr(9u32)) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = ((rax).wrapping_shr(17u32)) as u64;
    rax = (((rax) ^ (r9))) as u64;
    rax = (((rax) ^ (r8))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    rcx = (0x40u32) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64;
    rax = ((rax).wrapping_add(r9)) as u64;
    fl_l = ((rax).wrapping_shr(((rcx as u32) & 63))) as u64 as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de7380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r11 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    loop { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (r9) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (r10) as u64); rax = ((rax).wrapping_shl(23u32)) as u64; r9 = (((r9) ^ (rax))) as u64; rax = (r10) as u64; rax = ((rax).wrapping_shr(9u32)) as u64; rax = (((rax) ^ (r9))) as u64; rax = ((rax).wrapping_shr(17u32)) as u64; rax = (((rax) ^ (r10))) as u64; rax = (((rax) ^ (r9))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64); rax = ((rax).wrapping_add(r10)) as u64; rax = ((rax).wrapping_shr(56u32)) as u64; crate::simd_ops::wu8(r11.wrapping_add(rdx), ((rax as u8)) as u8); r11 = ((r11).wrapping_add(1)) as u64; fl_l = (r11) as u64 as u64; fl_r = (r8) as u64 as u64; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de73f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x7001df5d90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (rdx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (r8) as u64);
    rax = ((rax).wrapping_shl(23u32)) as u64;
    rdx = (((rdx) ^ (rax))) as u64;
    rax = (r8) as u64;
    rax = ((rax).wrapping_shr(9u32)) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    rax = ((rax).wrapping_shr(17u32)) as u64;
    rax = (((rax) ^ (r8))) as u64;
    r8 = ((r8).wrapping_shr(12u32)) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    rax = (0x3ff0000000000000u64) as u64;
    fl_l = (((r8) | (rax))) as u64 as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (r8) as u64);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64) as u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de7460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (rdx) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (r8) as u64);
    rax = ((rax).wrapping_shl(23u32)) as u64;
    rdx = (((rdx) ^ (rax))) as u64;
    rax = (r8) as u64;
    rax = ((rax).wrapping_shr(9u32)) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    rax = ((rax).wrapping_shr(17u32)) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    fl_l = ((rax).wrapping_add(r8)) as u64 as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de74a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = ((((rdx as u32)) as i32 as i64) as u64) as u64; r9 = (rcx) as u64; r10 = ((r8.wrapping_add(P0)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shl(23u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rcx) as u64); rax = (rcx) as u64; rax = ((rax).wrapping_shr(9u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = ((rax).wrapping_shr(17u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) ^ (rdx))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P2), (rax) as u64); rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_shr(33u32)) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = ((rax).wrapping_shr(31u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; return rax; }
            3 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = (rcx) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rdx) as u64); rax = ((rax).wrapping_shl(23u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(9u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(17u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) ^ (rcx))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P2), (rax) as u64); rcx = ((rax.wrapping_add(rdx)) as u64) as u64; rcx = ((rcx).wrapping_shr(33u32)) as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((r8 as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x7fffffffu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((rdx as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de7570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001df794c; const P2: u64 = 0x7001de8420; const P3: u64 = 0x7001df5d90; const P4: u64 = 0x7001df77a8; const P5: u64 = 0x7001de69c0; const P6: u64 = 0x7001de6fc0; const P7: u64 = 0x7001dc5d00; const P8: u64 = 0x7001de8a40; const P9: u64 = 0x7001de6ef0; const P10: u64 = 0x7001de8a20; const P11: u64 = 0x7001de786a; const P12: u64 = 0x7001df7838; const P13: u64 = 0x7001df5ca0; const P14: u64 = 0x7001dc7cd0; const P15: u64 = 0x7001de787e; const P16: u64 = 0x7001df7848; const P17: u64 = 0x7001de7892;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r15 = (r9) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; r13 = (rcx) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), (rsi) as u64); fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, (rsi) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rsi) as u64); bb = 26; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), (rdi) as u64); rdi = (rbx) as u64; rdi = ((rdi).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (r12) as u64); fl_l = (r15) as u64 as u64; fl_r = (rdi) as u64 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm7) as u128); rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rdi }) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rsi) as u64); rcx = ((rsi.wrapping_add(0x18u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; rdx = ((rsi.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x7u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x8u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = ((rsi as u32)) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { xmm7 = ((crate::simd_ops::ru64(P3)) as u64) as u128; rax = (0xaaaaaaaaaaaaaaabu64) as u64; xmm6 = ((crate::simd_ops::ru64(P4)) as u64) as u128; r8 = (0x3ff0000000000000u64) as u64; r12 = (0x8000000000000000u64) as u64; bb = 5; continue; }
            5 => { let prod = (rax as u128).wrapping_mul((rbp) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; xmm1 = 0u128; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (rcx) as u64; crate::simd_ops::wu64(r13.wrapping_add(0x8u64), (rdx) as u64); rax = ((rax).wrapping_shl(23u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(9u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(17u32)) as u64; rax = (((rax) ^ (rdx))) as u64; rdx = ((rdx).wrapping_shr(12u32)) as u64; rdx = (((rdx) | (r8))) as u64; rax = (((rax) ^ (rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rdx) as u64); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64) as u128; crate::simd_ops::wu64(r13.wrapping_add(0x10u64), (rax) as u64); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm7 as u64)).to_bits() as u128); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); bb = 9; continue; }
            8 => { rax = (rbx) as u64; rcx = (rbx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm6 as u64)).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (r12) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 13; continue; }
            13 => { r8 = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = ((rbp).wrapping_add(1)) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; r8 = (0x3ff0000000000000u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 14; continue; }
            14 => { fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            16 => { fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 17; continue; }
            17 => { rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r9 = (r15) as u64; r8 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = (r14) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(r14, (rsi) as u64); rax = (rbx) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rsi) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rax)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rax = (0x1fffffffffffffffu64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 27; continue; } bb = 22; continue; }
            22 => { rdx = (rsi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = ((rax).wrapping_add(0x8u64)) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 25; continue; }
            25 => { rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rax = (r14) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = ((P12) as u64) as u64; rcx = ((P13) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rdx = ((P16) as u64) as u64; rcx = ((P13) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P17, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de7960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001de8a40; const P2: u64 = 0x7001defd10; const P3: u64 = 0x7001de2050; const P4: u64 = 0x7001e0a720; const P5: u64 = 0x7001de7c09; const P6: u64 = 0x7001df5d90; const P7: u64 = 0x7001df77a8; const P8: u64 = 0x7001df794c; const P9: u64 = 0x7001de8420; const P10: u64 = 0x7001de8930; const P11: u64 = 0x7001de6fc0; const P12: u64 = 0x7001de6ef0; const P13: u64 = 0x7001de8028; const P14: u64 = 0x7001df78d0; const P15: u64 = 0x7001deddc0; const P16: u64 = 0x7001de8089; const P17: u64 = 0x7001df7858; const P18: u64 = 0x7001df5ca0; const P19: u64 = 0x7001dc7cd0; const P20: u64 = 0x7001de809d; const P21: u64 = 0x7001dcf9d0; const P22: u64 = 0x7001de80a3; const P23: u64 = 0x7001de8a20; const P24: u64 = 0x7001de80a9; const P25: u64 = 0x7001df7878; const P26: u64 = 0x7001de80bd;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((r11.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r13 = (r9) as u64; rax = (r8) as u64; r9 = (r8) as u64; r15 = (rcx) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8.wrapping_add(0x10u64)))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 86; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (rbx) as u64); r10 = (0x1fffffffffffffffu64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (rsi) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (rdi) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (r12) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), (r14) as u64); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 88; continue; } bb = 3; continue; }
            3 => { rdi = ((rax.wrapping_mul(8)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(0x27u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 87; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 6; continue; }
            6 => { rbx = ((rax.wrapping_add(0x27u64)) as u64) as u64; fl_l = (((rbx) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), (rax) as u64); bb = 9; continue; }
            7 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; r10 = (0x1fffffffffffffffu64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; bb = 10; continue; }
            10 => { r14 = ((rbx.wrapping_add(rdi)) as u64) as u64; bb = 11; continue; }
            11 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (rbx) as u64; rax = (0xcbf29ce484222325u64) as u64; r11 = (0x100000001b3u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rdx = (((rsi as u8)) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rax = (rsi) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = ((rax).wrapping_shr(8u32)) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(24u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(40u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(48u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rax = (rsi) as u64; rdx = (((rdx) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(56u32)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 18; continue; }
            18 => { fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx, (rsi) as u64); fl_l = ((rbx).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 29; continue; }
            20 => { rbx = ((rbx).wrapping_sub(rdi)) as u64; r13 = (rbx) as u64; r13 = ((((r13) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (r13) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 88; continue; } bb = 21; continue; }
            21 => { r14 = ((r14).wrapping_sub(rdi)) as u64; rdx = ((r13.wrapping_add(0x1u64)) as u64) as u64; r14 = ((((r14) as i64).wrapping_shr(3u32) as u64)) as u64; rax = (r10) as u64; rcx = (r14) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r15 = (r10) as u64; bb = 24; continue; }
            23 => { r15 = ((r14.wrapping_add(rcx)) as u64) as u64; fl_l = (r15) as u64 as u64; fl_r = (rdx) as u64 as u64; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r15 }) as u64; bb = 24; continue; }
            24 => { rdx = (r15) as u64; rcx = ((rbp.wrapping_sub(0x51u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rdi) as u64; rcx = (rax) as u64; r12 = (rax) as u64; crate::simd_ops::wu64(rax.wrapping_add(r13.wrapping_mul(8)), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rdx = ((r14.wrapping_mul(8)) as u64) as u64; rax = (rdi) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rbx = ((r13.wrapping_add(0x1u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rbx = ((r12.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rdi = (r12) as u64; r14 = ((r12.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; r11 = (0x100000001b3u64) as u64; bb = 29; continue; }
            29 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rax = (0xcbf29ce484222325u64) as u64; r10 = (0x1fffffffffffffffu64) as u64; bb = 13; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x5fu64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 33; continue; }
            33 => { r9 = ((r9).wrapping_sub(r13)) as u64; rax = (rbx) as u64; fl_l = (r9) as u64 as u64; fl_r = (r13) as u64 as u64; r12 = (rbx) as u64; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { r9 }) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 34; continue; }
            34 => { xmm3 = ((crate::simd_ops::ru64(P6)) as u64) as u128; rcx = (rbx) as u64; xmm2 = ((crate::simd_ops::ru64(P7)) as u64) as u128; r8 = (rbx) as u64; r11 = (0x3ff0000000000000u64) as u64; r10 = (0x8000000000000000u64) as u64; bb = 35; continue; }
            35 => { rcx = ((rcx).wrapping_sub(rdi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 44; continue; } bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; xmm1 = 0u128; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (rcx) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rdx) as u64); rax = ((rax).wrapping_shl(23u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(9u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = ((rax).wrapping_shr(17u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = (rbx) as u64; rax = (((rax) ^ (rdx))) as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; rdx = ((rdx).wrapping_shr(12u32)) as u64; rdx = (((rdx) | (r11))) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (rdx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rcx) as i64) as f64).to_bits() as u128); bb = 39; continue; }
            38 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); bb = 39; continue; }
            39 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64) as u128; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm3 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 42; continue; } bb = 40; continue; }
            40 => { xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm2 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = (r10) as u64; bb = 42; continue; }
            42 => { r8 = ((r8).wrapping_sub(rdi)) as u64; rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 89; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rbx = ((rbx).wrapping_add(0xfffffffffffffff8u64)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)))) as u64; r12 = (rbx) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), (rax) as u64); r8 = (rbx) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(rbx, (rcx) as u64); rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; rcx = (rbx) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 44; continue; }
            44 => { rax = (r12) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 83; continue; } bb = 45; continue; }
            45 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rbx) as u64); rcx = ((rbx.wrapping_add(0x18u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = (rax) as u64; rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru32(P8)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (0x7u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (0x8u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rdi) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 77; continue; } bb = 46; continue; }
            46 => { r15 = ((rdi.wrapping_add(0x2u64)) as u64) as u64; r8 = (0xaaaaaaaaaaaaaaau64) as u64; bb = 47; continue; }
            47 => { rsi = ((crate::simd_ops::ru8(r15.wrapping_sub(0x2u64))) as u32) as u32 as u64; rcx = (0x100000001b3u64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rax = (0xcbf29ce484222325u64) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_sub(0x1u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x1u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x2u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x3u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x4u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x5u64))) as u32) as u32 as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rdx = (((rdx) & (rsi))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 53; continue; } bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_sub(0x2u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 76; continue; } bb = 49; continue; }
            49 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 51; continue; }
            51 => { bb = 76; continue; }
            52 => { r13 = (rax) as u64; bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 85; continue; } bb = 54; continue; }
            54 => { rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r15.wrapping_sub(0x2u64))) as u64; rbx = (rax) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 57; continue; }
            56 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 59; continue; } bb = 58; continue; }
            58 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 60; continue; }
            59 => { rax = (rcx) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 60; continue; }
            60 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x31u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 68; continue; } bb = 61; continue; }
            61 => { rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (((rax) & (rsi))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 63; continue; } bb = 62; continue; }
            62 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (0x0u64) as u64); bb = 67; continue; }
            63 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 66; continue; } bb = 64; continue; }
            64 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 64; continue; } bb = 66; continue; }
            66 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (rcx) as u64); bb = 67; continue; }
            67 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x61u64))) as u128; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x51u64), (xmm0) as u128); r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; bb = 68; continue; }
            68 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rdx) as u64); crate::simd_ops::wu64(rbx, (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8, (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (((rax) & (rsi))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 71; continue; } bb = 69; continue; }
            69 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 74; continue; }
            70 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (0x0u64) as u64); bb = 67; continue; }
            71 => { fl_l = (rdx) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 73; continue; } bb = 72; continue; }
            72 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 75; continue; }
            73 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 74; continue; }
            74 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); bb = 75; continue; }
            75 => { r8 = (0xaaaaaaaaaaaaaaau64) as u64; bb = 76; continue; }
            76 => { r15 = ((r15).wrapping_add(0x8u64)) as u64; rax = ((r15.wrapping_sub(0x2u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 47; continue; } bb = 77; continue; }
            77 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 82; continue; } bb = 78; continue; }
            78 => { r14 = ((r14).wrapping_sub(rdi)) as u64; rax = (rdi) as u64; r14 = (((r14) & (0xfffffffffffffff8u64))) as u64; fl_l = (r14) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 81; continue; } bb = 79; continue; }
            79 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; r14 = ((r14).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 81; continue; } bb = 80; continue; }
            80 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            81 => { rdx = (r14) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 82; continue; }
            82 => { rax = (rbx) as u64; bb = 84; continue; }
            83 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rax, (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r14) as u64); bb = 84; continue; }
            84 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            85 => { rcx = ((P14) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            86 => { rdx = ((P17) as u64) as u64; rcx = ((P18) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P20, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            87 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P22, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            88 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P24, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            89 => { rdx = ((P25) as u64) as u64; rcx = ((P18) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P26, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de82a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e07568; const P1: u64 = 0x7001e07570; const P2: u64 = 0x7001dc54b0; const P3: u64 = 0x7001df7910; const P4: u64 = 0x7001dc7c30; const P5: u64 = 0x7001dc5060; const P6: u64 = 0x7001e07580; const P7: u64 = 0x7001dc5350;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = (rsp) as u64;
    crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x70u64)) as u64;
    rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64;
    rdi = ((P1) as u64) as u64;
    rbx = (rcx) as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = ((P2) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (rdi) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), (rax) as u64); rdx = ((r11.wrapping_sub(0x48u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (rax) as u64); rcx = ((P0) as u64) as u64; rax = ((r11.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; }
    rcx = (rdi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = (rdi) as u64;
    crate::simd_ops::wu64(P6, (rbx) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    rsp = ((rsp).wrapping_add(0x70u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x7001df7890; const P3: u64 = 0x7001df5ca0; const P4: u64 = 0x7001dc7cd0; const P5: u64 = 0x7001de83ea;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); r9 = (rcx) as u64; rax = (rdx) as u64; r11 = (0xff51afd7ed558ccdu64) as u64; rax = ((rax).wrapping_shr(33u32)) as u64; r10 = (0xc4ceb9fe1a85ec53u64) as u64; rax = (((rax) ^ (rdx))) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; rdx = (rax) as u64; rdx = ((rdx).wrapping_shr(33u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(33u32)) as u64; r8 = (((r8) ^ (rdx))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), (r8) as u64); rax = (r8) as u64; rax = (!(rax)) as u64; rcx = (rax) as u64; rcx = ((rcx).wrapping_shr(33u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = (rcx) as u64; rax = ((rax).wrapping_shr(33u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = (rax) as u64; rcx = ((rcx).wrapping_shr(33u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rcx) as u64); fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rdx = ((P2) as u64) as u64; rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2010; const P1: u64 = 0x7001de2050; const P2: u64 = 0x7001e0a720; const P3: u64 = 0x7001de8527; const P4: u64 = 0x7001de6cd0; const P5: u64 = 0x7001dcf9d0; const P6: u64 = 0x7001de8543;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (rax) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rbx = (r8) as u64; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rax = (0x1fffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { rsi = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(0x27u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { rdi = ((rax.wrapping_add(0x27u64)) as u64) as u64; fl_l = (((rdi) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), (rax) as u64); bb = 9; continue; }
            6 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; bb = 9; continue; }
            8 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rbp.wrapping_mul(8)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = (r8) as u64; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            14 => { rax = ((rsi.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(r14, (rdi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rax) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi, (rbx) as u64); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            18 => { r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x7001de2050; const P3: u64 = 0x8; const P4: u64 = 0x7001e0a720; const P5: u64 = 0x7001de8617;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rsi = (r9) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (r8) as u64; rdi = (rdx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (r8) as u64; bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = ((rdi.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx, (rdi) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); rax = ((rdi.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de2050;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rcx) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001de8420; const P1: u64 = 0x7001df78f0; const P2: u64 = 0x7001deddc0; const P3: u64 = 0x7001de887b;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0xfffffffffffffffu64) as u64; rsi = (rcx) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rdi = (0x1u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (((rax) | (rdi))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r8 = (rbx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r14) as u64); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdx = ((rdi.wrapping_add(rdi)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), (rdi) as u64); rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbp) as u64); rdi = (0x100000001b3u64) as u64; rbp = (0xcbf29ce484222325u64) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x11u64))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; r11 = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; r9 = (((r9) ^ (rbp))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x12u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x13u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x14u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x15u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x16u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x17u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; r9 = (((r9) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) & (r9))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; r9 = (crate::simd_ops::ru64(r11)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r11, (rcx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rcx) as u64); bb = 14; continue; }
            5 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10, (rdx) as u64); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, (r8) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (r9) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (r10) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rcx) as u64); bb = 14; continue; }
            9 => { fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10, (rdx) as u64); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, (rcx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r9) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (r10) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r11, (rcx) as u64); bb = 14; continue; }
            14 => { rcx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 15; continue; }
            15 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { r10 = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, (rdx) as u64); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, (r10) as u64); rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (r9) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); bb = 14; continue; }
            18 => { rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x38; const P2: u64 = 0x7001defdc0; const P3: u64 = 0x7001df7950; const P4: u64 = 0x7001de8690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (rcx) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 3; continue; }
            2 => { rax = (rdx) as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0x8000000000000000u64) as u64; rcx = (rax) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (0x8u32) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (0x200u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = ((rbx.wrapping_mul(8)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rbx) as u64; rcx = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x27; const P1: u64 = 0x7001de2010; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x7001e0a720; const P4: u64 = 0x7001de8a94; const P5: u64 = 0x7001dcf9d0; const P6: u64 = 0x7001de8aaf;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (0x1fffffffffffffffu64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x27u64)) as u64; rax = (((rax) & (0xffffffffffffffe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), (rcx) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001dc5930 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            6 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            8 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x7001de8a40; const P3: u64 = 0x7001defd10; const P4: u64 = 0x7001de8590; const P5: u64 = 0x7001de8a20; const P6: u64 = 0x7001de8b5f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rcx) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x1fffffffffffffffu64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rdi = ((rdi).wrapping_sub(rcx)) as u64; rcx = (rsi) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rax) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rbx = (rax) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (rbp) as u64; r8 = (rdi) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001df7960;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); r9 = (crate::simd_ops::ru64(r8)) as u64; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; r11 = ((r10 as u32)) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rbp = ((P0) as u64) as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rdi))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (rcx & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 5; continue; }
            5 => { rdx = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = ((r11.wrapping_add(0x5u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfffffffcu32) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (r11) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; r11 = (r8) as u64; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rax as u32))) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsi, (r9) as u64); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rax = (0x80000000u32) as u32 as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; return rax; }
            12 => { rcx = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; fl_l = ((((r10 as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l; r10 = (r10 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 11; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001de8c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001df7960; const P1: u64 = 0x7001de8c87; const P2: u64 = 0x7001de8c88; const P3: u64 = 0x7001de8c89; const P4: u64 = 0x7001de8c8a; const P5: u64 = 0x7001de8c8b; const P6: u64 = 0x7001de8c8c; const P7: u64 = 0x7001de8c8d; const P8: u64 = 0x7001de8c8e; const P9: u64 = 0x7001de8c8f; const P10: u64 = 0x7001de8c94; const P11: u64 = 0x7001de8c95; const P12: u64 = 0x7001de8c96; const P13: u64 = 0x7001de8c97; const P14: u64 = 0x7001de8c98; const P15: u64 = 0x7001de8c99; const P16: u64 = 0x7001de8c9a; const P17: u64 = 0x7001de8c9b; const P18: u64 = 0x7001de8c9c; const P19: u64 = 0x7001de8c9d; const P20: u64 = 0x7001de8c9e; const P21: u64 = 0x7001de8c9f; const P22: u64 = 0x7001de8ca4; const P23: u64 = 0x7001de8ca5; const P24: u64 = 0x7001de8ca6; const P25: u64 = 0x7001de8ca7; const P26: u64 = 0x7001de8ca8; const P27: u64 = 0x7001de8ca9; const P28: u64 = 0x7001de8caa; const P29: u64 = 0x7001de8cab; const P30: u64 = 0x7001de8cac; const P31: u64 = 0x7001de8cad; const P32: u64 = 0x7001de8cae; const P33: u64 = 0x7001de8caf;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((rcx as u8)) as u32) as u32 as u64; rcx = ((P0) as u64) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u32) as u32 as u64; return rax; }
            2 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            12 => { rax = (rcx) as u64; return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            17 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            18 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            19 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            20 => { crate::lift_ops::trap(0x80000003, P17, rcx, 0); return rax; }
            21 => { crate::lift_ops::trap(0x80000003, P18, rcx, 0); return rax; }
            22 => { crate::lift_ops::trap(0x80000003, P19, rcx, 0); return rax; }
            23 => { crate::lift_ops::trap(0x80000003, P20, rcx, 0); return rax; }
            24 => { crate::lift_ops::trap(0x80000003, P21, rcx, 0); return rax; }
            25 => { rax = (rcx) as u64; return rax; }
            26 => { crate::lift_ops::trap(0x80000003, P22, rcx, 0); return rax; }
            27 => { crate::lift_ops::trap(0x80000003, P23, rcx, 0); return rax; }
            28 => { crate::lift_ops::trap(0x80000003, P24, rcx, 0); return rax; }
            29 => { crate::lift_ops::trap(0x80000003, P25, rcx, 0); return rax; }
            30 => { crate::lift_ops::trap(0x80000003, P26, rcx, 0); return rax; }
            31 => { crate::lift_ops::trap(0x80000003, P27, rcx, 0); return rax; }
            32 => { crate::lift_ops::trap(0x80000003, P28, rcx, 0); return rax; }
            33 => { crate::lift_ops::trap(0x80000003, P29, rcx, 0); return rax; }
            34 => { crate::lift_ops::trap(0x80000003, P30, rcx, 0); return rax; }
            35 => { crate::lift_ops::trap(0x80000003, P31, rcx, 0); return rax; }
            36 => { crate::lift_ops::trap(0x80000003, P32, rcx, 0); return rax; }
            37 => { crate::lift_ops::trap(0x80000003, P33, rcx, 0); return rax; }
            38 => { rax = (crate::simd_ops::ru64(rcx)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}
