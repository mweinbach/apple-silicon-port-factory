#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_71307<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64, const P20: u64, const P21: u64, const P22: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 167>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x510u64)) as u64; rsi = r9; r14 = r8; r11 = rdx; r15 = rcx; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x560u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 51; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; r10 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = r9; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r10 = rax.wrapping_add(0x10); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 52; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r10); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 8; continue; }
            8 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x548u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x560u64))) as u32 as u64; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(r15.wrapping_add(0x50u64))) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r13 = (0xffffffffffffu64) as u64; rcx = (((rcx) & (r13))) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x4u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r10); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); bb = 22; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { r9 = rsi; r8 = P1; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = r15.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x168u64), (0x0u8) as u8); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x164u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x168u64), ((rsi as u8)) as u8); bb = 18; continue; }
            15 => { r9 = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x38u64), (1) as u32); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            21 => { r13 = (0xffffffffffffu64) as u64; bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = r12; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = rsi; r8 = P9; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r15.wrapping_add(0x20); rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            24 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), ((rdi as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rdi); r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xd4u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; fl_l = ((((rax as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xdbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xdcu64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xe0u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); r14 = rdi; bb = 25; continue; }
            25 => { fl_l = r14; fl_r = r12; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 36; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((r14 as u32)) as u32); rsi = r14; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(r11)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x168u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x270u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x278u64), (0x0u8) as u8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = rsp.wrapping_add(0x278); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            28 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (r13))) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; r9 = (crate::simd_ops::ru64(r9)) as u64; r8 = P10; rdx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { r9 = P12; r8 = P13; rdx = ((crate::simd_ops::ru8(P14)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x400); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x400); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x274u64))) as u32 as u64; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x273u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 32; continue; }
            31 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x270u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = rsp.wrapping_add(0x278); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x278u64)) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rcx = rsp.wrapping_add(0x270); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x164u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = rsp.wrapping_add(0x160); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 35; continue; }
            35 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, rbx); rcx = rsp.wrapping_add(0x160); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r14).wrapping_add(1)) as u64; r14 = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xd4u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x560u64))) as u32 as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x548u64))) as u64; bb = 25; continue; }
            36 => { bb = 38; continue; }
            37 => { rdi = 0; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x540u64))) as u64; bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 50; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x2f8u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x300u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x2f8); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 41; continue; }
            41 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 44; continue; }
            43 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x2f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r15.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x20u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000020u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x20u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x20u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x2f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x28u64), (1) as u32); rcx = rsp.wrapping_add(0x2f8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 50; continue; }
            50 => { rcx = rsp.wrapping_add(0xd8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { r11 = rsp.wrapping_add(0x510); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { r8 = rsi; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P22, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71324<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64, const P19: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 95>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1c0); rsp = ((rsp).wrapping_sub(0x2c0u64)) as u64; rdi = rdx; r13 = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; r14 = 0; fl_l = 0; fl_r = 0; r15 = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; r10 = (0xffffffffffffu64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (((rcx) & (r10))) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = rdx; r12 = (crate::simd_ops::ru64(r8)) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x40u64))) as u64; rsi = P1; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 17; continue; } bb = 9; continue; }
            9 => { r14 = rax.wrapping_add(0x10); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 71; continue; } bb = 11; continue; }
            11 => { r10 = (0xffffffffffffu64) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (0x20000020u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xb8u64), ((r15 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x5000005u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = (((r9) & (r10))) as u64; rsi = (crate::simd_ops::ru32(r14)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; rax = 0x1000; r11 = P3; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = ((rax as u32)) as u32 as u64; fl_l = ((((rdx as u16)) & (crate::simd_ops::ru16(r9.wrapping_add(0xeu64))))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (((crate::simd_ops::ru8(r9.wrapping_add(0xdu64))) as i8 as i32) as u32) as u32 as u64; rcx = 0xd; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 21; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 17; continue; }
            17 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r9 = rsi; r8 = P4; rdx = ((crate::simd_ops::ru8(P5)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = r13.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x10000010u64) as u64); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x58u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x50); rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r13.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r13.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u32(r13.wrapping_add(0x38u64), (1) as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x60); bb = 52; continue; }
            20 => { rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xd; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = r9; bb = 25; continue; }
            24 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (((rdx) & (r10))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { r10 = P12; fl_l = r10; fl_r = rdx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = rdx.wrapping_add(0x1); rax = ((crate::simd_ops::ru8(rax.wrapping_add(r11))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 28; continue; }
            28 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { r10 = (0xffffffffffffu64) as u64; bb = 30; continue; }
            30 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x20u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 31; continue; }
            31 => { rax = 0x1000; bb = 14; continue; }
            32 => { r15 = ((rsi as u32)) as u32 as u64; rdi = ((rbx as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru16(r9.wrapping_add(0x1eu64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = r9.wrapping_add(0x10); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { r9 = r12; r8 = P12; rdx = rbp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x60u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000020u32))) as u32 as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x5du64), (0x20u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), (0x0u32) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x58u64), (0x0u8) as u8); r9 = rbp.wrapping_sub(0x60); r8 = rbp.wrapping_add(0xb0); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { r10 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = (0xffffffffffffu64) as u64; r10 = (((r10) & (r8))) as u64; rbx = (crate::simd_ops::ru32(r14)) as u32 as u64; r14 = P15; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; r15 = 0x1000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 38; continue; }
            38 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 54; continue; } bb = 39; continue; }
            39 => { rdx = ((rdi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (((rdx) & (r8))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 41; continue; }
            41 => { r9 = r14; fl_l = rdx; fl_r = r14; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rsi = P3; bb = 42; continue; }
            42 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rax = ((crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 44; continue; }
            44 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 45; continue; }
            45 => { r9 = ((rdi as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; fl_l = ((r9).wrapping_add(r10)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 46; continue; }
            46 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru16(r9.wrapping_add(0x1eu64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rcx = r9.wrapping_add(0x10); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { r9 = r12; r8 = r14; rdx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000005u32))) as u32 as u64; rax = ((((rax as u32)) | (0x5u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), (0x5u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); r9 = rsp.wrapping_add(0x20); r8 = rsp.wrapping_add(0x70); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0xb0); bb = 52; continue; }
            52 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x300u64))) as u64; rsp = ((rsp).wrapping_add(0x2c0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            54 => { rsi = P3; bb = 56; continue; }
            55 => { r8 = (0xffffffffffffu64) as u64; bb = 56; continue; }
            56 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 50; continue; } bb = 57; continue; }
            57 => { r9 = r10; bb = 58; continue; }
            58 => { rdx = ((r15 as u32)) as u32 as u64; fl_l = ((((rdx as u16)) & (crate::simd_ops::ru16(r9.wrapping_add(0xeu64))))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = (((crate::simd_ops::ru8(r9.wrapping_add(0xdu64))) as i8 as i32) as u32) as u32 as u64; rcx = 0xd; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 61; continue; }
            60 => { rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; bb = 61; continue; }
            61 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 62; continue; }
            62 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rdx = r9; bb = 65; continue; }
            64 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = (((rdx) & (r8))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 65; continue; }
            65 => { r10 = r14; fl_l = r10; fl_r = rdx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 66; continue; }
            66 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdx = rdx.wrapping_add(0x1); rax = ((crate::simd_ops::ru8(rax.wrapping_add(rsi))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 66; continue; } bb = 68; continue; }
            68 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 69; continue; }
            69 => { r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x20u64)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 50; continue; } bb = 70; continue; }
            70 => { r8 = (0xffffffffffffu64) as u64; bb = 58; continue; }
            71 => { r8 = rsi; rdx = r12; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P19, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71382<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = P3; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P7)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r10 = ((rsi.wrapping_add(P9)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = r10; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71384<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = P3; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P7)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71593<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64;
    rdx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71725<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rbx.wrapping_add(P6)) as u64) as u64; bb = 4; continue; }
            2 => { rdi = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rdx = ((rsi.wrapping_add(P9)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdx, (0x0u32) as u32); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71824<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r15 = rdx; r13 = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 121; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; rsi = (0x333333333333333u64) as u64; r12 = 0x1000; r14 = (0xffffffffffffu64) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (((r8) & (r14))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r8); rax = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x21u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x77); rcx = rbp.wrapping_sub(0x11); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rdi as u32)) as u32); rcx = rbx; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 12; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x1u32) as u32); rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 122; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::lk_and_u32(r8.wrapping_add(0x20u64), (0xff000005u32) as u32); crate::simd_ops::lk_or_u32(r8.wrapping_add(0x20u64), (0x5u32) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x23u64), (0x5u8) as u8); crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu64(r8, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rbx); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x11); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 17; continue; }
            17 => { rdx = rcx.wrapping_add(0x20); rcx = r13.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r14))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { r8 = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r13)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r13, r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 20; continue; }
            20 => { bb = 121; continue; }
            21 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); r15 = 0; fl_l = 0; fl_r = 0; rsi = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r15); rdi = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); r14 = ((r15 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rax = ((rdi as u32)) as u32 as u64; r15 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r15; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { r15 = ((r15).wrapping_sub(rsi)) as u64; rdi = r15; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(rsi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rcx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; r8 = r15; rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rsi = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((r14 as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 27; continue; }
            27 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(rax.wrapping_mul(8)), rbx); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 31; continue; }
            31 => { rbx = rax; bb = 35; continue; }
            32 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 36; continue; }
            36 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            37 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r13)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 67; continue; } bb = 38; continue; }
            38 => { r12 = 0x1000; r15 = (0xffffffffffffu64) as u64; bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (((rcx) & (r15))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rcx); rax = (0xffffffffffffffffu64) as u64; bb = 42; continue; }
            42 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 42; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x41u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x77); rcx = rbp.wrapping_sub(0x11); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx; r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 49; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 48; continue; }
            47 => { rbx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 62; continue; } bb = 50; continue; }
            50 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 62; continue; } bb = 51; continue; }
            51 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 62; continue; } bb = 52; continue; }
            52 => { rdx = rbx.wrapping_add(0x20); rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 54; continue; }
            54 => { rcx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r13.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rcx = ((rdi as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 65; continue; } bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 59; continue; } bb = 56; continue; }
            56 => { rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rdx); bb = 58; continue; }
            58 => { fl_l = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdi as u32)) as u32); bb = 60; continue; }
            59 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 60; continue; }
            60 => { rcx = ((rdi as u32)) as u32 as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 61; continue; }
            61 => { bb = 65; continue; }
            62 => { rcx = rbp.wrapping_sub(0x21); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu32(rax, ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 64; continue; }
            64 => { rdx = ((rdx).wrapping_add(0x4u64)) as u64; rcx = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r13.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(r13)) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r13, rcx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 66; continue; }
            66 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 67; continue; }
            67 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x41u64), (xmm0) as u128); r13 = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x0u64) as u64); r14 = ((r15 as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rdi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdi); rax = rdi; rax = ((rax).wrapping_sub(rsi)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { r14 = r15; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; r14 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(4)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((r14 as u32)) as u32); bb = 69; continue; }
            69 => { rbx = rsi; rcx = ((r15 as u32)) as u32 as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 77; continue; } bb = 70; continue; }
            70 => { rdi = ((r15 as u32)) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 71; continue; }
            71 => { r15 = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rcx as u32)) as u32 as u64; r12 = ((rax.wrapping_mul(4)) as u64) as u64; r12 = ((r12).wrapping_add(r13)) as u64; rax = ((r14 as u32)) as u32 as u64; rcx = ((rax.wrapping_mul(4)) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; fl_l = r12; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 72; continue; }
            72 => { r12 = ((r12).wrapping_sub(r13)) as u64; rdi = r12; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r13)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; r14 = rcx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rcx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rcx = ((r14.wrapping_mul(4)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); r8 = r12; rdx = r13; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 74; continue; }
            74 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r13); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((r14 as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; bb = 75; continue; }
            75 => { rcx = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rcx as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rdi = ((rcx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 71; continue; } bb = 76; continue; }
            76 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; bb = 77; continue; }
            77 => { rbx = r13; rax = ((rcx as u32)) as u32 as u64; rdi = ((rax.wrapping_mul(4)) as u64) as u64; rdi = ((rdi).wrapping_add(r13)) as u64; fl_l = r13; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 97; continue; } bb = 78; continue; }
            78 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 79; continue; }
            79 => { r10 = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rax = rcx; rdx = r10; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 87; continue; } bb = 80; continue; }
            80 => { r9 = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 81; continue; }
            81 => { r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 83; continue; } bb = 82; continue; }
            82 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 86; continue; }
            83 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 85; continue; } bb = 84; continue; }
            84 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) < ((fl_r as u32))) { rax } else { rdx }) as u64; bb = 85; continue; }
            85 => { r10 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 86; continue; }
            86 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 81; continue; } bb = 87; continue; }
            87 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 89; continue; } bb = 88; continue; }
            88 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; bb = 89; continue; }
            89 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 95; continue; } bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 91; continue; }
            91 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 93; continue; } bb = 92; continue; }
            92 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 94; continue; }
            93 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 94; continue; }
            94 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 91; continue; } bb = 95; continue; }
            95 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x1); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 79; continue; } bb = 96; continue; }
            96 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; bb = 97; continue; }
            97 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; rbx = rdi; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x34u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 112; continue; } bb = 98; continue; }
            98 => { rdx = (0x333333333333333u64) as u64; bb = 99; continue; }
            99 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rdi = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r15 as u32)) as u32); rcx = rdi; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 106; continue; } bb = 100; continue; }
            100 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 101; continue; }
            101 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 103; continue; } bb = 102; continue; }
            102 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 104; continue; }
            103 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x1u32) as u32); rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 104; continue; }
            104 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 101; continue; } bb = 105; continue; }
            105 => { rdx = (0x333333333333333u64) as u64; bb = 106; continue; }
            106 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 108; continue; } bb = 107; continue; }
            107 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 110; continue; } bb = 108; continue; }
            108 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 123; continue; } bb = 109; continue; }
            109 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::lk_and_u32(r8.wrapping_add(0x20u64), (0xff000005u32) as u32); crate::simd_ops::lk_or_u32(r8.wrapping_add(0x20u64), (0x5u32) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x23u64), (0x5u8) as u8); crate::simd_ops::wu32(r8.wrapping_add(0x24u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu64(r8, rdi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdi); crate::simd_ops::wu16(r8.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r15); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x1); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 110; continue; }
            110 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rdx = rbx.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x34u64)) as u64; fl_l = rbx; fl_r = r14; rdx = (0x333333333333333u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 99; continue; } bb = 111; continue; }
            111 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 112; continue; }
            112 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 114; continue; } bb = 113; continue; }
            113 => { rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 114; continue; }
            114 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 117; continue; } bb = 115; continue; }
            115 => { rbx = rdi; bb = 116; continue; }
            116 => { rcx = rbx.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x34u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 116; continue; } bb = 117; continue; }
            117 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 118; continue; }
            118 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 119; continue; }
            119 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 121; continue; } bb = 120; continue; }
            120 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 121; continue; }
            121 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            122 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 123; continue; }
            123 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71920<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_71952<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(6u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_72090<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rsi = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rbx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    rdx = rsp.wrapping_add(0x40);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0;
    if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = ((rax as u32)) as u32 as u64;
    loop { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_72099<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 3; continue; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_72438<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = ((crate::simd_ops::ru64(P0)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rdi = rdx;
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rcx.wrapping_add(P2)) as u64) as u64;
    rax = P3;
    rbx = rcx;
    r9 = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r8 = 0x1;
    rcx = rdi;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(P6)) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64);
    rax = P7;
    r8 = 0x1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rax = P9;
    rcx = rdi;
    r9 = P10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_73097<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(4u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_73446<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); fl_l = ((((rax as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r14 = ((r14).wrapping_add(0x20u64)) as u64; r15 = (crate::simd_ops::ru64(rbx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = r15.wrapping_add(0x18); bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r15, r12); rcx = rdi.wrapping_sub(0x10); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), rax); rdx = r14.wrapping_sub(0x18); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; r15 = r15.wrapping_add(0x28); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x28); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; r14 = r14.wrapping_add(0x28); crate::simd_ops::wu32(rdi.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = (((rsi as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = rbx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_73525<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbp = rdx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; r15 = (((rbp) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = r14.wrapping_add(0x1c); rsi = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = r14.wrapping_add(0x8); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xa0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rsi); r14 = ((r14).wrapping_add(0xa8u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x14u64), rsi); crate::simd_ops::wu32(rbx.wrapping_sub(0xcu64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0x8u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rsi as u32)) as u32); rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rsi as u8)) as u8); rbx = rbx.wrapping_add(0xa8); rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0xacu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0xa9u64), (0x10u8) as u8); fl_l = r14; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { r14 = (((rcx) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; rbx = (((rbp) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rsi; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(0x8u64), rax); rax = (tmp_xchg) as u64; rbx = ((rbx).wrapping_add(0xa8u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 7; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_73552<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r15 = rdx; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r10 = 0x6348; rdx = ((rdx).wrapping_shl(3u32)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rsi = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = r12; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdi = rax; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdi = r12; bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r13); rbx = rdi.wrapping_add(0x18); r13 = P4; fl_l = ((r14).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r13); rcx = rbx.wrapping_sub(0x10); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rax); rdx = r14.wrapping_sub(0x18); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; r14 = r14.wrapping_add(0x28); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rbx = rbx.wrapping_add(0x28); rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = (((rbp as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 23; continue; }
            23 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { r15 = ((((r15 as u32)) & (0x3fffffffu32))) as u32 as u64; crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74013<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); rbp = rcx; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rcx = rbp.wrapping_add(0x8); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = ((rdi.wrapping_add(P6)) as u64) as u64; bb = 4; continue; }
            2 => { rsi = ((rdi.wrapping_add(P6)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = rsp.wrapping_add(0x40); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x55u64), ((rbx as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x57u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), ((rbx as u8)) as u8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsp.wrapping_add(0x48u64), rbx); rbx = (tmp_xchg) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rdx = rbp.wrapping_add(0x20); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_r = 0x1; rdx = rbp.wrapping_add(0x24); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdx, (0x42700000u32) as u32); bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74014<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rcx = rbp.wrapping_add(0x8); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rdi.wrapping_add(0x369); bb = 4; continue; }
            2 => { rsi = rdi.wrapping_add(0x369); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = rsp.wrapping_add(0x40); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x55u64), ((rbx as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x57u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), ((rbx as u8)) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsp.wrapping_add(0x48u64), rbx); rbx = (tmp_xchg) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x120u64), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = rcx.wrapping_add(0x4); crate::simd_ops::wu64(rdx.wrapping_add(0x120u64), rax); bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), (0x42700000u32) as u32); bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74644<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rbx.wrapping_add(P4), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P7))) as u32 as u64;
    rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74766<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_74767<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = ((rcx.wrapping_add(P8)) as u64) as u64;
    rcx = rdi;
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75182<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), (0x1u32) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x2u32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x338u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x328u64), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75183<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75184<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rsi = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rdx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rdx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x328u64), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75185<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rdx.wrapping_add(P6), (0x4u64) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); crate::simd_ops::lk_add_u64(rcx.wrapping_add(P6), (0x4u64) as u64); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75186<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rdi = ((rdi).wrapping_shl(4u32)) as u64;
    rdi = ((rdi).wrapping_add(rbx)) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp);
    rbp = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x50u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75295<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    r11 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r11 = ((r11).wrapping_shl(6u32)) as u64;
    r11 = ((r11).wrapping_add(r10)) as u64;
    fl_l = r10; fl_r = r11;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P2; r8 = 0x2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = r10; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = ((r10).wrapping_add(0x40u64)) as u64; fl_l = r10; fl_r = r11; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75597<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    rdx = rsp.wrapping_add(0x60);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_75619<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P7))) as u32 as u64;
    rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_76215<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P7))) as u32 as u64;
    rax = ((rdx.wrapping_add(P8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_76689<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rdx; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rbx = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rax.wrapping_add(0x18); rdx = r8.wrapping_add(0x18); bb = 4; continue; }
            4 => { rcx = rcx.wrapping_add(0x30); xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; r8 = r8.wrapping_add(0x30); rdx = rdx.wrapping_add(0x30); crate::simd_ops::wu64(rcx.wrapping_sub(0x48u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x40u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x3cu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x3cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x34u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_sub(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_sub(0x24u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = (((r9 as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_76826<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdi)) as u64; rdi = (((crate::simd_ops::ru32(rdi.wrapping_add(P0))) as i32 as i64) as u64) as u64; fl_l = rdi; fl_r = rdi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x458u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77161<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r8 = (crate::simd_ops::ru64(P0)) as u64;
    rdi = rdx;
    rdx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4cu32) as u32);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4cu32) as u32);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = rbx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P6))) as u32 as u64;
    rax = ((rdx.wrapping_add(P7)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77368<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = ((((rsi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((((rcx as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77369<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = ((((rsi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((((rcx as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77609<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = r14; rcx = rsi; r14 = ((r14).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77610<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77668<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); rsi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); rbp = (crate::simd_ops::ru32(rax.wrapping_add(r13).wrapping_add(0x180acu64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x18098u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = rbp; rcx = r12; r14 = ((r11.wrapping_add(rax)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = rbp; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); fl_l = rdi; fl_r = rdi; rcx = ((r15.wrapping_add(rdi)) as u64) as u64; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rdi }) as u64; fl_l = r11; fl_r = r11; rcx = ((r15.wrapping_add(r11)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { r11 }) as u64; rcx = ((r15.wrapping_add(rax)) as u64) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r11); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77679<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x98u64)) as u64;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (((rdx as u16)) as u32) as u32 as u64;
    r10 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64;
    fl_l = r10; fl_r = r10;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; } else { rax = 0; fl_l = 0; fl_r = 0; }
    r9 = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); r8 = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm1) as u128); rdx = rbx; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; }
    rsp = ((rsp).wrapping_add(0x98u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77683<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P1))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P2))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu16(rsp.wrapping_add(0x40u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = rsp.wrapping_add(0x40); let y_addr: u64 = rax; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77684<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r12); r12 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rbp = (crate::simd_ops::ru32(rax.wrapping_add(r12).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r12).wrapping_add(0x18098u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rsi.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r12).wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = rbp; rcx = r15; rsi = ((rsi).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(r12).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); fl_l = r11; fl_r = r11; rcx = ((r14.wrapping_add(r11)) as u64) as u64; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { r11 }) as u64; rcx = ((r14.wrapping_add(rax)) as u64) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r11); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x90u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77694<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r10 = (crate::simd_ops::ru64(r8)) as u64; rdi = rcx; rax = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r11 = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = rcx; r9 = r11; r9 = ((r9).wrapping_sub(rcx)) as u64; r9 = ((((r9) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r8 = r9; r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0xffffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = ((r9).wrapping_add(rax)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; rax = rcx.wrapping_add(0x2); bb = 7; continue; }
            6 => { r9 = r8; bb = 7; continue; }
            7 => { fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 23; continue; } bb = 10; continue; }
            10 => { r9 = (((rdx as u16)) as u32) as u32 as u64; r8 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rbx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10).wrapping_add(rbx)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((r9).wrapping_add(rbx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r9); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu8(rdi.wrapping_add(0x40u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { crate::simd_ops::wu8(rdi.wrapping_add(0x40u64), (0x0u8) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77866<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdi = r8; rbp = rdx; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r15); r15 = ((r8.wrapping_add(P2)) as u64) as u64; bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P0))) as u128; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r9 = rsp.wrapping_add(0x50); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_77984<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { xmm1 = 0u128; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rax = rcx; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm0 = 0u128; fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128); bb = 11; continue; }
            10 => { rax = r8; rcx = r8; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 13; continue; }
            13 => { rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rdi as u8)) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rdi as u8)) as u8); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rsi); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rsi = rax; fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 30; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = rbx; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 21; continue; }
            21 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 22; continue; }
            22 => { rdx = ((r8.wrapping_add(r8)) as u64) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 25; continue; }
            24 => { rax = rdx; rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 25; continue; }
            25 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x14u64)))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (0x8000000000000000u64) as u64; rax = rcx; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rcx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r8 }) as u64; bb = 29; continue; }
            29 => { rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rdx = P5; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P7; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78029<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = r11.wrapping_sub(0x38); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); r8 = rax.wrapping_add(0x40); rax = ((r9.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r13).wrapping_add(rcx)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r15).wrapping_add(rcx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r14).wrapping_add(rcx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rsi).wrapping_add(rcx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdx); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rcx); rcx = rbp.wrapping_sub(0x50); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let y_addr: u64 = rbp.wrapping_sub(0x30u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdx); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x130u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78030<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rcx = (((rdx as u16)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rbx).wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; r14 = (0xf1a9f1a9f1a9f1a9u64) as u64; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { rdi } else { r14 }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rdx = rbx; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78031<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); r11 = (0xf1a9f1a9f1a9f1a9u64) as u64; bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = ((r12 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rsi = r12; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r13).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r13).wrapping_add(0x18090u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; r15 = r11; r15 = (if (((fl_l as u32)) == ((fl_r as u32))) { r12 } else { r15 }) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rdx = r13; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { r11 = (0xf1a9f1a9f1a9f1a9u64) as u64; bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78054<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((rdx as u16)) as u16); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r13); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x98u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78055<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r15); r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rsi = rdi; bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; rax = ((r14.wrapping_add(rsi)) as u64) as u64; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rsi }) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r8 = r15; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdx = rbx; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78056<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); rsi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r14 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rax as u16)) as u16); rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r12); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78057<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x580e0u64))) as u64; r13 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r14 = 0; fl_l = 0; fl_r = 0; rax = ((r12.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r8 = r13; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rdx = rsi; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78188<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78195<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rbp); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r14; rcx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = r14; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r13)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r12; fl_r = r12; rax = ((r12.wrapping_add(r13)) as u64) as u64; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r12 }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r13)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78312<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); rsi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r15 = (crate::simd_ops::ru32(rax.wrapping_add(r13).wrapping_add(0x180acu64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x18098u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r15; rcx = r12; r14 = ((r11.wrapping_add(rax)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = r15; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); fl_l = rdi; fl_r = rdi; rcx = ((rdi.wrapping_add(rbp)) as u64) as u64; let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rdi }) as u64; rdx = r13; fl_l = r11; fl_r = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rcx = ((r11.wrapping_add(rbp)) as u64) as u64; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { r11 }) as u64; rcx = ((rax.wrapping_add(rbp)) as u64) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r11); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; rcx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78361<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = rsi; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; rbx = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14.wrapping_add(P0), rdi); bb = 8; continue; }
            4 => { if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(P2))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rdi = ((rdi).wrapping_sub(rcx)) as u64; r9 = r14; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(P0), rax); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78389<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rbx = rbp; rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rsi.wrapping_add(P6))) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rsi)) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78436<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r8, (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); rdx = rbp; rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi, rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, (0x0u64) as u64); bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78451<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64, const P18: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<512, 257>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x7f0u64)) as u64; r12 = rdx; r15 = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rbp = P0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdi = (0xffffffffffffffffu64) as u64; rax = rdi; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r15)) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu16(rax, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); rbp = (crate::simd_ops::ru64(r14)) as u64; r14 = ((r14).wrapping_add(0x48u64)) as u64; rcx = rsp.wrapping_add(0xa8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x40); rdx = rbp; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xa8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x80u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xc8u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi.wrapping_add(0xe8u64), ((xmm0 as u64)) as u64); rbx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x40u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P9; crate::simd_ops::wu64(rsi.wrapping_add(0x120u64), rax); rax = P10; crate::simd_ops::wu64(rsi.wrapping_add(0x128u64), rax); crate::simd_ops::wu8(rsi.wrapping_add(0x115u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r14 = ((crate::simd_ops::ru16(P11)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rbp.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xeu64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xcu64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64), rbp); rcx = (crate::simd_ops::ru64(r15)) as u64; r9 = 0x2; r8 = ((crate::simd_ops::ru16(r15.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(r12, rax); crate::simd_ops::wu16(r12.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(r12.wrapping_add(0xau64), ((rbp as u16)) as u16); crate::simd_ops::wu128(r12.wrapping_add(0x10u64), (xmm0) as u128); rax = r12; r11 = rsp.wrapping_add(0x7f0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 11; continue; }
            11 => { r8 = 0x71a; rdx = P15; rcx = rsp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x7e2u64), (0x0u8) as u8); rcx = rbp; rdx = rsp.wrapping_add(0xc8); r8 = 0x71b; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78525<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r13 = rdx; rbx = r8; rdx = rbp.wrapping_sub(0x49); r15 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rsi); rax = (0xaaaaaaaaaaaaaaabu64) as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rdi); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; rdx = ((rdx).wrapping_shr(5u32)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r14); rax = (((rdx as u16)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add((rdx as u16))) as u16 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rsi); rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rdx as u16))) as u16 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rdi); rbx = ((rbx).wrapping_add(rsi)) as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shl(4u32)) as u16 as u64)); r8 = (r8 & !0xffffu64) | (((((r8 as u16)).wrapping_sub((rax as u16))) as u16 as u64)); crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rdi); rdx = (((r8 as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rsi); rcx = rbx.wrapping_add(0xa); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rbx); bb = 5; continue; }
            5 => { let y_addr: u64 = rbp.wrapping_sub(0x49u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x1fu64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x21u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); bb = 6; continue; }
            6 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = r9; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x1au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r8); fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rax = rdx; bb = 16; continue; }
            16 => { rdx = rax.wrapping_add(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rdx); rax = rdx; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r8); fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 6; continue; }
            19 => { let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; let y_addr: u64 = r13; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; let y_addr: u64 = r13.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(r13.wrapping_add(0x40u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::wu8(r13.wrapping_add(0x48u64), (0x0u8) as u8); bb = 21; continue; }
            20 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; rdx = rbp.wrapping_sub(0x29); r8 = rbx; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r13.wrapping_add(0x48u64), (0x1u8) as u8); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r13; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = r13.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(r13.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); bb = 21; continue; }
            21 => { rax = r13; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78526<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbp = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r15 = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rcx = (0xfffffffffffffffu64) as u64; rdi = r8; r14 = rdx; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdx = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = rbx.wrapping_sub(0x1); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); bb = 3; continue; }
            3 => { r8 = rdi; rcx = rbp.wrapping_add(0x38); rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; let prod = (rax as u128).wrapping_mul((r15) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rdx = ((rdx).wrapping_shr(5u32)) as u64; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rax = (((rdx as u16)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add((rdx as u16))) as u16 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rdx as u16))) as u16 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rbx = ((rbx).wrapping_add(rcx)) as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shl(4u32)) as u16 as u64)); r15 = (r15 & !0xffffu64) | (((((r15 as u16)).wrapping_sub((rax as u16))) as u16 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rdx = (((r15 as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rbx.wrapping_add(0xa); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0x28u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(r14, rbp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rdx); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; bb = 19; continue; }
            19 => { rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rdx); rcx = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 9; continue; }
            22 => { rax = r14; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rdx = P3; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P5; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78527<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x100u64), (xmm6) as u128); rbx = rcx; rbp = (0x8000000000000000u64) as u64; xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(0x30u64)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; fl_l = r9; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32) as u128; r9 = ((r9).wrapping_shr(1u32)) as u64; fl_l = r9; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { xmm0 = 0u128; fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r9) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rax = r9; rcx = r9; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbp; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r9 = ((r9).wrapping_shr(1u32)) as u64; fl_l = r9; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 13; continue; }
            13 => { r8 = rbx; rcx = rsp.wrapping_add(0x40); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rdi as u8)) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), ((rdi as u8)) as u8); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), rsi); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rsi = rax; fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 30; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rbx; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 21; continue; }
            21 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 22; continue; }
            22 => { rdx = ((r9.wrapping_add(r9)) as u64) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 25; continue; }
            24 => { rax = rdx; rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 25; continue; }
            25 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x14u64)))).to_bits() as u128); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = rbp; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x30u64)))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rcx; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r9 }) as u64; bb = 29; continue; }
            29 => { r8 = rbx; rcx = rsp.wrapping_add(0xa0); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0xa0); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0xd8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rdx = P5; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P7; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78777<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = r11.wrapping_sub(0x58); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); r8 = rax.wrapping_add(0x40); rax = ((r9.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rcx = ((rax.wrapping_add(rdx)) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 11; continue; }
            11 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r13).wrapping_add(rdx)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r12).wrapping_add(rdx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r15).wrapping_add(rdx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r14).wrapping_add(rdx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rsi).wrapping_add(rdx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdi).wrapping_add(rdx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r8); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rbx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r8); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp, (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 30; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; bb = 31; continue; }
            31 => { rsp = ((rsp).wrapping_add(0x150u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_78787<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = ((r15).wrapping_add(r15)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); r12 = ((r15.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r8 = r13.wrapping_add(0x40); fl_l = r15; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { rbp = ((rax.wrapping_add(rbx)) as u64) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x28u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = ((r12).wrapping_add(1)) as u64; fl_l = r12; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 22; continue; }
            22 => { r8 = r13.wrapping_add(0x40); bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = r15; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 25; continue; }
            25 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0xe0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79166<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); r8 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79167<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); rsi = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(0x180acu64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x18098u64))) as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rdx = r14; rcx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rsi).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r12)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r12)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79281<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r13 = rdx; r12 = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 92; continue; } bb = 2; continue; }
            2 => { rdi = 0x1000; r15 = (0xffffffffffffu64) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (((r8) & (r15))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), r8); rax = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x58); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp.wrapping_add(0x58); rdx = rbp.wrapping_sub(0x30); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_add(0x14u64)) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r12.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r12)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r12, r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { bb = 92; continue; }
            11 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rdx = 0; fl_l = 0; fl_r = 0; r14 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rdx); rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rdx); rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { r15 = ((rdx as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = ((rdi as u32)) as u32 as u64; rdi = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rsi as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdi = ((rdi).wrapping_sub(r14)) as u64; r15 = rdi; r15 = ((((r15) as i64).wrapping_shr(3u32) as u64)) as u64; rax = r15.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rsi = rcx; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rax; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rsi }) as u64; rcx = ((rsi.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; r8 = rdi; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { r14 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r13); crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((rsi as u32)) as u32); rdx = ((r15 as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 17; continue; }
            17 => { rax = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax.wrapping_mul(8)), rbx); rdi = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = rax; r15 = ((rdi as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r14); rdx = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rdx); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(r12)) as u64; rbx = (0x100000001b3u64) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 20; continue; }
            20 => { rcx = 0x1000; r15 = (0xffffffffffffu64) as u64; bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rcx = (0xffffffffffffffffu64) as u64; bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rcx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x58); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rcx = (((rsi as u8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x59u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x5au64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x5bu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = (((rcx) & (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 30; continue; }
            30 => { bb = 32; continue; }
            31 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 33; continue; }
            33 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 34; continue; }
            34 => { rdx = rbx.wrapping_add(0x14); rax = (crate::simd_ops::ru64(r12)) as u64; rcx = 0x1000; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 36; continue; }
            36 => { r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r12.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r14; rcx = ((rdi as u32)) as u32 as u64; rdx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 38; continue; }
            38 => { rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rdx); bb = 40; continue; }
            40 => { fl_l = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rdi as u32)) as u32); bb = 42; continue; }
            41 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 42; continue; }
            42 => { rcx = ((rdi as u32)) as u32 as u64; rdx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 43; continue; }
            43 => { bb = 47; continue; }
            44 => { rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(r12)) as u64; rcx = 0x1000; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (((rax) & (r15))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 46; continue; }
            46 => { rdx = ((rdx).wrapping_add(0x4u64)) as u64; r9 = rax; r8 = 0; fl_l = 0; fl_r = 0; rcx = r12.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(r12)) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(r12, rax); fl_l = rax; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rcx = 0x1000; rbx = (0x100000001b3u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); rdx = 0; fl_l = 0; fl_r = 0; bb = 49; continue; }
            49 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); r15 = rdx; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rdx); rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64); r12 = ((rdx as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rdi = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rdi); rax = rdi; rax = ((rax).wrapping_sub(r14)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { r12 = rdx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; r12 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { r12 }) as u64; rcx = ((r12.wrapping_mul(4)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((r12 as u32)) as u32); bb = 51; continue; }
            51 => { rbx = r14; rax = ((rdx as u32)) as u32 as u64; fl_l = r14; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 59; continue; } bb = 52; continue; }
            52 => { rsi = ((rdx as u32)) as u32 as u64; r13 = ((rdx as u32)) as u32 as u64; bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rax = ((rsi as u32)) as u32 as u64; rsi = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r12 as u32)) as u32 as u64; rcx = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 54; continue; }
            54 => { rsi = ((rsi).wrapping_sub(r15)) as u64; r13 = rsi; r13 = ((((r13) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r13.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r15)) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; rdi = rcx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rcx = ((rdi.wrapping_mul(4)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; r8 = rsi; rdx = r15; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 56; continue; }
            56 => { r15 = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r12); r12 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rdi as u32)) as u32); rdx = ((r13 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 57; continue; }
            57 => { rcx = ((r13 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rsi = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rsi as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; r13 = ((rsi as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 58; continue; }
            58 => { rax = ((rsi as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 59; continue; }
            59 => { rbx = r15; rcx = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r15; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 60; continue; }
            60 => { r14 = 0; fl_l = 0; fl_r = 0; r12 = (0x100000001b3u64) as u64; bb = 61; continue; }
            61 => { rdx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rcx = rdx; rcx = (((rcx) & (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 67; continue; } bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 68; continue; } bb = 63; continue; }
            63 => { bb = 64; continue; }
            64 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 67; continue; } bb = 65; continue; }
            65 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 66; continue; }
            66 => { bb = 68; continue; }
            67 => { rdi = r14; bb = 68; continue; }
            68 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 69; continue; }
            69 => { rdx = (((rdx) & (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 73; continue; } bb = 70; continue; }
            70 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); bb = 75; continue; }
            72 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); bb = 75; continue; }
            73 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 75; continue; }
            75 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::lk_sub_u64(r13.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rdi.wrapping_add(0x14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 76; continue; }
            76 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = ((rsi as u32)) as u32 as u64; rcx = ((r15.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 77; continue; }
            77 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 78; continue; }
            78 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rbx = rsi; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; r12 = (((rdi) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; r12 = ((r12).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 82; continue; } bb = 79; continue; }
            79 => { bb = 80; continue; }
            80 => { r8 = rbx; rdx = rbp.wrapping_sub(0x40); rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rax)) as u64; rdx = rbx.wrapping_add(0x4); rcx = rsi.wrapping_add(0x14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x58u64)) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 80; continue; } bb = 81; continue; }
            81 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; bb = 82; continue; }
            82 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 84; continue; } bb = 83; continue; }
            83 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 84; continue; }
            84 => { rax = ((rdi as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 88; continue; } bb = 85; continue; }
            85 => { rax = ((rdi as u32)) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; rbx = rsi; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 88; continue; } bb = 86; continue; }
            86 => { bb = 87; continue; }
            87 => { rcx = rbx.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x58u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 87; continue; } bb = 88; continue; }
            88 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 89; continue; }
            89 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 90; continue; }
            90 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 92; continue; } bb = 91; continue; }
            91 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 92; continue; }
            92 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79502<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = ((r15).wrapping_add(r15)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); r12 = ((r15.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); fl_l = r15; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { rbp = ((rax.wrapping_add(r13)) as u64) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_add(r13)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rsi).wrapping_add(r13)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(r13)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(r13)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { r13 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; fl_r = r13; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x28u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = ((r13).wrapping_add(1)) as u64; fl_l = r13; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = r15; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 25; continue; }
            25 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0xe0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79525<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = r8; rbx = rdx; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(P3), r14); r14 = ((r8.wrapping_add(P4)) as u64) as u64; bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128; ymm1_hi = 0; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = rsp.wrapping_add(0x70); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(P1))) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = rsp.wrapping_add(0x90); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79591<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0xb0); rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9).wrapping_add(r9)) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r10); r8 = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; r8 = r10.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rax = ((r9.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1c0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x1a8u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x1a0u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x198u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x188u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x180u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rcx = ((rax.wrapping_add(rdx)) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 11; continue; }
            11 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((r13).wrapping_add(rdx)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r12).wrapping_add(rdx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r15).wrapping_add(rdx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r14).wrapping_add(rdx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rsi).wrapping_add(rdx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rdi).wrapping_add(rdx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r8); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x38u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); rdx = rbp; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r8); r8 = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rbx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r8); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 30; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x188u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x198u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1c0u64))) as u64; bb = 31; continue; }
            31 => { rsp = ((rsp).wrapping_add(0x1b0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79667<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = ((rax).wrapping_shr(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = rdi; r8 = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rax, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79679<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); r15 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = r15.wrapping_add(0x40); crate::simd_ops::wu16(rbp.wrapping_sub(0x79u64), ((r9 as u16)) as u16); rcx = rbp.wrapping_sub(0x19); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r11 = ((rsi.wrapping_add(rax)) as u64) as u64; bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r9).wrapping_add(rsi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r10).wrapping_add(rsi)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(rsi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(rsi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { let y_addr: u64 = rbp.wrapping_sub(0x79u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); rcx = rbp.wrapping_sub(0x19); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r11); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r8); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x7u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rdx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; bb = 25; continue; }
            25 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0xe8u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79801<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rsi = rdx; r8 = rdx; rdx = rsp.wrapping_add(0x20); rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x50u64)))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru32(rsi)) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 3; continue; }
            3 => { r8 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = (0xcbf29ce484222325u64) as u64; rax = ((rsi as u32)) as u32 as u64; rcx = (((rsi as u8)) as u32) as u32 as u64; rcx = (((rcx) ^ (rdx))) as u64; rax = (((rax as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = (((rax as u8)) as u32) as u32 as u64; r14 = ((r14).wrapping_sub(1)) as u64; rax = (((rax) ^ (rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rdx = (((rcx as u8)) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rax = (((rax) ^ (rdx))) as u64; rcx = ((rcx).wrapping_shr(24u32)) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rdx = r11.wrapping_sub(0x1); rax = (((rax) ^ (rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rdi = rbx.wrapping_add(0x28); rax = (((rax) & (r14))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11); rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(r8))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { r10 = ((rcx.wrapping_add(r8)) as u64) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; r9 = r8; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 9; continue; }
            8 => { r9 = r8; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { r11 = ((r11).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(r11)) as u64; rax = (((rax) & (r14))) as u64; rcx = rax; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(rcx))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 10; continue; }
            10 => { r8 = r9; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rax; r9 = r8; bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r8 = (0xfffffffffffffffu64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = r10; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 13; continue; }
            13 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = rdx; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = rcx.wrapping_sub(0x1); crate::simd_ops::wu64(rdi, rax); bb = 20; continue; }
            16 => { rdx = (0xffffffffffffffffu64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = r10.wrapping_add(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = ((rdx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu128(rax.wrapping_add(rdx), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            21 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r8 = (0xfffffffffffffffu64) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = r10; rax = ((rax).wrapping_sub(r9)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = rbx.wrapping_add(0x50); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r8; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = r9.wrapping_sub(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); bb = 26; continue; }
            25 => { rax = r10.wrapping_add(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r8 = ((r8).wrapping_shl(4u32)) as u64; crate::simd_ops::wu128(r8.wrapping_add(rax), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rdx = P2; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = P2; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79803<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x28u64)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { xmm1 = 0u128; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rax = rcx; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm0 = 0u128; fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128); bb = 11; continue; }
            10 => { rax = r8; rcx = r8; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = ((r8).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 13; continue; }
            13 => { rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rdi as u8)) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rdi as u8)) as u8); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = (0xfffffffffffffffeu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rsi); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rsi = rax; fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 30; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = rbx; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x28u64)))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 21; continue; }
            21 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 22; continue; }
            22 => { rdx = ((r8.wrapping_add(r8)) as u64) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 25; continue; }
            24 => { rax = rdx; rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 25; continue; }
            25 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x14u64)))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (0x8000000000000000u64) as u64; rax = rcx; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x28u64)))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rcx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r8 }) as u64; bb = 29; continue; }
            29 => { rdx = rbx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rdx = P5; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P7; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_79821<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r12); r12 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(r12.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(r12).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r12).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r12).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; rax = ((r15.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); fl_l = rbp; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80366<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rcx = r8.wrapping_add(0x1); rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P2)) as u128; xmm3 = (crate::simd_ops::ru128(P3)) as u128; rax = rdx; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rax)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rax, (xmm1) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rdx).wrapping_add(0x1u64), (xmm0) as u128); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r8.wrapping_add(rdx), (0xffu8) as u8); r8 = (crate::simd_ops::ru64(P1)) as u64; r11 = ((rbp as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r12); r12 = (0xde5fb9d2630458e9u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r13); r13 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rsp, r15); r15 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r11))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r14 = ((r10.wrapping_add(r11)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = (((rdx) ^ (r13))) as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x5u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x7u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = r12; rdx = (((rdx) as i64).wrapping_mul((r15) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; rbx = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rbp; rcx = rbx; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r9 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = rbx; rax = r11; r9 = ((r9).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r8))) as u64; r9 = (((r9) & (r8))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_shr(4u32)) as u64; rax = ((rax).wrapping_shr(4u32)) as u64; rcx = r9; rcx = ((rcx).wrapping_sub(rdx)) as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & (0x7fu8))) as u8 as u64)); rcx = ((rcx).wrapping_shr(4u32)) as u64; rax = (((rax) & (r8))) as u64; rcx = (((rcx) & (r8))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r14, ((rbx as u8)) as u8); rcx = rdi; rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(P0))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(0x1u64), ((rbx as u8)) as u8); bb = 14; continue; }
            11 => { rax = ((r10.wrapping_add(r9)) as u64) as u64; rcx = r9.wrapping_sub(0x10); r9 = ((r9).wrapping_add(r9)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rdx = (crate::simd_ops::ru64(P1)) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx) & (rdx))) as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P0))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(0x1u64), ((rbx as u8)) as u8); rcx = rdi; rax = (crate::simd_ops::ru64(P4)) as u64; xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r9.wrapping_mul(8)), (xmm0) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu8(rax.wrapping_add(r11), (0x80u8) as u8); rdx = (crate::simd_ops::ru64(P1)) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(P0))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(0x1u64), (0x80u8) as u8); bb = 14; continue; }
            13 => { rcx = (((rcx) & (rdx))) as u64; r11 = ((r11).wrapping_sub(1)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P0))) as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; rdi = ((rdi).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(0x1u64), ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(P4)) as u64; xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(rax))) as u128; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r9.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(rax), (xmm0) as u128); rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rsi; fl_r = (0x10u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(r9.wrapping_mul(8)), (xmm1) as u128); bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(P1)) as u64; r10 = (crate::simd_ops::ru64(P0)) as u64; bb = 15; continue; }
            15 => { r11 = ((r11).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r11; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 17; continue; }
            17 => { rax = r8; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(P5))) as u64; crate::simd_ops::wu64(P6, r8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80367<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx.wrapping_add(0x18); rdi = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; r14 = (crate::simd_ops::ru64(P1)) as u64; rsi = (((rsi) & (0xfffffffffffffff8u64))) as u64; rbp = (crate::simd_ops::ru64(P2)) as u64; r15 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = 0x6348; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = r15; bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rax; bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r15; bb = 14; continue; }
            14 => { rax = ((rsi.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(P0, rcx); r8 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(P1, rax); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P0)) as u64; rcx = rbx; r10 = r15; crate::simd_ops::wu8(rax.wrapping_add(rbx), (0xffu8) as u8); rax = rbx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(P2, rbx); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(P8))) as u64; crate::simd_ops::wu64(P9, rcx); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r11 = r14.wrapping_add(0x2); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r13); r14 = (0x100000001b3u64) as u64; r13 = (0xcbf29ce484222325u64) as u64; r12 = (0xde5fb9d2630458e9u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r10))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 17; continue; }
            17 => { rax = ((crate::simd_ops::ru8(r11.wrapping_sub(0x1u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_sub(0x2u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rcx = (((rcx) ^ (r13))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = r12; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; rsi = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r15; rcx = rsi; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (rbx))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r9))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (rbx))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r9))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x7fu8))) as u8 as u64)); r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (rbx))) as u64; crate::simd_ops::wu8(r9.wrapping_add(r8), ((rsi as u8)) as u8); rcx = r8.wrapping_sub(0x10); rdx = (crate::simd_ops::ru64(P2)) as u64; rcx = (((rcx) & (rdx))) as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(P0))) as u64; fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx).wrapping_add(0x1u64), ((rsi as u8)) as u8); rax = (crate::simd_ops::ru64(P1)) as u64; xmm0 = (crate::simd_ops::ru128(r11.wrapping_sub(0x2u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r8.wrapping_mul(8)), (xmm0) as u128); rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 21; continue; }
            21 => { r10 = ((r10).wrapping_add(1)) as u64; r11 = ((r11).wrapping_add(0x10u64)) as u64; fl_l = r10; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = rdi.wrapping_sub(0x1); rax = rdi; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rdi); if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r9 = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80371<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = ((rdx).wrapping_shl(6u32)) as u64; rcx = 0x11; fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = r9; rax = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, (0xc0u64) as u64); rsi = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x8u32) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = P6; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P9), rbp); crate::simd_ops::wu64(rbx, r14); rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80418<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 27; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = ((rax as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rax = 0x8; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = (crate::simd_ops::ru64(rcx)) as u64; rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rbp as u32) }) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r10 = 0x6348; rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (rdx as u32) } else { (rbp as u32) }) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rdi = rax; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { r9 = 0; fl_l = 0; fl_r = 0; rsi = rbx.wrapping_add(0x10); r14 = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(r8.wrapping_mul(8)), (xmm0) as u128); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rsi = rbx.wrapping_add(0x10); bb = 18; continue; }
            18 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { rcx = r15.wrapping_sub(0x1); rax = r15; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r15, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r15); if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rbx.wrapping_add(0x8); rsi = rbx.wrapping_add(0x10); bb = 26; continue; }
            23 => { r9 = r15; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { r14 = rbx.wrapping_add(0x8); rsi = rbx.wrapping_add(0x10); bb = 26; continue; }
            25 => { r9 = r15; r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(r14, (0x0u32) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80471<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P2)) as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rcx = 0x8; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; rdi = rax; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(P2)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(r9.wrapping_mul(8)), rcx); r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = rbx.wrapping_sub(0x1); rax = rbx; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rbx); if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { r9 = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(P1, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); crate::simd_ops::wu64(P2, rdi); crate::simd_ops::wu32(P4, (0x0u32) as u32); bb = 15; continue; }
            14 => { rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rdi = (crate::simd_ops::ru64(P2)) as u64; bb = 15; continue; }
            15 => { rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rcx.wrapping_mul(8)), rax); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(P7, (0x0u32) as u32); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = rdx; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(P2)) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80707<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = (crate::simd_ops::ru64(P0)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = r8; rbx = rdx; r12 = rcx; rsi = ((r13 as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; ymm0_hi = 0; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x3u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x1; let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = P3; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r13); let y_addr: u64 = rdx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdx.wrapping_add(0x38u64), (xmm1) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = 0x3; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(1)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80829<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x68); rsp = ((rsp).wrapping_sub(0x168u64)) as u64; rdi = r9; r14 = r8; rbx = rcx; rsi = (crate::simd_ops::ru64(rcx)) as u64; rsi = ((rsi).wrapping_add(0x58u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rsi); rcx = ((crate::simd_ops::ru16(r9.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = ((((rcx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x40u64), ((rax as u16)) as u16); r13 = 0x1; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14; rdx = rsp.wrapping_add(0x38); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((((r10 as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = P3; r8 = ((r10.wrapping_sub(0x2u64)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x33e1600u64)))) as u32 as u64; r9 = ((r10 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x33e1568u64)))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(0x2u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rcx = (((crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rbx = (((rbx as u32)).wrapping_shr(14u32)) as u32 as u64; rbx = ((((rbx as u32)) & (0xfffu32))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(14u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0xfffu32))) as u32 as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_shr(14u32)) as u32 as u64; r12 = ((((r12 as u32)) & (0xfffu32))) as u32 as u64; r15 = (((crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; r15 = ((r15).wrapping_add(rdx)) as u64; rax = (((crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rsi = (((crate::simd_ops::ru32(r11.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; rsi = (((rsi) as i64).wrapping_mul((rax) as i64) as u64) as u64; r14 = (((crate::simd_ops::ru32(r11.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; r14 = (((r14) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x50u64), (xmm1) as u128); r8 = rbp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x48); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rcx = ((rbx as u32)) as u32 as u64; rbx = ((crate::simd_ops::ru8(r15.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(7u32)) as u64; rbx = ((rbx).wrapping_add(rsi)) as u64; r11 = ((crate::simd_ops::ru8(r15.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r11 = ((r11).wrapping_shl(7u32)) as u64; r11 = ((r11).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r9 = ((crate::simd_ops::ru8(r15.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; r9 = ((r9).wrapping_shl(7u32)) as u64; r9 = ((r9).wrapping_add(rsi)) as u64; r10 = ((crate::simd_ops::ru8(r15.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r10 = ((r10).wrapping_shl(7u32)) as u64; r10 = ((r10).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); rdx = ((crate::simd_ops::ru8(r15.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(7u32)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; r8 = ((crate::simd_ops::ru8(r15.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r8 = ((r8).wrapping_shl(7u32)) as u64; r8 = ((r8).wrapping_add(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdx); rcx = r9; rcx = ((rcx).wrapping_sub(rbx)) as u64; rax = r10; rax = ((rax).wrapping_sub(r11)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rcx); rcx = rdx; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = r8; rax = ((rax).wrapping_sub(r10)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rcx); rbx = ((rbx).wrapping_sub(rdx)) as u64; r11 = ((r11).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r11); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rbx); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = rbp.wrapping_sub(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp.wrapping_add(0xc0); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu8(rdi, ((r13 as u8)) as u8); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rbx = ((r13.wrapping_sub(0x2u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 78; continue; }
            12 => { rax = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x74u64))) as u32 as u64; rsi = 0x1; bb = 13; continue; }
            13 => { rdi = 0x1; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x1u32) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x50u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            18 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 22; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rbx as u32)) as u32); fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_sub(0x50u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            21 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rax as u32)) as u32); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = rbp.wrapping_sub(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp.wrapping_add(0xb0); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 39; continue; } bb = 24; continue; }
            24 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 25; continue; }
            25 => { rdi = 0x1; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x44u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), (0x0u32) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x4cu64), (1) as u32)) as u32 as u64; bb = 33; continue; }
            29 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_sub(0x4cu64), (1) as u32)) as u32 as u64; bb = 32; continue; }
            31 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 32; continue; }
            32 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), ((rax as u32)) as u32); bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = rbp.wrapping_sub(0x40); r8 = rsp.wrapping_add(0x50); rdx = rbp.wrapping_add(0xc8); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 34; continue; }
            34 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 35; continue; }
            35 => { r13 = (0u32.wrapping_sub((r13 as u32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 36; continue; }
            36 => { rdi = 0x1; bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x1u32) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_sub(0x50u64), (1) as u32)) as u32 as u64; bb = 58; continue; }
            39 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rbx = 0xffffffff; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 43; continue; }
            43 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 45; continue; }
            45 => { rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 78; continue; }
            48 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rbx = 0xffffffff; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 52; continue; }
            52 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 56; continue; }
            56 => { rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 78; continue; }
            57 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rax as u32)) as u32); bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = rbp.wrapping_sub(0x40); r8 = rsp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x30); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 69; continue; } bb = 59; continue; }
            59 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 37; continue; } bb = 60; continue; }
            60 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rbx = 0xffffffff; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 63; continue; }
            63 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 64; continue; }
            64 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 66; continue; }
            66 => { rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            68 => { rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 78; continue; }
            69 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rbx = 0xffffffff; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 70; continue; }
            70 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 72; continue; }
            72 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 75; continue; } bb = 73; continue; }
            73 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 75; continue; }
            75 => { rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 76; continue; }
            76 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 77; continue; }
            77 => { rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 78; continue; }
            78 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 79; continue; }
            79 => { let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = (crate::simd_ops::ru64(P8)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 81; continue; }
            81 => { rax = rdi; rsp = ((rsp).wrapping_add(0x168u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80893<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rsi = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x2u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x2u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; r8 = ((rcx.wrapping_add(P2)) as u64) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32) as u128; r8 = ((rdi.wrapping_add(P11)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rcx = rbx.wrapping_add(0x58); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x2u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_80942<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rsi = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x2u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; r8 = ((rcx.wrapping_add(P2)) as u64) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32) as u128; r8 = ((rdi.wrapping_add(P10)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_81305<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (0x2e8ba2e8ba2e8ba3u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rdx = ((rdx).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rax.wrapping_add(rcx)) as u64) as u64; rdx = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_81395<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdi = rdx;
    r8 = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rcx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32);
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rdx = rbx;
    r8 = (crate::simd_ops::ru64(P4)) as u64;
    rcx = rdi;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_81848<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r9 = rdx;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(r8)) as u64;
    rdi = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = rdi;
    rcx = (crate::simd_ops::ru64(r9)) as u64;
    rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), (0x0u64) as u64); }
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rdx = rdi;
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_82529<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P0)) as u128; xmm3 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rcx, (xmm1) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(r8.wrapping_add(rdx), (0xffu8) as u8); r15 = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); rbp = r15.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rsi = (0xde5fb9d2630458e9u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r15))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r13 = ((r14.wrapping_add(r15)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rax = rsi; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (rdi))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r14.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (rdi))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r14.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rsi = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = r9; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); rsi = ((rsi).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))))) as u64; rsi = (((rsi) & (rdi))) as u64; rcx = rsi; rax = r15; rcx = ((rcx).wrapping_sub(rdx)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = (((rcx) ^ (rax))) as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x7fu8))) as u8 as u64)); rcx = (((rcx) & (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r13, ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rbp))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); bb = 14; continue; }
            11 => { rax = ((r14.wrapping_add(rsi)) as u64) as u64; rcx = rsi.wrapping_sub(0x10); rsi = ((rsi).wrapping_shl(5u32)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; crate::simd_ops::wu8(rax, ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((rax.wrapping_add(r12)) as u64) as u64; rdx = ((rsi.wrapping_add(rax)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(r15.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rbp))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 14; continue; }
            13 => { rcx = (((rcx) & (rax))) as u64; rdx = rsp.wrapping_add(0x20); rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(r12)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((rsi.wrapping_add(rax)) as u64) as u64; rdx = ((rax.wrapping_add(r12)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x20); rdx = ((rdx).wrapping_add(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_sub(1)) as u64; r12 = ((r12).wrapping_sub(0x20u64)) as u64; fl_l = ((rbp).wrapping_sub(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rsi = (0xde5fb9d2630458e9u64) as u64; bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r15 = ((r15).wrapping_add(1)) as u64; r12 = ((r12).wrapping_add(0x20u64)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = r15; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 16; continue; }
            16 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 17; continue; }
            17 => { rax = rdi; rax = ((rax).wrapping_shr(3u32)) as u64; rdi = ((rdi).wrapping_sub(rax)) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_82555<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rbp; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx, rax); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            5 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rbp; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_83353<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rsi = r8;
    rbp = rsp.wrapping_add(0x78);
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp);
    r9 = rsi;
    r8 = rbx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rdx = rdi;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rcx = 0xffffffff;
    rax = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_83915<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = r8; r14 = rcx; rbx = 0x1; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(rdi)) as u128; xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64) as u128; rax = (crate::simd_ops::ru64(r14)) as u64; r8 = rsp.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = r14; rdx = ((xmm1 as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x2; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rdi = ((rdi.wrapping_add(P2)) as u64) as u64; rsi = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rbx as u32) } else { (rcx as u32) }) as u32 as u64; rbx = ((rcx as u32)) as u32 as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_83923<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r8 = 0x1; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rax = ((r8).wrapping_shl((((rcx) as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            2 => { rax = rdx.wrapping_sub(0x1); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P2)) as u64; rax = (((rax) | (r8))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((r8).wrapping_shl((((rcx) as u32) & 63))) as u64; r8 = rbx; rcx = P3; rdx = ((rdi.wrapping_add(rdi)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P2)) as u64; r9 = rdi.wrapping_sub(0x1); crate::simd_ops::wu64(P5, r9); crate::simd_ops::wu64(P6, rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rdi = (0x100000001b3u64) as u64; rsi = (0xcbf29ce484222325u64) as u64; bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x11u64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r11 = (((r11) ^ (rsi))) as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x12u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x13u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x14u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x15u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x16u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x17u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) & (r9))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(P3))) as u64; r9 = (crate::simd_ops::ru64(r11)) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r11, rax); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            10 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10, rcx); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rdx); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r10); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(r11, rax); bb = 15; continue; }
            15 => { rax = rcx; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(P5)) as u64; bb = 4; continue; }
            17 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 15; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84116<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r11 = (((rax) as i64).wrapping_mul((0x78u64) as i64) as u64) as u64;
    r11 = ((r11).wrapping_add(r10)) as u64;
    fl_l = r10; fl_r = r11;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P2; r8 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = r10; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = ((r10).wrapping_add(0x78u64)) as u64; fl_l = r10; fl_r = r11; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84176<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx, r8); rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rbp as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r11 = (crate::simd_ops::ru32(rsi)) as u32 as u64; bb = 4; continue; }
            4 => { rdx = r9; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; r9 = rdx; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { r8 } else { rcx }) as u64; r8 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = ((r8.wrapping_add(P4)) as u64) as u64; fl_l = r8; fl_r = r8; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rdx }) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru32(r10.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((r10.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdi, rcx); rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84434<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rbp = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r15 = 0; fl_l = 0; fl_r = 0; r12 = P3; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rdi.wrapping_add(0x369); bb = 4; continue; }
            2 => { rsi = rdi.wrapping_add(0x369); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rsp.wrapping_add(0x40); rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rdx = rbp.wrapping_add(0x28); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rsp.wrapping_add(0x40); rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rdx = rbp.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rsp.wrapping_add(0x40); rcx = rbp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rdx = rbp.wrapping_add(0x78); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rdx, ((r15 as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = rbp.wrapping_add(0x7c); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x80); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x84); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x88); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 20; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_add(0x7cu64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x84u64), r15); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rdx = rbp.wrapping_add(0x8c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rdx = rsp.wrapping_add(0x40); rcx = rbp.wrapping_add(0x8c); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x50); rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84435<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = 0; fl_l = 0; fl_r = 0; r14 = P3; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rbx.wrapping_add(0x369); bb = 4; continue; }
            2 => { rsi = rbx.wrapping_add(0x369); fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rcx = rdi.wrapping_add(0x28); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rcx = rdi.wrapping_add(0x50); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x120u64), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 16; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rbp as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x120u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x7cu64), (xmm0) as u128); fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(0x120u64), (0x10u64) as u64)) as u64; fl_r = fl_l; bb = 20; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x7cu64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x84u64), rbp); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rcx = rdi.wrapping_add(0x8c); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84719<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x9u64))) as u32) as u32 as u64; rdi = r8.wrapping_add(0xb); r15 = rcx; rbx = rdx; rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(rsp.wrapping_add(0x60u64)))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rsi }) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = P0; rax = (((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as i8 as i32) as u32) as u32 as u64; rbp = ((rsi).wrapping_shr((((rax) as u32) & 63))) as u64; rsi = ((rsi).wrapping_sub(rbp)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { bb = 12; continue; }
            8 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = ((rax.wrapping_add(rdi)) as u64) as u64; bb = 10; continue; }
            10 => { r8 = r14; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = rsp.wrapping_add(0x58); r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { bb = 22; continue; }
            19 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rbp = ((rax.wrapping_add(rdi)) as u64) as u64; bb = 21; continue; }
            21 => { r8 = rbp; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = r15; crate::simd_ops::wu64(r15, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84722<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x9u64))) as u32) as u32 as u64; r12 = rcx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rbx = rdx; r15 = r8; rdx = (crate::simd_ops::ru32(r8)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rdi }) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = P0; rax = (((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as i8 as i32) as u32) as u32 as u64; rsi = ((rdi).wrapping_shr((((rax) as u32) & 63))) as u64; rdi = ((rdi).wrapping_sub(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0xfu64))) as u32) as u32 as u64; r14 = r8.wrapping_add(0xb); fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { bb = 11; continue; }
            8 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbp = ((rax.wrapping_add(r14)) as u64) as u64; bb = 10; continue; }
            10 => { r8 = rbp; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((((rsi as u32)) & (0xffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rsi as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rsi = (((rsi as u32)).wrapping_shr(8u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x30u8) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rcx = r14.wrapping_add(0x18); r8 = rbx; rdx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0xfu64))) as u32) as u32 as u64; rsi = r15.wrapping_add(0xb); fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 23; continue; }
            23 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 28; continue; }
            28 => { bb = 32; continue; }
            29 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { rbp = ((rax.wrapping_add(rsi)) as u64) as u64; bb = 31; continue; }
            31 => { r8 = rbp; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(r12, rbx); rax = r12; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_84969(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_85168(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_85590<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x48); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; r12 = r8; rsi = rdx; r15 = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rsp.wrapping_add(0x40); rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rdx = rbp.wrapping_sub(0x48); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r14 = (r14 & !0xffu64) | ((((rdi as u8)) as u8 as u64)); bb = 5; continue; }
            4 => { rcx = 0xff; r14 = 0x1; r14 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { (rcx as u32) } else { (r14 as u32) }) as u32 as u64; bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rdx = rsp.wrapping_add(0x40); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) <= 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; rax = (((crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); if (((fl_l as u64)) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rcx = 0x30; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = rsp.wrapping_add(0x20); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsi, rdi); rax = rsi; r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 15; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86129<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rbx); r14 = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r13 = r8; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = (xmm3) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = 0; fl_l = 0; fl_r = 0; r12 = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp, ((r15 as u32)) as u32); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x50u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { r8 = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_sub(0x2u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; r11 = ((rax.wrapping_add(rax)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((r9.wrapping_add(r8)) as u32) as u32 as u64; fl_l = (((r15 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(rdx.wrapping_mul(8)))) as u64; r15 = ((crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; bb = 9; continue; }
            9 => { rax = ((r9.wrapping_add(r8)) as u32) as u32 as u64; fl_l = (((r15 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            10 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = r13; r8 = rdi; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = rdx.wrapping_add(0x8); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(15u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbp, ((r15 as u32)) as u32); bb = 14; continue; }
            14 => { rsi = ((r12.wrapping_mul(4)) as u64) as u64; rcx = (0xffffffffffffff0u64) as u64; rax = rsi.wrapping_add(0xf); fl_l = rax; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = rcx; bb = 16; continue; }
            16 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r8 = ((r12.wrapping_mul(4)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rsp.wrapping_add(0x50); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r15 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(3u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (0xffffffffffffff0u64) as u64; bb = 18; continue; }
            18 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; r10 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x50u64))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; r14 = ((r10 as u32)) as u32 as u64; r12 = rsp.wrapping_add(0x50); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 42; continue; } bb = 19; continue; }
            19 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 65; continue; } bb = 20; continue; }
            20 => { r8 = ((rsi.wrapping_add(rbx)) as u64) as u64; r15 = rdi; bb = 21; continue; }
            21 => { r9 = (crate::simd_ops::ru64(r13)) as u64; r11 = ((r14 as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x8u64))) as u32) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 22; continue; }
            22 => { rdx = r10; r10 = ((rax as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((crate::simd_ops::ru16(r9.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r12.wrapping_add(rax.wrapping_mul(8)), rcx); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rdi = ((r12.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; r9 = (if (((fl_l as u64)) > ((fl_r as u64))) { r10 } else { r9 }) as u64; bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx; r11 = r10; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { fl_l = r9; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 27; continue; }
            27 => { rax = r9.wrapping_sub(0x1); rax = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r10 = rbx.wrapping_sub(0x4); r10 = ((r10.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = r10; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = r9; fl_l = (((rax) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r11 = ((r11).wrapping_add(0x10u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx, (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x30u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 31; continue; }
            31 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 33; continue; }
            33 => { rax = r8; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 34; continue; }
            34 => { rax = r8.wrapping_sub(0xc); bb = 35; continue; }
            35 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 36; continue; }
            36 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            38 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 39; continue; }
            39 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 40; continue; }
            40 => { r13 = ((r13).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 41; continue; }
            41 => { bb = 64; continue; }
            42 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 65; continue; } bb = 43; continue; }
            43 => { r8 = ((rsi.wrapping_add(rbx)) as u64) as u64; r15 = rdi; bb = 44; continue; }
            44 => { r9 = (crate::simd_ops::ru64(r13)) as u64; r11 = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(15u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 63; continue; } bb = 45; continue; }
            45 => { rdx = r10; r10 = ((rcx as u32)) as u32 as u64; bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((crate::simd_ops::ru16(r9.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = ((r14 as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r12.wrapping_add(rax.wrapping_mul(8)), rcx); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 46; continue; } bb = 47; continue; }
            47 => { r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rdi = ((r12.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; r9 = (if (((fl_l as u64)) > ((fl_r as u64))) { r10 } else { r9 }) as u64; bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx; r11 = r10; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 49; continue; }
            49 => { fl_l = r9; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 55; continue; } bb = 50; continue; }
            50 => { rax = r9.wrapping_sub(0x1); rax = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r10 = rbx.wrapping_sub(0x4); r10 = ((r10.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 52; continue; } bb = 51; continue; }
            51 => { fl_l = r10; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 54; continue; } bb = 52; continue; }
            52 => { rax = r9; fl_l = (((rax) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 53; continue; }
            53 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r11 = ((r11).wrapping_add(0x10u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx, (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x30u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = r11; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 53; continue; } bb = 54; continue; }
            54 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 55; continue; }
            55 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 62; continue; } bb = 56; continue; }
            56 => { rax = r8; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 60; continue; } bb = 57; continue; }
            57 => { rax = r8.wrapping_sub(0xc); bb = 58; continue; }
            58 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 58; continue; } bb = 59; continue; }
            59 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 62; continue; } bb = 60; continue; }
            60 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 61; continue; }
            61 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 61; continue; } bb = 62; continue; }
            62 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 63; continue; }
            63 => { r13 = ((r13).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 64; continue; }
            64 => { r15 = (crate::simd_ops::ru32(rbp)) as u32 as u64; bb = 65; continue; }
            65 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r8 = ((r15 as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rdx = rbx; rcx = r12; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_r = 0x80; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32) as u128; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 66; continue; }
            66 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            67 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            68 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86144<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rdi = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    rbx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86149<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rbx = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86720<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r12 = r9; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r13 = r8; rdi = rcx; r10 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); r8 = r13; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_86840<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((rsi as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_87469<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r9; rdi = r8; rsi = rdx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 9; continue; }
            2 => { rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; r8 = P1; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbp as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rbp.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 8; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rcx = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbp as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rbp as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { rax = 0x64; bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_87703<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x178u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; r8 = rsp.wrapping_add(0x40); xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm0 = 0u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88194<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; r12 = rcx.wrapping_add(0x8); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rbx = r9; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = r15; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = r15; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rbp; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rbx.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; r12 = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r14 as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = rbx.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdi = (((r13 as u8)) as u32) as u32 as u64; bb = 44; continue; }
            30 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r15 as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rsi = r15; bb = 33; continue; }
            33 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rdx = r15; bb = 40; continue; }
            40 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rsi = rdx.wrapping_sub(0x10); bb = 43; continue; }
            42 => { rsi = r15; bb = 43; continue; }
            43 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r15 as u32)) as u32); bb = 46; continue; }
            46 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { r13 = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 52; continue; }
            48 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rcx = rsi.wrapping_add(0x10); let xa_src: u32 = (r12 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r12 = (xa_old) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 53; continue; }
            53 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { r8 }) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(r9, r8); bb = 57; continue; }
            57 => { rcx = r12; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rax = ((r13 as u32)) as u32 as u64; bb = 59; continue; }
            59 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            60 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 61; continue; }
            61 => { rax = ((r13.wrapping_sub(0x5au64)) as u32) as u32 as u64; bb = 59; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88195<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r14 = r8; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rdi.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rsi; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 52; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r14 = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rbp as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 42; continue; }
            30 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((r14 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 38; continue; }
            38 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 42; continue; }
            41 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u32)) != 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 44; continue; }
            44 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 56; continue; }
            46 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 48; continue; }
            48 => { let xa_src: u32 = (r14 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r14 = (xa_old) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            50 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 51; continue; }
            51 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 55; continue; } bb = 53; continue; }
            53 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu64(r8, rbx); bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = ((r13 as u32)) as u32 as u64; bb = 57; continue; }
            57 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88388<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r14 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x268u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x270); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x2b0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a8u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x270); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x268u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x268u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x298u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x2a0u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x270); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rax = 0xf; bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88537<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = P1; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rbx.wrapping_add(P0)) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = P1; crate::simd_ops::wu64(rbx, rcx); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x20; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_88721<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((((rdx as u32)) & (0xfffffff0u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(P0)) as u128; rbx = 0x8; bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(r10.wrapping_mul(4)))) as u128; rax = ((rbx.wrapping_sub(0x4u64)) as u32) as u32 as u64; xmm0 = crate::simd_ops::vmulps(xmm0, xmm1); crate::simd_ops::wu128(r8.wrapping_add(r10.wrapping_mul(4)), (xmm0) as u128); r10 = (((r10 as u32)).wrapping_add(0x10u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); rax = ((rbx as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); rax = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x10u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = ((r11 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rbx = ((r10 as u32)) as u32 as u64; r9 = ((r9).wrapping_sub(r8)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((r11 as u32)) as u32 as u64; rcx = r8.wrapping_add(0x4); rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rcx = ((rcx.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; r10 = ((r10.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rbx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_sub(0x4u64))) as u32) as u128; rcx = rcx.wrapping_add(0x10); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x14u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_sub(0x10u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_sub(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(r9).wrapping_sub(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), ((xmm1 as u32)) as u32); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r11 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = ((r8.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rcx = ((r11 as u32)) as u32 as u64; bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(rax))) as u32) as u128; rax = rax.wrapping_add(0x4); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), ((xmm0 as u32)) as u32); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89472<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
            9 => { crate::simd_ops::wu8(rcx, ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), r12); crate::simd_ops::wu8(rbx.wrapping_sub(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rbx, r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89481<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x28); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx, (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx, (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rcx, ((rbp as u8)) as u8); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89516<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = rcx; crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P3))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P4))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if ((((fl_l as u16)) as i16) <= (((fl_r as u16)) as i16)) { bb = 11; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = ((rbx as u32)) as u32 as u64; fl_l = ((rdi).wrapping_shl(3u32)) as u64; rdi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rdi = ((rdi.wrapping_add(P7)) as u64) as u64; rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rax))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 11; continue; }
            11 => { rcx = ((rsi.wrapping_add(P9)) as u64) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P10), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89668<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffecu64) as u64; r14 = ((r14).wrapping_sub(rbp)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = rbx.wrapping_sub(0x14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rcx, ((r12 as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_sub(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r12); bb = 10; continue; }
            10 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89678<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rbp = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x18); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x4u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x4u64), (0x0u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x0u64) as u64); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rcx, ((rbp as u8)) as u8); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_89773<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm6) as u128); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P1), rbx); rbx = rcx; crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rcx = ((rax.wrapping_add(P5)) as u64) as u64; rdx = rdi; crate::simd_ops::wu64(rax.wrapping_add(P6), rbp); crate::simd_ops::wu64(rax.wrapping_add(P7), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, rdi); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; r9 = ((rsi as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x150u64))) as u64; xmm1 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P14))).to_bits() as u128); bb = 6; continue; }
            4 => { xmm6 = 0u128; bb = 6; continue; }
            5 => { xmm6 = ((crate::simd_ops::ru32(P15)) as u32) as u128; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_90506<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); bb = 15; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 15; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r8 = rdi; rax = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = rax; rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 15; continue; }
            12 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(r8, rcx); bb = 15; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_90795<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rbx = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = ((rsi.wrapping_mul(8)) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xffffffffu32) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rbp = ((rcx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; r14 = ((rcx.wrapping_add(r15)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru64(rdi)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = rbx; rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_90980<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rsi = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x2u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; r8 = ((rcx.wrapping_add(P2)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; rdx = rsp.wrapping_add(0x30); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32) as u128; r8 = ((rdi.wrapping_add(P9)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32) as u128; rdx = rsp.wrapping_add(0x30); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_92687<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r14 = r8; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rbx = ((rax.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) < ((fl_r as u32))) { (r15 as u32) } else { (rbx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = ((rbx as u32)) as u32 as u64; r8 = ((r13.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((r13 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rsi; rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rbp), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdx.wrapping_add(rbp).wrapping_add(0x8u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rdx.wrapping_add(rbp).wrapping_add(0x8u64), r13); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = rsi; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbp)) as u64; r12 = ((r12).wrapping_shl(4u32)) as u64; fl_l = ((r12).wrapping_add(rbp)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rcx = ((rdx as u32)) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdx.wrapping_add(rcx).wrapping_add(0x8u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rdx.wrapping_add(rcx).wrapping_add(0x8u64), r13); bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = r15; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rbx = rsi; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
