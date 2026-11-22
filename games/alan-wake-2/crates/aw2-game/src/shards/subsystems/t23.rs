#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_36651<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x5u64) as u64); r9 = 0x5; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36652<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); r9 = 0x12; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(24u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = rsp.wrapping_add(0x38); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { r15 }) as u64; fl_l = rbx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } fl_l = rbx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r9 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15)) as u64; rax = (0x651e95c4d06fbfb1u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as u64; rdx = ((rdx).rotate_right(37u32)) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; r8 = ((r8).rotate_right(25u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64)))) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = r8; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; bb = 10; continue; }
            3 => { fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r8 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r15).wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(r15))) as u32) as u32 as u64; rax = (0xe16a3b2f90404f00u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(r15)) as u32) as u32 as u64; r8 = (0x9ae16a3b2f90404fu64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(r15).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax as u32)) as u32 as u64; rcx = (0xc3a5c85c97cb3127u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = rdx; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; bb = 10; continue; }
            5 => { rcx = (0x9ae16a3b2f90404fu64) as u64; bb = 10; continue; }
            6 => { r8 = (0x9ae16a3b2f90404fu64) as u64; r11 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; r9 = (((r9) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r10 = r11; r10 = (((r10) as i64).wrapping_mul((crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x8u64))) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(r15).wrapping_sub(0x10u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r8 = ((r8).rotate_right(18u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rax = ((rdx.wrapping_add(r9)) as u64) as u64; rax = ((rax).rotate_right(43u32)) as u64; r10 = ((r10).rotate_right(30u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) ^ (r8))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; bb = 10; continue; }
            7 => { r13 = ((rbx.wrapping_add(r15)) as u64) as u64; fl_l = rbx; fl_r = (0x40u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } r8 = (0x9ae16a3b2f90404fu64) as u64; r14 = ((r8.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = r14; rdi = (((rdi) as i64).wrapping_mul((crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = ((rax).rotate_right(30u32)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdx; rax = ((rax).rotate_right(43u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = ((r8).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rdx = ((r8.wrapping_add(rcx)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rdx = ((rdx).swap_bytes()) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = ((r9.wrapping_add(r10)) as u64) as u64; r9 = rax; r9 = ((r9).rotate_right(42u32)) as u64; r9 = ((r9).wrapping_add(r11)) as u64; rcx = ((rax.wrapping_add(r11)) as u64) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x8u64)))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; rax = ((rax).swap_bytes()) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x20u64)))) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(r13.wrapping_sub(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_sub(0x38u64))) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x10u64)))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; rdx = (0x9ddfea08eb382d69u64) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) ^ (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = r14; rax = ((rax).wrapping_shr(47u32)) as u64; r14 = (((r14) ^ (rax))) as u64; r14 = (((r14) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x40u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r9 = ((rdi.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(r14)) as u64; r8 = r10; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r13.wrapping_sub(0x38u64)))) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); r9 = ((rdi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r9); r11 = (crate::simd_ops::ru64(r13.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_sub(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_sub(0x20u64))) as u64; r12 = (0xb492b66fbe98f273u64) as u64; r10 = ((r10).wrapping_add(r12)) as u64; r10 = ((r10).wrapping_add(rbp)) as u64; r9 = ((r11.wrapping_add(r10)) as u64) as u64; r9 = ((r9).wrapping_add(rdi)) as u64; r13 = r10; rax = ((rax).wrapping_add(rcx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r9 = ((r9).rotate_right(21u32)) as u64; rax = r10; rax = ((rax).rotate_right(44u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r9)) as u64; r8 = ((r11.wrapping_add(r10)) as u64) as u64; rdi = (((rdi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r15))) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = (((rbx) & (0xffffffffffffffc0u64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rax = r15.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); r15 = rdi; bb = 9; continue; }
            9 => { r11 = (crate::simd_ops::ru64(rax.wrapping_sub(0x28u64))) as u64; rsi = ((r11.wrapping_add(rbp)) as u64) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rsi = ((rsi).rotate_right(37u32)) as u64; rsi = (((rsi) as i64).wrapping_mul((r12) as i64) as u64) as u64; rsi = (((rsi) ^ (r13))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).rotate_right(42u32)) as u64; rbp = (((rbp) as i64).wrapping_mul((r12) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64)))) as u64; r15 = ((r14.wrapping_add(r8)) as u64) as u64; r15 = ((r15).rotate_right(33u32)) as u64; r15 = (((r15) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = ((rsi.wrapping_add(r8)) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_sub(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_sub(0x30u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = ((r9.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = r8; rax = ((r10.wrapping_add(r11)) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rdx = ((rdx).rotate_right(21u32)) as u64; r11 = r8; r11 = ((r11).rotate_right(44u32)) as u64; r11 = ((r11).wrapping_add(rdx)) as u64; r11 = ((r11).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r11); rax = ((r9.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = ((r10.wrapping_add(rbp)) as u64) as u64; r9 = ((r15.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_sub(0x10u64)))) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; r13 = r9; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = ((r8).rotate_right(21u32)) as u64; rax = r9; rax = ((rax).rotate_right(44u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; r13 = ((r13).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(r9)) as u64) as u64; r14 = rsi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = (0x40u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } rcx = r13; rcx = (((rcx) ^ (r11))) as u64; r10 = (0x9ddfea08eb382d69u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; r13 = (((r13) ^ (rcx))) as u64; r13 = (((r13) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = r13; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = r8; rdx = (((rdx) ^ (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; r8 = (((r8) ^ (rdx))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = r8; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = rbp; rdx = ((rdx).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rbp))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rdx = (((rdx) ^ (rcx))) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rdx; rax = ((rax).wrapping_shr(47u32)) as u64; rdx = (((rdx) ^ (rax))) as u64; rcx = (((rcx) ^ (rdx))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rax = (0xde5fb9d2630458e9u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P4))) as u64; rcx = P6; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36663<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36666<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64;
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rdx = (crate::simd_ops::ru64(P2)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (((rcx as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36673(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36677(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36739<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36740<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { r12 = (crate::simd_ops::ru64(rbx)) as u64; rsi = r12; rsi = ((rsi).wrapping_shr(32u32)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 2; continue; }
            2 => { rbp = ((r12 as u32)) as u32 as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; r15 = ((r13 as u32)) as u32 as u64; r15 = ((r15).wrapping_shl(6u32)) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rcx); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(r15.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(r15.wrapping_add(0x28u64), rax); r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rdx = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = r15.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rcx); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rcx); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (0x7u64) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (0x8u64) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x38u64), rax); r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36742<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rbx = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36768(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xa0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36792<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    r8 = rsp.wrapping_add(0x20);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = P1;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x70);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(P3)) as u128;
    rbx = ((rbx).wrapping_add(0x8u64)) as u64;
    r8 = rsp.wrapping_add(0x20);
    rdx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36794(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36820<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x70); rax = P2; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P4)) as u128; r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P9)) as u128; r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x20); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36822<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (((crate::simd_ops::ru64(rcx)) as i64).wrapping_mul((0x44u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(rax).wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36828<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x48; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36829<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = 0x48;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P1;
    crate::simd_ops::wu64(rax, rcx);
    let y_addr: u64 = rbx.wrapping_add(P2); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(P2); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rbx.wrapping_add(P3); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36830<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0xffffffffffffffffu64) as u64); rdx = ((rdx).wrapping_shr(32u32)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let y_addr: u64 = rcx.wrapping_add(P1); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm0 = (ymm1_hi) as u128; xmm1 = (ymm1_hi) as u128; ymm1_hi = 0; rax = (((xmm1 >> 64) as u64)) as u64; rcx = ((xmm0 as u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P2))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x48u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36831<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36864<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = r9; r12 = r8; r14 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } r13 = P1; bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rbp = rsi; rbp = ((rbp).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } rdi = P2; bb = 2; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((r12.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(r15.wrapping_add(rcx.wrapping_mul(8)), ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = ((rcx.wrapping_add(P3)) as u64) as u64; rcx = ((r15.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r9 = 0x2; r8 = ((r9.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36865<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx, ((rax as u8)) as u8);
    rdx = ((r9.wrapping_add(P0)) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r9 = 0x2;
    r8 = ((r9.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36866<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    loop { r9 = P1; rdx = 0x10; r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36903<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36914<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    r9 = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rdx = ((rdx).wrapping_add(0x4u64)) as u64;
    r8 = ((rcx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36917<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rcx = rbx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x6u32) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64;
    rax = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36932<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rdx).wrapping_shl(7u32)) as u64;
    rdx = ((fl_l as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax).wrapping_add(P1))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36935<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36968<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rsi = rcx; r8 = ((r9.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0))) as u64; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = 0u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 1; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(P6))) as u128; let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { r11 = rsp.wrapping_add(0x70); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36972<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36973<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P4))) as u128; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36976(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu16(rcx, ((rdx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36980<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_36992<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37005<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rdi = rdx;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = ((rcx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37015<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x47); rcx = rbp.wrapping_add(0x27); r11 = (crate::simd_ops::ru64(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x8u64))) as u128; r11 = ((r11).wrapping_add(0x40u64)) as u64; rdx = r11; crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x27u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64) as u128; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); bb = 3; continue; }
            3 => { rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9).wrapping_add(0x49u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 5; continue; }
            5 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } fl_l = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdx); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            9 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 11; continue; }
            10 => { rax = ((rbx as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); bb = 12; continue; }
            12 => { rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37040<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rsi; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rsi); rdi = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rbp = rbx.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); rsi = rbx.wrapping_add(0x170); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } rax = rdi; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r14 = 0x1; bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (r14) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); rbx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37078<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    rcx = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    r9 = P1;
    rcx = P2;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x1;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37079<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37080<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37081<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = r8;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37083<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P0), rax);
    rax = (tmp_xchg) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37085<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } loop { rax = 0; fl_l = 0; fl_r = 0; rcx = r8; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as i64) as u64) as u64; r8 = ((r8).wrapping_add(1)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(rdx).wrapping_add(P2), rax); rax = (tmp_xchg) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_l = r8; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } } rcx = rdx; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37086<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; rcx = rdx; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(P0), rax); rax = (tmp_xchg) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37107<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37137<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r10 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = r10;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37138<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); r14 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rcx = r8; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0xfu64))) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x14); rbp = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x100u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rbp as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x118u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), ((rax as u8)) as u8); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37142<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn tpl_37143<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128;
    let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm1 as u64)) as u64);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37144<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37159<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37173<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    crate::simd_ops::wu32(rdx, ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37178<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    xmm2 = 0u128;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { r8 = (crate::simd_ops::ru64(rcx)) as u64; xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128; loop { rcx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8).wrapping_add(P2))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = rdx; fl_r = rax; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } } xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37179<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    loop { rcx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_add(P1)))) as u32 as u64; fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37180<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    loop { rcx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (crate::simd_ops::ru8(rcx.wrapping_add(r9).wrapping_add(P1))))) as u8 as u64)); fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37181<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    xmm0 = 0u128;
    fl_l = rdx; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    loop { rcx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8).wrapping_add(P2))) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = rdx; fl_r = rax; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37183<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(rdx)) as u64;
    xmm0 = 0u128;
    fl_l = rdx; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    loop { rcx = (((crate::simd_ops::ru64(rdx)) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r8).wrapping_add(P1)))).to_bits() as u128); fl_l = rdx; fl_r = rax; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37191<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = rcx.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); rax = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_sub(0x40); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x58u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (0x2aaaaaaaaaaaaaabu64) as u64; r13 = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P12)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x40); rdx = rbp.wrapping_sub(0x60); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; r14 = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P14)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r15, ((rdi as u16)) as u16); bb = 6; continue; }
            6 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((r14 as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37192<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = r8; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rbx = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu16(rcx, ((rdx as u16)) as u16); r11 = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P2), r8); crate::simd_ops::wu64(rcx.wrapping_add(P3), r9); r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdi = (((rdx as u16)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(P6))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rax = ((rdi.wrapping_add(P8)) as u64) as u64; fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 2; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi.wrapping_add(rcx)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = r11; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37201<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0xb0u64)) as u64;
    rbx = rcx;
    rax = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x10u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x10u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsp.wrapping_add(0x20);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0xb0u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37202<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0xb0u64)) as u64;
    rbx = rcx;
    rax = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x10u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x10u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    rcx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm3 = 0u128;
    rdx = rsp.wrapping_add(0x20);
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0xb0u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37203<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0xb0u64)) as u64;
    rbx = rcx;
    rax = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x10u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x10u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    rcx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    rdx = rsp.wrapping_add(0x20);
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0xb0u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37205<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r14 = rdx.wrapping_add(0x8); rsi = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; rbx = rax.wrapping_add(0xc); r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r12 = P0; fl_l = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; rbp = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rdx = ((rsi.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = r14; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(r14)) as u64; bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(rbx.wrapping_sub(0x5u64))) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } r9 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { r9 = (((r9 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x7c00; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rdi = rbx; r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } rdi = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; fl_l = r9; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } fl_l = rdi; fl_r = rdx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u32) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(r12))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(r12))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 9; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 9; continue; } rdx = ((rdx).wrapping_add(1)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 11; continue; }
            9 => { rax = ((rax).wrapping_add(0xf0u64)) as u64; rbx = ((rbx).wrapping_add(0xf0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37213(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x555555555555555u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37227<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37228<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37231<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x24u64) as u64); r9 = 0x24; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37233<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; r15 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xcu64) as u64); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); r14 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = r11; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rbx as u32)) as u32 as u64; fl_l = r11; fl_r = rcx; r10 = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { r10 }) as u64; r9 = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rdx; fl_r = r10; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { r10 }) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); r8 = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = r11; rax = ((rax).wrapping_sub(r9)) as u64; rdx = rbx; fl_l = r11; fl_r = r9; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); fl_l = rdx; fl_r = rdx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rdx }) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rbp.wrapping_sub(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x100u64) as u64); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu8(rcx, (0x0u8) as u8); rdx = (0xffffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rcx = rsi.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x22u64) as u64); r9 = 0x22; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x190u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37235<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rcx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37236<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } fl_l = r10; fl_r = rcx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = P3; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 6; continue; } rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37237<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((rdx.wrapping_add(P0)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } fl_l = r10; fl_r = rcx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r11 = P4; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rcx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_sub(1)) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 6; continue; } rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37246<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37254<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rdx as u8)) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37287<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37291<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P0)))).to_bits() as u128);
    xmm4 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    xmm2 = ((crate::simd_ops::ru32(r9)) as u32) as u128;
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128);
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    xmm3 = 0u128;
    xmm3 = crate::simd_ops::vblendps(xmm3, xmm1, 0x1u8);
    xmm3 = crate::simd_ops::vinsertps(xmm3, xmm4, 0x10u8);
    xmm3 = crate::simd_ops::vinsertps(xmm3, ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u128), 0x20u8);
    crate::simd_ops::wu128(rcx, (xmm3) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37297<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r15 = r8; let y_addr: u64 = r9; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rdx)) as u128; ymm1_hi = 0; rdi = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x27); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; r12 = rcx; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rsi = r9; let y_addr: u64 = rbp.wrapping_add(0x27u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rdx = rbp.wrapping_sub(0x29); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = rbp.wrapping_sub(0x9); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rbx); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x9u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; r8 = rbp.wrapping_add(0x27); xmm1 = (crate::simd_ops::ru128(r15)) as u128; ymm1_hi = 0; let y_addr: u64 = rbp.wrapping_add(0x27u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rdx = rbp.wrapping_sub(0x29); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; rcx = rbp.wrapping_sub(0x9); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rbx); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x9u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x28u64))) as u64; r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(rax), (0x1u8) as u8); rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37318<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x30); rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37319<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37327<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37328<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37356<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37360<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; return rax; } rax = P0; return rax; } rax = P1; return rax; } rax = P2; return rax; }
    rax = P3;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37397<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(P3), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P2), rbp); rdi = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rsi = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(P6), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P1), (1) as u64); rbx = ((rbx).wrapping_add(0x170u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(P1), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(P6), (1) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P1), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37399<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37417<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru8(P2)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbx = rax.wrapping_add(0x2d40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x2d58u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; r8 = rdi; rdx = (crate::simd_ops::ru64(P0)) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(P8, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(P9, rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(P10, (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = P11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(P13, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(P14, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(P15, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(P16, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(P17, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(P18, rax); crate::simd_ops::wu8(P2, (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb8u64))) as u32) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } rcx = rsp.wrapping_add(0x60); bb = 9; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbx = rax.wrapping_add(0x2d40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x2d58u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(P8)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x4fc0u64))) as u64; fl_l = (crate::simd_ops::ru64(P9)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } rax = P19; fl_l = (crate::simd_ops::ru64(P18)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } rbx = (crate::simd_ops::ru64(P8)) as u64; bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 10; continue; }
            8 => { rax = P6; crate::simd_ops::wu64(P18, rax); crate::simd_ops::wu64(P17, rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = P8; rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } rcx = rsp.wrapping_add(0x88); bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37425<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r9 = ((crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u32) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64; rcx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (((crate::simd_ops::ru64(rax.wrapping_add(P5))) as i64).wrapping_mul((0x138u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P7), ((rbx as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37426<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    r9 = ((crate::simd_ops::ru16(rdx.wrapping_add(r8.wrapping_mul(2)))) as u32) as u32 as u64;
    rdx = rsp.wrapping_add(0x38);
    r9 = ((r9).wrapping_add(r9)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))) as u64;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_r = rcx;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (((crate::simd_ops::ru64(rax.wrapping_add(P5))) as i64).wrapping_mul((0x138u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P7))) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37447(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37460<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm4 = ((crate::simd_ops::ru64(r8.wrapping_add(0x24u64))) as u64) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); rdi = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x2cu64))) as u32 as u64; rdx = rdi; xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x24u64))) as u64) as u128; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu64(rax.wrapping_sub(0x48u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); rbx = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x58u64), ((xmm4 as u64)) as u64); xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm2 = (((rcx as u32)) as u32) as u128; xmm1 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((xmm1 as u32)) as u32); xmm3 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_sub(0x54u64)))).to_bits() as u128); xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_sub(0x44u64)))).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64)))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = rax.wrapping_sub(0x58); crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm4 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x24u64)))).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))).to_bits() as u128); xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x20u64)))).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64)))).to_bits() as u128); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm3 = (xmm4) ^ (xmm0); xmm4 = (xmm5) ^ (xmm0); xmm5 = (xmm7) ^ (xmm0); xmm1 = (xmm8) ^ (xmm0); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm4 as u32)) as u32); bb = 1; continue; }
            1 => { r9 = rsp.wrapping_add(0x30); xmm2 = (xmm6) as u128; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37480<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    let y_addr: u64 = rdx.wrapping_add(r8.wrapping_mul(4)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37501<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x1; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37507<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    r9 = rsp.wrapping_add(0x50);
    r8 = rcx;
    rcx = rsp.wrapping_add(0x48);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rax, rdi);
    crate::simd_ops::wu64(rbx, rcx);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37528<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37596<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37607<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } crate::simd_ops::wu32(rax, (0x2u32) as u32); bb = 2; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } crate::simd_ops::wu32(rax, (0x3u32) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(P6), (0x0u64) as u64); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x0u8) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r9 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; r8 = P10; rdx = (((rax) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P11)))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37608<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rax; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = ((crate::simd_ops::ru8(rax.wrapping_add(P5))) as u32) as u32 as u64; bb = 2; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(P7), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; r9 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; r8 = P10; rdx = (((rax) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P11)))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), (0x1u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), (0x3f000000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x3f000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x44u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37610<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rdx; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; xmm3 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0x4; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = 0x3; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37611<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37612<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37613<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P0), rdi); xmm3 = 0u128; r8 = rsi; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rdx = 0x1; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37615<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); rax = P2; crate::simd_ops::wu64(rdi, rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P3), rcx); rax = P4; crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P6), rcx); crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x70u64), ((rax as u8)) as u8); rax = P12; crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2cu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x84u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x85u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); rax = P13; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); rax = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37617<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(5u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = ((r9.wrapping_add(P4)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); if (((fl_l as u32)) != 0) { bb = 5; continue; } rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            2 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rsi); rdi = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37622<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37625<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    rax = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37646(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37647<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x60; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37673<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((rdx as u32)) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37677<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; ymm0_hi = 0;
    let y_addr: u64 = rcx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r9);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37678<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::wu16(rcx, ((rdx as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37679<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37680<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37681<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37682<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x0u64) as u64);
    let y_addr: u64 = rdx.wrapping_add(P3); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P4); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P6))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37683<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rcx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    let y_addr: u64 = rcx.wrapping_add(P2); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rdx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P3))) as u128; ymm0_hi = 0;
    let y_addr: u64 = rdx.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = rdx;
    crate::simd_ops::wu128(rdx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), (0x0u64) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37689(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37718<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37719(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rdx, ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37728<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = rdx;
    rdx = 0x8;
    fl_l = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37729(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0xfffffffffffffff0u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37730<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37731(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x10u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37732(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    rax = rdx;
    fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37733(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x10u64) as u64)) as u64; fl_r = fl_l;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37758<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_shl(6u32)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37765<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37766<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = rcx;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(5u32)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64;
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r11 = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = 0x3;
    fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x4;
    rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37767<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P4)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37768<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37769<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P9), rax);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37770<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37771<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = rdx; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; xmm3 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0x4; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = 0x3; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            4 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37773<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = 0x2; xmm3 = 0u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37774<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P0), rdi); xmm3 = 0u128; r8 = rsi; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rdx = 0x1; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37776<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = P4;
    crate::simd_ops::wu64(rcx, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37790<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    let y_addr: u64 = rcx.wrapping_add(P1); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; ymm0_hi = 0;
    rax = P3;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm1) as u128);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm0 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37791<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rcx.wrapping_add(P3)) as u64) as u64;
    r9 = ((rcx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37792<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = ((rcx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    r8 = rsp.wrapping_add(0x30);
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37793<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    r9 = ((rcx.wrapping_add(P3)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = rsp.wrapping_add(0x30);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37794<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37798<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm5 = (crate::simd_ops::ru128(r9)) as u128; xmm4 = (crate::simd_ops::ru128(r8)) as u128; xmm3 = crate::simd_ops::vdpps(xmm5, xmm5, 0x7fu8); xmm1 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 1; continue; } if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vdpps(xmm5, xmm4, 0x7fu8); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = crate::simd_ops::vbroadcastss((xmm1 as u32)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37815<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37825<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rsi = rdx;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37852<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = r12; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r12); rdi = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rsi = rbx.wrapping_add(0x170); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); fl_l = (crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; rbp = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } rax = rdx; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r15 = 0x1; bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdx); rdi = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x158); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); rbx = ((rbx).wrapping_add(0x170u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37864<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = rcx;
    rax = ((r8 as u32)) as u32 as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rdx = r9;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37868<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((r8 as u32)) as u32 as u64;
    r8 = P0;
    fl_l = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx), r8);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P2), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37920<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37935<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x0u32) as u32);
    rax = 0x1;
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (0x18u64) as u64)) as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37952<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = rax; fl_r = rdx; rcx = r8; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 1; continue; } rcx = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            4 => { r8 = rsp.wrapping_add(0x50); rcx = rdi; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rcx = (crate::simd_ops::ru64(r15)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 7; continue; }
            6 => { rcx = rsi; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (rsi & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x38u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); return rax; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            10 => { r8 = rsp.wrapping_add(0x50); rcx = rdi; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rcx = (crate::simd_ops::ru64(r15)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 7; continue; }
            12 => { rcx = rsi; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (rsi & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rax = 0x1; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rsp = ((rsp).wrapping_add(0x38u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37953<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = 0x1; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rsp.wrapping_add(0x70); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { r8 = P8; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            8 => { r8 = rsp.wrapping_add(0x70); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { r8 = P8; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37972<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rax); r8 = r11.wrapping_sub(0x68); rdx = r11.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_37999<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P9), ((rax as u16)) as u16);
    rax = P10;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38004<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38006<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((rdx.wrapping_mul(8)) as u64) as u64; rax = rbx; r11 = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rdx = 0x8; rdx = ((rdx).wrapping_sub(rax)) as u64; r10 = rcx; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdx }) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rax = rcx; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rax = ((rax).wrapping_sub(r9)) as u64; rcx = ((rdx.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); rax = ((rcx.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } crate::simd_ops::wu64(r10.wrapping_add(P2), rcx); crate::simd_ops::wu64(r10.wrapping_add(P3), r11); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            1 => { fl_l = rcx; fl_r = r9; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), r9); crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38007<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38008<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38020<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38021<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu16(rdx.wrapping_add(P7), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = P9;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38022<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38026<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38027<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38028<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = (((rdx as u8)) as u32) as u32 as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (((rsi as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38040<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rdx;
    r9 = r8;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = ((r8).wrapping_add(0x30u64)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P3, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38042<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38047<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38048<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38085<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r8;
    r10 = rdx;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_add(0x20u64)) as u64;
    r9 = rsp.wrapping_add(0x20);
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38103<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r9);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38114<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = (((rcx) & (r8))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rax }) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38115<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r10 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38123<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38136<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128);
    xmm2 = 0u128;
    xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128);
    xmm4 = 0u128;
    fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8); }
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38157<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((rbx.wrapping_add(P8)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38159<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; xmm4 = (crate::simd_ops::ru128(rdx)) as u128; xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm2 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm2, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm2 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(P0)); crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rdx = rax.wrapping_sub(0x78); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); rcx = rsp.wrapping_add(0x20); xmm8 = (crate::simd_ops::ru128(P1)) as u128; xmm3 = crate::simd_ops::vsubps(xmm8, xmm1); xmm1 = (crate::simd_ops::ru128(r8)) as u128; xmm0 = crate::simd_ops::vmulps(xmm3, xmm2); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = crate::simd_ops::vmulps(xmm0, xmm4); xmm0 = crate::simd_ops::vshufps(xmm1, xmm1, 0xd2u8); xmm1 = crate::simd_ops::vshufps(xmm1, xmm1, 0xc9u8); xmm5 = crate::simd_ops::vpermilps(xmm9, 0xc9u8); xmm3 = crate::simd_ops::vmulps(xmm0, xmm5); xmm6 = crate::simd_ops::vpermilps(xmm9, 0xd2u8); xmm2 = crate::simd_ops::vmulps(xmm1, xmm6); xmm4 = crate::simd_ops::vsubps(xmm2, xmm3); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm2 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm2, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm2 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(P0)); xmm3 = crate::simd_ops::vsubps(xmm8, xmm1); xmm0 = crate::simd_ops::vmulps(xmm3, xmm2); xmm4 = crate::simd_ops::vmulps(xmm0, xmm4); xmm1 = crate::simd_ops::vpermilps(xmm4, 0xc9u8); xmm2 = crate::simd_ops::vmulps(xmm1, xmm6); xmm1 = 0u128; xmm0 = crate::simd_ops::vpermilps(xmm4, 0xd2u8); xmm3 = crate::simd_ops::vmulps(xmm0, xmm5); xmm5 = crate::simd_ops::vsubps(xmm3, xmm2); crate::simd_ops::wu32(rax.wrapping_sub(0x6cu64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x68u64), (((xmm5 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x64u64), (((xmm5 >> 64) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x78u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x74u64), (((xmm4 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x70u64), (((xmm4 >> 64) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x60u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x5cu64), (((xmm9 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x58u64), (((xmm9 >> 64) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x54u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x50u64), (((xmm1 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4cu64), (((xmm1 >> 64) as u32) as u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x24u64))) as u64) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u128), 0x20u8); xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u128), 0x30u8); r11 = rsp.wrapping_add(0xa0); rax = rbx; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rsp = r11; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38168<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } rcx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            1 => { rcx = 0x1; bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(r8.wrapping_add(P3), (rcx) as u64); crate::simd_ops::wu64(r8.wrapping_add(P2), rbp); rdi = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rsi = ((r8.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P0), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(P1), (1) as u64); crate::simd_ops::lk_add_u64(rsi.wrapping_add(P0), (1) as u64); rbx = ((rbx).wrapping_add(0x170u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(P0), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(P1), (1) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(P0), (1) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38185<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38186<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx.wrapping_mul(8)), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38188<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38189<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = ((r8 as u32)) as u32 as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rbp = rdx;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38191<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = (xmm2) as u128;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; xmm2 = (xmm6) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38218<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rsi = rcx;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(5u32)) as u64;
    rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64;
    fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = rsi; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rax; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38248<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38249<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38250<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38275<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((xmm0 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38294<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(P5), (0x0u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38295<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(P5), (0x0u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38296<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; rdx = 0x2; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38297<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38298<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(0x60u64), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38313<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = rdx; rax = (((xmm0 >> 64) as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsi; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0x2; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38429<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu64(r8.wrapping_add(P4), rsi); bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = rdx; rdx = 0x2; xmm3 = 0u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38430<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = P3; crate::simd_ops::wu64(rdi.wrapping_add(P1), rdi); xmm3 = 0u128; r8 = rsi; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); rdx = 0x1; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38431<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rdi, rax); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = P12; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x70u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32); rax = P13; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0xb0u64), (0x1u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0xb4u64), ((rax as u8)) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38434<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rbp = 0xffff; r15 = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r15; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 2; continue; }
            2 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x5au64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r15; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x5au64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38436(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38459<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = 0u128; r8 = rdi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rcx = (((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P4), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38484<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38485<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); } else { crate::simd_ops::wu64(rdx, (0x0u64) as u64); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38486<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = rdi; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rsi.wrapping_add(P1))) as i64) as u64) as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(rsi))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(rsi), rbp); rdi = ((rdi).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 4; continue; }
            4 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38487<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38492<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; rbx = rdx; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; rdi = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8cu64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8cu64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38493<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(P0), (xmm1) as u128);
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P1), ((xmm0 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38494<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P0), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P1), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38498<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64) as u128;
    xmm1 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rdx), (xmm1) as u128);
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P3), ((rcx as u32)) as u32);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P4), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38499<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128;
    xmm1 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rcx = ((rdx.wrapping_add(P2)) as u64) as u64;
    rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    r9 = rdx;
    crate::simd_ops::wu128(rax.wrapping_add(rdx), (xmm1) as u128);
    crate::simd_ops::wu64(rax.wrapping_add(rdx).wrapping_add(P0), ((xmm0 as u64)) as u64);
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P1), ((rdx as u32)) as u32);
    rcx = ((r9.wrapping_add(P3)) as u64) as u64;
    xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(rax), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P1), ((rdx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38500<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P0), (xmm1) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38510<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38513<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P4)) as u64) as u64;
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P6), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x74u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38518<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = P3;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38522<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38523<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38525<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38526<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38528<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x28u64)) as u64;
    rcx = P3;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38530<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38532<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    rcx = P3;
    r8 = 0x4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38534<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38535<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38536<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38537<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38538<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    r10 = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rax = rdx;
    rcx = P3;
    rdx = ((rdx).wrapping_add(0x34u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r8 = 0x2;
    rcx = r10;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P5)) as u64) as u64;
    r8 = 0xb;
    rax = P6;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P7;
    rax = P8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = P9;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38542<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38543<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rcx;
    r9 = P0;
    r10 = rdx;
    r8 = 0x2;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = r10;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rax.wrapping_add(P6)) as u64) as u64;
    r8 = 0xb;
    rax = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    r9 = P8;
    rax = P9;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = P10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38544<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu128(rax.wrapping_add(rcx).wrapping_add(P0), (xmm1) as u128);
    crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P1), ((xmm0 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38545<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rcx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r9).wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38546<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38548<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    r8 = 0x5;
    rcx = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P2;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38549<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r9 = rdx;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r9))) as u64;
    fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38551<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0x4;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38557<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38559<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = r8; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38561<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38566<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38569<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rcx = ((r8.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rdi.wrapping_add(P2)) as u64) as u64;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38571<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P3), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P3), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rdx.wrapping_add(P4), (0x8u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P6), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x74u64), (0x4u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_38582<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = rdx;
    r11 = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = r11.wrapping_add(0x98);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r9 = P3;
    rcx = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
