#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70007f0e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f0e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f0e58); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f0e5e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f0e60); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f0e62); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f0e63); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f0e64); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f0e65); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f0e66); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70007f0e67); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70007f0e6b); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2db0)) else { return RegfileExit::fault(0x70007f0e6f); };
            c.w64(6, t1);
            let Some(t2) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x70007f0e76); };
            c.cmp(t2, 0x0u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let r3 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f0e86); };
            c.w64(6, t4);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(8));
            let r5 = c.sub(c.r64(9), c.r64(7), 64);
            c.w64(9, r5);
            c.w64(0, c.r64(9));
            if let Some(sr) = c.shr(c.r64(0), 0xcu64, 64) { c.w64(0, sr); }
            let Some(t6) = c.ld64(c.r64(6).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f0e9d); };
            c.w64(6, t6);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, c.r64(9));
            if let Some(sr) = c.shr(c.r64(7), 0x5u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let r7 = c.r64(7) & 0x7cu64;
            c.w64(7, r7);
            c.logic(r7, 64);
            let r8 = c.r64(9) & 0x1fu64;
            c.w64(9, r8);
            c.logic(r8, 64);
            let r9 = c.add(c.r64(6), c.r64(7), 64);
            c.w64(6, r9);
            c.w32(7, 0x1u64);
            c.w64(1, c.r64(9));
            if let Some(sr) = c.shl(c.r64(7), c.r8(1), 64) { c.w64(7, sr); }
            let Some(t10) = c.ld32(c.r64(6)) else { return RegfileExit::fault(0x70007f0ec8); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.r64(7) | c.r64(8);
            c.w64(7, r11);
            c.logic(r11, 64);
            if !c.st32(c.r64(6), c.r32(7)) { return RegfileExit::fault(0x70007f0ece); }
            blk = 19; continue;
        }
        4 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f0ed5); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f0ed6); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f0ed7); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f0ed8); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f0ed9); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f0eda); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f0edb); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f0edd); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f0edf); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f0ee1); }
            let r12 = c.sub(c.r64(4), 0x100u64, 64);
            c.w64(4, r12);
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007f0eea); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x70007f0eee); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x70007f0ef3); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x70007f0ef8); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x70007f0efd); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x70007f0f02); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x70007f0f07); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x70007f0f0c); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 8) { return RegfileExit::fault(0x70007f0f11); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 9) { return RegfileExit::fault(0x70007f0f1a); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xa0), 10) { return RegfileExit::fault(0x70007f0f23); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xb0), 11) { return RegfileExit::fault(0x70007f0f2c); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xc0), 12) { return RegfileExit::fault(0x70007f0f35); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xd0), 13) { return RegfileExit::fault(0x70007f0f3e); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xe0), 14) { return RegfileExit::fault(0x70007f0f47); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xf0), 15) { return RegfileExit::fault(0x70007f0f50); }
            c.w64(10, c.r64(4));
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            let r14 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r14);
            c.logic(r14, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f0f64); }
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1388)) else { return RegfileExit::fault(0x70007f0f6f); };
            c.w64(0, t15);
            c.w64(10, 0x70007f0f7du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f0f7d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f0f81); }
            return c.call(c.r64(0), 0x70007f0f87, 0x70007f0f85);
        }
        5 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f0f87); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f0f8f); };
            c.w64(4, t16);
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x70007f0f94); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007f0f98); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007f0f9d); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007f0fa2); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007f0fa7); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007f0fac); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007f0fb1); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007f0fb6); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007f0fbb); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007f0fc4); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007f0fcd); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007f0fd6); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007f0fdf); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007f0fe8); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007f0ff1); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007f0ffa); }
            let r17 = c.add(c.r64(4), 0x100u64, 64);
            c.w64(4, r17);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f100a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f100c); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f100e); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1010); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1012); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1013); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1014); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1015); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1016); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1017); }
            blk = 19; continue;
        }
        6 => {
            let Some(t18) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70007f101d); };
            c.w32(6, t18);
            let r19 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r19);
            let r20 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f102a); };
            c.logic(t21 & 0x18u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(7));
            let r22 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r22);
            c.logic(r22, 64);
            let Some(t23) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f103e); };
            c.logic(t23 & 0x18u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, c.r64(7));
            let r24 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r24);
            c.logic(r24, 64);
            let Some(t25) = c.ld64(c.r64(6).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f1052); };
            c.w64(9, t25);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(8));
            let r26 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r26);
            c.w64(2, c.r64(0));
            if let Some(sr) = c.shr(c.r64(2), 0xcu64, 64) { c.w64(2, sr); }
            let Some(t27) = c.ld64(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f1069); };
            c.w64(9, t27);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(6, c.r64(0));
            if let Some(sr) = c.shr(c.r64(6), 0x5u64, 64) { c.w64(6, sr); }
            if let Some(sr) = c.shr(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            let r28 = c.r64(6) & 0x7cu64;
            c.w64(6, r28);
            c.logic(r28, 64);
            let r29 = c.r64(0) & 0x1fu64;
            c.w64(0, r29);
            c.logic(r29, 64);
            let r30 = c.add(c.r64(6), c.r64(9), 64);
            c.w64(6, r30);
            c.w64(1, c.r64(0));
            c.w32(0, 0x1u64);
            if let Some(sr) = c.shl(c.r64(0), c.r8(1), 64) { c.w64(0, sr); }
            let Some(t31) = c.ld32(c.r64(6)) else { return RegfileExit::fault(0x70007f1094); };
            c.w64(2, (t31 as u32 as i32 as i64 as u64));
            let r32 = c.r64(0) | c.r64(2);
            c.w64(0, r32);
            c.logic(r32, 64);
            if !c.st32(c.r64(6), c.r32(0)) { return RegfileExit::fault(0x70007f109a); }
            blk = 13; continue;
        }
        11 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f10a1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f10a2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f10a3); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f10a4); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f10a5); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f10a6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f10a7); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f10a9); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f10ab); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f10ad); }
            let r33 = c.sub(c.r64(4), 0x100u64, 64);
            c.w64(4, r33);
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007f10b6); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x70007f10ba); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x70007f10bf); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x70007f10c4); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x70007f10c9); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x70007f10ce); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x70007f10d3); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x70007f10d8); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 8) { return RegfileExit::fault(0x70007f10dd); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 9) { return RegfileExit::fault(0x70007f10e6); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xa0), 10) { return RegfileExit::fault(0x70007f10ef); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xb0), 11) { return RegfileExit::fault(0x70007f10f8); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xc0), 12) { return RegfileExit::fault(0x70007f1101); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xd0), 13) { return RegfileExit::fault(0x70007f110a); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xe0), 14) { return RegfileExit::fault(0x70007f1113); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xf0), 15) { return RegfileExit::fault(0x70007f111c); }
            c.w64(10, c.r64(4));
            let r34 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            let r35 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r35);
            c.logic(r35, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f1130); }
            c.w64(1, c.r64(6));
            c.w64(2, c.r64(8));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1388)) else { return RegfileExit::fault(0x70007f113b); };
            c.w64(0, t36);
            c.w64(10, 0x70007f1149u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f1149); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f114d); }
            return c.call(c.r64(0), 0x70007f1153, 0x70007f1151);
        }
        12 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f1153); }
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f115b); };
            c.w64(4, t37);
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x70007f1160); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007f1164); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007f1169); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007f116e); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007f1173); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007f1178); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007f117d); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007f1182); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007f1187); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007f1190); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007f1199); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007f11a2); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007f11ab); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007f11b4); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007f11bd); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007f11c6); }
            let r38 = c.add(c.r64(4), 0x100u64, 64);
            c.w64(4, r38);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f11d6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f11d8); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f11da); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f11dc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f11de); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f11df); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f11e0); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f11e1); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f11e2); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f11e3); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f11e4); };
            c.w64(8, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f11e8); };
            c.w64(7, t40);
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.w64(2, c.r64(8));
            let Some(t41) = c.ld32(c.r64(2)) else { return RegfileExit::fault(0x70007f11f5); };
            c.w32(6, t41);
            let r42 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r42);
            c.w64(8, c.r64(6));
            if let Some(sr) = c.shr(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(6));
            if let Some(sr) = c.shr(c.r64(9), 0x5u64, 64) { c.w64(9, sr); }
            let r43 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r43);
            c.logic(r43, 64);
            let r44 = c.r64(8) & 0x1fu64;
            c.w64(8, r44);
            c.logic(r44, 64);
            let r45 = c.r64(9) & 0x1ffcu64;
            c.w64(9, r45);
            c.logic(r45, 64);
            c.w64(0, c.r64(6).wrapping_add(0x110));
            c.w64(1, c.r64(8));
            c.w32(8, 0x1u64);
            if let Some(sr) = c.shl(c.r64(8), c.r8(1), 64) { c.w64(8, sr); }
            let Some(t46) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70007f122d); };
            c.logic(t46 & c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t47) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1233); };
            c.logic(t47 & 0x40u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(6, c.r64(7));
            let r48 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1247); };
            c.logic(t49 & 0x58u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1251); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1252); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1253); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1254); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1255); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1256); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f1257); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f1259); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f125b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f125d); }
            let r50 = c.sub(c.r64(4), 0x100u64, 64);
            c.w64(4, r50);
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007f1266); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x70007f126a); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x70007f126f); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x70007f1274); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x70007f1279); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x70007f127e); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x70007f1283); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x70007f1288); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 8) { return RegfileExit::fault(0x70007f128d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 9) { return RegfileExit::fault(0x70007f1296); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xa0), 10) { return RegfileExit::fault(0x70007f129f); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xb0), 11) { return RegfileExit::fault(0x70007f12a8); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xc0), 12) { return RegfileExit::fault(0x70007f12b1); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xd0), 13) { return RegfileExit::fault(0x70007f12ba); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xe0), 14) { return RegfileExit::fault(0x70007f12c3); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xf0), 15) { return RegfileExit::fault(0x70007f12cc); }
            c.w64(10, c.r64(4));
            let r51 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            let r52 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r52);
            c.logic(r52, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f12e0); }
            c.w64(1, c.r64(7));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1618)) else { return RegfileExit::fault(0x70007f12e8); };
            c.w64(0, t53);
            c.w64(10, 0x70007f12f6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f12f6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f12fa); }
            return c.call(c.r64(0), 0x70007f1300, 0x70007f12fe);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f1300); }
            let Some(t54) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f1308); };
            c.w64(4, t54);
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x70007f130d); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007f1311); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007f1316); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007f131b); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007f1320); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007f1325); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007f132a); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007f132f); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007f1334); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007f133d); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007f1346); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007f134f); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007f1358); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007f1361); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007f136a); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007f1373); }
            let r55 = c.add(c.r64(4), 0x100u64, 64);
            c.w64(4, r55);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f1383); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f1385); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1387); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1389); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f138b); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f138c); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f138d); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f138e); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f138f); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1390); }
            blk = 19; continue;
        }
        19 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f1391); };
            c.w64(0, t56);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1398); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1399); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f139a); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f139b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f139c); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f139d); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f139f); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f13a4); }
            return c.ret(0x0, 0x70007f13a5);
        }
        20 => {
            return RegfileExit::jump(0x70007f13a8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f13b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f13b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f13b8); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f13ba); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f13bc); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f13be); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f13bf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f13c0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f13c1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f13c2); }
            let Some(t0) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70007f13c3); };
            c.w32(6, t0);
            let r1 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1);
            c.w64(9, c.r64(6));
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            c.w64(0, c.r64(6));
            if let Some(sr) = c.shr(c.r64(0), 0x5u64, 64) { c.w64(0, sr); }
            let r2 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r2);
            c.logic(r2, 64);
            let r3 = c.r64(9) & 0x1fu64;
            c.w64(9, r3);
            c.logic(r3, 64);
            let r4 = c.r64(0) & 0x1ffcu64;
            c.w64(0, r4);
            c.logic(r4, 64);
            c.w64(2, c.r64(6).wrapping_add(0x110));
            c.w64(1, c.r64(9));
            c.w32(9, 0x1u64);
            if let Some(sr) = c.shl(c.r64(9), c.r8(1), 64) { c.w64(9, sr); }
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70007f13fb); };
            c.logic(t5 & c.r32(9), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t6) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1401); };
            c.logic(t6 & 0x40u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(7));
            let r7 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1415); };
            c.logic(t8 & 0x58u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f141f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1420); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1421); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1422); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1423); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1424); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f1425); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f1427); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f1429); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f142b); }
            let r9 = c.sub(c.r64(4), 0x100u64, 64);
            c.w64(4, r9);
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007f1434); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x70007f1438); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x70007f143d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x70007f1442); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x70007f1447); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x70007f144c); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x70007f1451); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x70007f1456); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 8) { return RegfileExit::fault(0x70007f145b); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 9) { return RegfileExit::fault(0x70007f1464); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xa0), 10) { return RegfileExit::fault(0x70007f146d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xb0), 11) { return RegfileExit::fault(0x70007f1476); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xc0), 12) { return RegfileExit::fault(0x70007f147f); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xd0), 13) { return RegfileExit::fault(0x70007f1488); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xe0), 14) { return RegfileExit::fault(0x70007f1491); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xf0), 15) { return RegfileExit::fault(0x70007f149a); }
            c.w64(10, c.r64(4));
            let r10 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            let r11 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r11);
            c.logic(r11, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f14ae); }
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1618)) else { return RegfileExit::fault(0x70007f14b9); };
            c.w64(0, t12);
            c.w64(10, 0x70007f14c7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f14c7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f14cb); }
            return c.call(c.r64(0), 0x70007f14d1, 0x70007f14cf);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f14d1); }
            let Some(t13) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f14d9); };
            c.w64(4, t13);
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x70007f14de); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007f14e2); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007f14e7); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007f14ec); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007f14f1); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007f14f6); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007f14fb); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007f1500); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007f1505); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007f150e); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007f1517); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007f1520); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007f1529); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007f1532); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007f153b); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007f1544); }
            let r14 = c.add(c.r64(4), 0x100u64, 64);
            c.w64(4, r14);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f1554); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f1556); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1558); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f155a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f155c); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f155d); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f155e); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f155f); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1560); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1561); }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f1562); };
            c.w64(0, t15);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1569); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f156a); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f156b); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f156c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f156d); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f156e); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1570); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1575); }
            return c.ret(0x0, 0x70007f1576);
        }
        6 => {
            return RegfileExit::jump(0x70007f1578);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1594); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f1598); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f159e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f15a0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f15a2); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f15a3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f15a4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f15a5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f15a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70007f15a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70007f15ab); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2db0)) else { return RegfileExit::fault(0x70007f15af); };
            c.w64(6, t1);
            let Some(t2) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x70007f15b6); };
            c.cmp(t2, 0x0u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let r3 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld64(c.r64(7).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f15c6); };
            c.w64(6, t4);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(8));
            let r5 = c.sub(c.r64(9), c.r64(7), 64);
            c.w64(9, r5);
            c.w64(0, c.r64(9));
            if let Some(sr) = c.shr(c.r64(0), 0xcu64, 64) { c.w64(0, sr); }
            let Some(t6) = c.ld64(c.r64(6).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f15dd); };
            c.w64(6, t6);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, c.r64(9));
            if let Some(sr) = c.shr(c.r64(7), 0x5u64, 64) { c.w64(7, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let r7 = c.r64(7) & 0x7cu64;
            c.w64(7, r7);
            c.logic(r7, 64);
            let r8 = c.r64(9) & 0x1fu64;
            c.w64(9, r8);
            c.logic(r8, 64);
            let r9 = c.add(c.r64(6), c.r64(7), 64);
            c.w64(6, r9);
            c.w32(7, 0x1u64);
            c.w64(1, c.r64(9));
            if let Some(sr) = c.shl(c.r64(7), c.r8(1), 64) { c.w64(7, sr); }
            let Some(t10) = c.ld32(c.r64(6)) else { return RegfileExit::fault(0x70007f1608); };
            c.w64(8, (t10 as u32 as i32 as i64 as u64));
            let r11 = c.r64(7) | c.r64(8);
            c.w64(7, r11);
            c.logic(r11, 64);
            if !c.st32(c.r64(6), c.r32(7)) { return RegfileExit::fault(0x70007f160e); }
            blk = 19; continue;
        }
        4 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1615); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1616); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1617); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1618); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1619); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f161a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f161b); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f161d); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f161f); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f1621); }
            c.w64(10, c.r64(4));
            let r12 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r12);
            let r13 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r13);
            c.logic(r13, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f162e); }
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1388)) else { return RegfileExit::fault(0x70007f1639); };
            c.w64(0, t14);
            c.w64(10, 0x70007f1647u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f1647); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f164b); }
            return c.call(c.r64(0), 0x70007f1651, 0x70007f164f);
        }
        5 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f1651); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f1659); };
            c.w64(4, t15);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f165e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f1660); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1662); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1664); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1666); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1667); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1668); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1669); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f166a); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f166b); }
            blk = 19; continue;
        }
        6 => {
            let Some(t16) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70007f1671); };
            c.w32(6, t16);
            let r17 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r17);
            let r18 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r18);
            c.logic(r18, 64);
            let Some(t19) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f167e); };
            c.logic(t19 & 0x18u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(6, c.r64(7));
            let r20 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r20);
            c.logic(r20, 64);
            let Some(t21) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1692); };
            c.logic(t21 & 0x18u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, c.r64(7));
            let r22 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r22);
            c.logic(r22, 64);
            let Some(t23) = c.ld64(c.r64(6).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f16a6); };
            c.w64(9, t23);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(8));
            let r24 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r24);
            c.w64(2, c.r64(0));
            if let Some(sr) = c.shr(c.r64(2), 0xcu64, 64) { c.w64(2, sr); }
            let Some(t25) = c.ld64(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f16bd); };
            c.w64(9, t25);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(6, c.r64(0));
            if let Some(sr) = c.shr(c.r64(6), 0x5u64, 64) { c.w64(6, sr); }
            if let Some(sr) = c.shr(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            let r26 = c.r64(6) & 0x7cu64;
            c.w64(6, r26);
            c.logic(r26, 64);
            let r27 = c.r64(0) & 0x1fu64;
            c.w64(0, r27);
            c.logic(r27, 64);
            let r28 = c.add(c.r64(6), c.r64(9), 64);
            c.w64(6, r28);
            c.w64(1, c.r64(0));
            c.w32(0, 0x1u64);
            if let Some(sr) = c.shl(c.r64(0), c.r8(1), 64) { c.w64(0, sr); }
            let Some(t29) = c.ld32(c.r64(6)) else { return RegfileExit::fault(0x70007f16e8); };
            c.w64(2, (t29 as u32 as i32 as i64 as u64));
            let r30 = c.r64(0) | c.r64(2);
            c.w64(0, r30);
            c.logic(r30, 64);
            if !c.st32(c.r64(6), c.r32(0)) { return RegfileExit::fault(0x70007f16ee); }
            blk = 13; continue;
        }
        11 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f16f2); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f16f3); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f16f4); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f16f5); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f16f6); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f16f7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f16f8); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f16fa); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f16fc); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f16fe); }
            c.w64(10, c.r64(4));
            let r31 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            let r32 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r32);
            c.logic(r32, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f170b); }
            c.w64(1, c.r64(6));
            c.w64(2, c.r64(8));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1388)) else { return RegfileExit::fault(0x70007f1716); };
            c.w64(0, t33);
            c.w64(10, 0x70007f1724u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f1724); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f1728); }
            return c.call(c.r64(0), 0x70007f172e, 0x70007f172c);
        }
        12 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f172e); }
            let Some(t34) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f1736); };
            c.w64(4, t34);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f173b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f173d); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f173f); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1741); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1743); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1744); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1745); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1746); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1747); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1748); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f1749); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f174d); };
            c.w64(7, t36);
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0x1u64);
            blk = 14; continue;
        }
        14 => {
            c.w64(2, c.r64(8));
            let Some(t37) = c.ld32(c.r64(2)) else { return RegfileExit::fault(0x70007f175a); };
            c.w32(6, t37);
            let r38 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r38);
            c.w64(8, c.r64(6));
            if let Some(sr) = c.shr(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(6));
            if let Some(sr) = c.shr(c.r64(9), 0x5u64, 64) { c.w64(9, sr); }
            let r39 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r39);
            c.logic(r39, 64);
            let r40 = c.r64(8) & 0x1fu64;
            c.w64(8, r40);
            c.logic(r40, 64);
            let r41 = c.r64(9) & 0x1ffcu64;
            c.w64(9, r41);
            c.logic(r41, 64);
            c.w64(0, c.r64(6).wrapping_add(0x110));
            c.w64(1, c.r64(8));
            c.w32(8, 0x1u64);
            if let Some(sr) = c.shl(c.r64(8), c.r8(1), 64) { c.w64(8, sr); }
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70007f1792); };
            c.logic(t42 & c.r32(8), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t43) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1798); };
            c.logic(t43 & 0x40u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(6, c.r64(7));
            let r44 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f17a8); };
            c.logic(t45 & 0x58u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f17ae); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f17af); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f17b0); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f17b1); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f17b2); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f17b3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f17b4); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f17b6); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f17b8); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f17ba); }
            c.w64(10, c.r64(4));
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            let r47 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r47);
            c.logic(r47, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f17c7); }
            c.w64(1, c.r64(7));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1618)) else { return RegfileExit::fault(0x70007f17cf); };
            c.w64(0, t48);
            c.w64(10, 0x70007f17ddu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f17dd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f17e1); }
            return c.call(c.r64(0), 0x70007f17e7, 0x70007f17e5);
        }
        18 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f17e7); }
            let Some(t49) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f17ef); };
            c.w64(4, t49);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f17f4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f17f6); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f17f8); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f17fa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f17fc); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f17fd); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f17fe); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f17ff); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1800); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1801); }
            blk = 19; continue;
        }
        19 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f1802); };
            c.w64(0, t50);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1809); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f180a); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f180b); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f180c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f180d); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f180e); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1810); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1815); }
            return c.ret(0x0, 0x70007f1816);
        }
        20 => {
            return RegfileExit::jump(0x70007f1818);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1834); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f1838); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f183a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f183c); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f183e); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f183f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1840); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1841); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1842); }
            let Some(t0) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70007f1843); };
            c.w32(6, t0);
            let r1 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1);
            c.w64(9, c.r64(6));
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            c.w64(0, c.r64(6));
            if let Some(sr) = c.shr(c.r64(0), 0x5u64, 64) { c.w64(0, sr); }
            let r2 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r2);
            c.logic(r2, 64);
            let r3 = c.r64(9) & 0x1fu64;
            c.w64(9, r3);
            c.logic(r3, 64);
            let r4 = c.r64(0) & 0x1ffcu64;
            c.w64(0, r4);
            c.logic(r4, 64);
            c.w64(2, c.r64(6).wrapping_add(0x110));
            c.w64(1, c.r64(9));
            c.w32(9, 0x1u64);
            if let Some(sr) = c.shl(c.r64(9), c.r8(1), 64) { c.w64(9, sr); }
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70007f187b); };
            c.logic(t5 & c.r32(9), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t6) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1881); };
            c.logic(t6 & 0x40u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(7));
            let r7 = c.r64(6) & 0xfffffffffffc0000u64;
            c.w64(6, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1891); };
            c.logic(t8 & 0x58u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1897); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1898); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1899); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f189a); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f189b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f189c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f189d); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f189f); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f18a1); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f18a3); }
            c.w64(10, c.r64(4));
            let r9 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r9);
            let r10 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r10);
            c.logic(r10, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f18b0); }
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1618)) else { return RegfileExit::fault(0x70007f18bb); };
            c.w64(0, t11);
            c.w64(10, 0x70007f18c9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f18c9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f18cd); }
            return c.call(c.r64(0), 0x70007f18d3, 0x70007f18d1);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f18d3); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f18db); };
            c.w64(4, t12);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f18e0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f18e2); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f18e4); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f18e6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f18e8); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f18e9); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f18ea); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f18eb); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f18ec); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f18ed); }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f18ee); };
            c.w64(0, t13);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f18f5); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f18f6); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f18f7); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f18f8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f18f9); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f18fa); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f18fc); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1901); }
            return c.ret(0x0, 0x70007f1902);
        }
        6 => {
            return RegfileExit::jump(0x70007f1904);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f1918); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f191a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f191c); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f191e); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f191f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1920); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1921); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1922); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1923); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1924); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1925); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1926); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1927); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1928); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f1929); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f192b); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f192d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f192f); }
            let r0 = c.sub(c.r64(4), 0x100u64, 64);
            c.w64(4, r0);
            if !c.mov128_mr(c.r64(4), 0) { return RegfileExit::fault(0x70007f1938); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 1) { return RegfileExit::fault(0x70007f193c); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 2) { return RegfileExit::fault(0x70007f1941); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 3) { return RegfileExit::fault(0x70007f1946); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 4) { return RegfileExit::fault(0x70007f194b); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 5) { return RegfileExit::fault(0x70007f1950); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 6) { return RegfileExit::fault(0x70007f1955); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 7) { return RegfileExit::fault(0x70007f195a); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 8) { return RegfileExit::fault(0x70007f195f); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 9) { return RegfileExit::fault(0x70007f1968); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xa0), 10) { return RegfileExit::fault(0x70007f1971); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xb0), 11) { return RegfileExit::fault(0x70007f197a); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xc0), 12) { return RegfileExit::fault(0x70007f1983); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xd0), 13) { return RegfileExit::fault(0x70007f198c); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xe0), 14) { return RegfileExit::fault(0x70007f1995); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0xf0), 15) { return RegfileExit::fault(0x70007f199e); }
            c.w64(10, c.r64(4));
            let r1 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r1);
            let r2 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r2);
            c.logic(r2, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f19b2); }
            c.w64(6, c.r64(13));
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(6));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x12b0)) else { return RegfileExit::fault(0x70007f19c3); };
            c.w64(0, t3);
            c.w64(10, 0x70007f19d1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f19d1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f19d5); }
            return c.call(c.r64(0), 0x70007f19db, 0x70007f19d9);
        }
        1 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f19db); }
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f19e3); };
            c.w64(4, t4);
            if !c.mov128_rm(0, c.r64(4)) { return RegfileExit::fault(0x70007f19e8); }
            if !c.mov128_rm(1, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007f19ec); }
            if !c.mov128_rm(2, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007f19f1); }
            if !c.mov128_rm(3, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007f19f6); }
            if !c.mov128_rm(4, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007f19fb); }
            if !c.mov128_rm(5, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007f1a00); }
            if !c.mov128_rm(6, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007f1a05); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007f1a0a); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007f1a0f); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007f1a18); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0xa0)) { return RegfileExit::fault(0x70007f1a21); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0xb0)) { return RegfileExit::fault(0x70007f1a2a); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0xc0)) { return RegfileExit::fault(0x70007f1a33); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0xd0)) { return RegfileExit::fault(0x70007f1a3c); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0xe0)) { return RegfileExit::fault(0x70007f1a45); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0xf0)) { return RegfileExit::fault(0x70007f1a4e); }
            let r5 = c.add(c.r64(4), 0x100u64, 64);
            c.w64(4, r5);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f1a5e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f1a60); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1a62); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1a64); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1a66); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1a67); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1a68); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1a69); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1a6a); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1a6b); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f1a6c); };
            c.w64(0, t6);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1a73); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1a74); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1a75); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1a76); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1a77); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1a78); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1a7a); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1a7f); }
            return c.ret(0x0, 0x70007f1a80);
        }
        2 => {
            return RegfileExit::jump(0x70007f1a84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1a94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f1a98); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f1a9a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f1a9c); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1a9e); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1a9f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1aa0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1aa1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1aa2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1aa3); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1aa4); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1aa5); }
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1aa6); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1aa7); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1aa8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f1aa9); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f1aab); }
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f1aad); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007f1aaf); }
            c.w64(10, c.r64(4));
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let r1 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r1);
            c.logic(r1, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f1abc); }
            c.w64(6, c.r64(13));
            c.w64(1, c.r64(7));
            c.w64(2, c.r64(8));
            c.w64(8, c.r64(6));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x12b0)) else { return RegfileExit::fault(0x70007f1acd); };
            c.w64(0, t2);
            c.w64(10, 0x70007f1adbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70007f1adb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70007f1adf); }
            return c.call(c.r64(0), 0x70007f1ae5, 0x70007f1ae3);
        }
        1 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70007f1ae5); }
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70007f1aed); };
            c.w64(4, t3);
            if !c.pop64_reg(11) { return RegfileExit::fault(0x70007f1af2); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f1af4); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1af6); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1af8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1afa); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1afb); }
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1afc); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1afd); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1afe); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1aff); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70007f1b00); };
            c.w64(0, t4);
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1b07); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f1b08); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f1b09); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1b0a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1b0b); }
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f1b0c); }
            if !c.pop64_reg(9) { return RegfileExit::fault(0x70007f1b0e); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1b13); }
            return c.ret(0x0, 0x70007f1b14);
        }
        2 => {
            return RegfileExit::jump(0x70007f1b18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1b34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1b34); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f1b3a); };
            c.w32(6, t2);
            let r3 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f1b40); };
            c.w32(1, t4);
            c.cmp(c.r16(1), 0xffffu64, 16);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_ge() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(1));
            blk = 3; continue;
        }
        3 => {
            let r5 = c.add(c.r32(0), 0x5u64, 32);
            c.w32(0, r5);
            c.w32(1, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f1b58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1b59); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f1b5a); }
            let Some(t6) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f1b5b); };
            if !c.push64(t6) { return RegfileExit::fault(0x70007f1b5b); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f1b5f); };
            c.w64(10, t7);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70007f1b66); }
            return RegfileExit::jump(0x700086ee74);
        }
        4 => {
            return RegfileExit::jump(0x70007f1b74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1b94); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1b9a); };
            c.logic(t2 & 0x400u64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f1ba6); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1bac); };
            c.logic(t5 & 0x60u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f1bb6); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f1bbc); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f1bc2); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(1)) { return RegfileExit::fault(0x70007f1bc8); }
            blk = 3; continue;
        }
        3 => {
            let Some(t12) = c.ld16(c.r64(2).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f1bcd); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007f1bd1); };
            c.w64(10, t13);
            let Some(t14) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007f1bd8); };
            c.cmp(t14, 0x0u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1be2); };
            c.w64(2, t15);
            c.cmp(c.r32(3), 0xffffu64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let r16 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r16);
            if c.cc_le() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(10, c.r64(4));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f1c01); };
            let r18 = c.sub(c.r64(10), t17, 64);
            c.w64(10, r18);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(3), 64);
            if c.cc_le() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(3).wrapping_mul(8));
            blk = 9; continue;
        }
        8 => {
            let r19 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r19);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f1c29); }
            let r20 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r20);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 10; continue;
        }
        10 => {
            let r21 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r21);
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f1c47); };
            c.w64(10, t22);
            if !c.st64(c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f1c4b); }
            let r23 = c.inc(c.r64(11), 64);
            c.w64(11, r23);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_l() { blk = 11; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1c5b); };
            c.w64(10, t24);
            blk = 13; continue;
        }
        13 => {
            let r25 = c.dec(c.r64(3), 64);
            c.w64(3, r25);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f1c65); }
            if c.cc_g() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 18; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1c70); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1c74); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f1c78); };
            c.w64(3, t26);
            return c.call(0x700086ed74, 0x70007f1c84, 0x70007f1c7f);
        }
        16 => {
            return RegfileExit::fault(0x70007f1c84);
        }
        17 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1c88); }
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f1c89); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1c8f); };
            c.logic(t29 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let r30 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r30);
            blk = 21; continue;
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f1ca5); };
            c.w32(1, t31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007f1ca8); };
            c.w64(1, t32);
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::jump(c.r64(1));
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1cb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1cb8); }
            let r33 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r33);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1cbc); }
            let r34 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r34);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1cbf); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1cc6); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1cc7); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f1cc8); };
            if !c.push64(t35) { return RegfileExit::fault(0x70007f1cc8); }
            c.w32(0, 0x2u64);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007f1cd0); };
            c.w64(3, t36);
            return c.call(0x700086ed74, 0x70007f1cdc, 0x70007f1cd7);
        }
        24 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1cdc); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1cdd); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1ce4); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1cee); }
            blk = 4; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1cf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1cf8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1cfa); }
            c.w32(0, 0x1u64);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1da0)) else { return RegfileExit::fault(0x70007f1d00); };
            c.w64(3, t37);
            return c.call(0x700086ed74, 0x70007f1d0c, 0x70007f1d07);
        }
        26 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1d0f); }
            return RegfileExit::jump(0x70007f1d10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1d14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1d14); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1d1a); };
            c.logic(t2 & 0x400u64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f1d26); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1d2c); };
            c.logic(t5 & 0x60u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1d36); };
            c.w64(1, t6);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f1d40); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f1d46); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1d52); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1d56); }
            let r10 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1d5a); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1d5b); }
            c.w64(0, c.r64(1));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1d5f); }
            return c.call(0x70008a2054, 0x70007f1d65, 0x70007f1d60);
        }
        5 => {
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1d65); }
            c.w64(1, c.r64(0));
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1d69); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1d6a); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1d74); }
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1d75); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(1)) { return RegfileExit::fault(0x70007f1d7b); }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld16(c.r64(2).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f1d80); };
            c.w32(3, t13);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007f1d84); };
            c.w64(10, t14);
            let Some(t15) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007f1d8b); };
            c.cmp(t15, 0x0u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1d95); };
            c.w64(2, t16);
            c.cmp(c.r32(3), 0xffffu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 8; continue;
        }
        8 => {
            let r17 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r17);
            if c.cc_le() { blk = 21; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(10, c.r64(4));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f1db4); };
            let r19 = c.sub(c.r64(10), t18, 64);
            c.w64(10, r19);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(3), 64);
            if c.cc_le() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(3).wrapping_mul(8));
            blk = 12; continue;
        }
        11 => {
            let r20 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r20);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f1ddc); }
            let r21 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r21);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 13; continue;
        }
        13 => {
            let r22 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r22);
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f1dfa); };
            c.w64(10, t23);
            if !c.st64(c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f1dfe); }
            let r24 = c.inc(c.r64(11), 64);
            c.w64(11, r24);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_l() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1e0e); };
            c.w64(10, t25);
            blk = 16; continue;
        }
        16 => {
            let r26 = c.dec(c.r64(3), 64);
            c.w64(3, r26);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f1e18); }
            if c.cc_g() { blk = 16; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 21; continue;
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1e23); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1e27); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f1e2b); };
            c.w64(3, t27);
            return c.call(0x700086ed74, 0x70007f1e37, 0x70007f1e32);
        }
        19 => {
            return RegfileExit::fault(0x70007f1e37);
        }
        20 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1e3b); }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f1e3c); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1e42); };
            c.logic(t30 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let r31 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r31);
            blk = 24; continue;
        }
        23 => {
            let Some(t32) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f1e58); };
            c.w32(1, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007f1e5b); };
            c.w64(1, t33);
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::jump(c.r64(1));
        }
        25 => {
            blk = 28; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1e67); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1e6b); }
            let r34 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f1e6f); }
            let r35 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1e72); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1e79); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1e7a); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f1e7b); };
            if !c.push64(t36) { return RegfileExit::fault(0x70007f1e7b); }
            c.w32(0, 0x2u64);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007f1e83); };
            c.w64(3, t37);
            return c.call(0x700086ed74, 0x70007f1e8f, 0x70007f1e8a);
        }
        27 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1e8f); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1e90); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f1e97); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1ea1); }
            blk = 7; continue;
        }
        28 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1ea7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1eab); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1ead); }
            c.w32(0, 0x1u64);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1da0)) else { return RegfileExit::fault(0x70007f1eb3); };
            c.w64(3, t38);
            return c.call(0x700086ed74, 0x70007f1ebf, 0x70007f1eba);
        }
        29 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1ec2); }
            return RegfileExit::fault(0x70007f1ec3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f1ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1ed4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1eda); };
            c.logic(t2 & 0x400u64, 16);
            if c.cc_ne() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f1ee6); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let Some(t5) = c.ld8(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f1eec); };
            c.logic(t5 & 0x60u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f1ef6); };
            c.w64(1, t6);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f1f00); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f1f06); };
            c.cmp(t9, 0xbau64, 16);
            if c.cc_ae() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1f12); };
            c.cmp(c.r32(1), t10, 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f1f1b); };
            c.cmp(c.r32(1), t11, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f1f24); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f1f2a); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f1f30); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            blk = 9; continue;
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f1f3b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f1f3f); }
            let r18 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r18);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f1f43); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f1f44); }
            c.w64(0, c.r64(1));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f1f48); }
            return c.call(0x70008a2054, 0x70007f1f4e, 0x70007f1f49);
        }
        8 => {
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007f1f4e); }
            c.w64(1, c.r64(0));
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f1f52); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f1f53); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f1f5d); }
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f1f5e); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(1)) { return RegfileExit::fault(0x70007f1f64); }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld16(c.r64(2).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f1f69); };
            c.w32(3, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007f1f6d); };
            c.w64(10, t22);
            let Some(t23) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007f1f74); };
            c.cmp(t23, 0x0u64, 8);
            if c.cc_ne() { blk = 30; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1f7e); };
            c.w64(2, t24);
            c.cmp(c.r32(3), 0xffffu64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 12; continue;
        }
        12 => {
            let r25 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r25);
            if c.cc_le() { blk = 25; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(10, c.r64(4));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f1f9d); };
            let r27 = c.sub(c.r64(10), t26, 64);
            c.w64(10, r27);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(3), 64);
            if c.cc_le() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(3).wrapping_mul(8));
            blk = 16; continue;
        }
        15 => {
            let r28 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r28);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f1fc5); }
            let r29 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r29);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 15; continue; }
            blk = 17; continue;
        }
        17 => {
            let r30 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r30);
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        18 => {
            let Some(t31) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f1fe3); };
            c.w64(10, t31);
            if !c.st64(c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f1fe7); }
            let r32 = c.inc(c.r64(11), 64);
            c.w64(11, r32);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_l() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f1ff7); };
            c.w64(10, t33);
            blk = 20; continue;
        }
        20 => {
            let r34 = c.dec(c.r64(3), 64);
            c.w64(3, r34);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f2001); }
            if c.cc_g() { blk = 20; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 25; continue;
        }
        22 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f200c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f2010); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f2014); };
            c.w64(3, t35);
            return c.call(0x700086ed74, 0x70007f2020, 0x70007f201b);
        }
        23 => {
            return RegfileExit::fault(0x70007f2020);
        }
        24 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f2024); }
            blk = 25; continue;
        }
        25 => {
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f2025); };
            c.w32(1, t36);
            let r37 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f202b); };
            c.logic(t38 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let r39 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r39);
            blk = 28; continue;
        }
        27 => {
            let Some(t40) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f2041); };
            c.w32(1, t40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007f2044); };
            c.w64(1, t41);
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::jump(c.r64(1));
        }
        29 => {
            blk = 32; continue;
        }
        30 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2050); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f2054); }
            let r42 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r42);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f2058); }
            let r43 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r43);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f205b); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2062); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2063); }
            let Some(t44) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f2064); };
            if !c.push64(t44) { return RegfileExit::fault(0x70007f2064); }
            c.w32(0, 0x2u64);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007f206c); };
            c.w64(3, t45);
            return c.call(0x700086ed74, 0x70007f2078, 0x70007f2073);
        }
        31 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2078); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f2079); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f2080); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f208a); }
            blk = 11; continue;
        }
        32 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2090); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f2094); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2096); }
            c.w32(0, 0x1u64);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1da0)) else { return RegfileExit::fault(0x70007f209c); };
            c.w64(3, t46);
            return c.call(0x700086ed74, 0x70007f20a8, 0x70007f20a3);
        }
        33 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f20ab); }
            return RegfileExit::jump(0x70007f20ac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f20b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f20b4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x70007f20ba); }
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f20bf); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f20c5); };
            c.w64(3, (t4 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.logic(c.r32(3) & c.r32(3), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            if let Some(sr) = c.shl(c.r64(3), 0x3u64, 64) { c.w64(3, sr); }
            c.w64(10, c.r64(4));
            let r5 = c.sub(c.r64(10), c.r64(3), 64);
            c.w64(10, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f20de); };
            c.cmp(c.r64(10), t6, 64);
            if c.cc_ae() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f20e4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f20e8); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f20ec); };
            c.w64(3, t7);
            return c.call(0x700086ed74, 0x70007f20f8, 0x70007f20f3);
        }
        3 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f20f8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f20fa); }
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f20fc); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f2102); };
            c.w64(3, (t10 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r11 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r11);
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f210c); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007f2114); }
            let r14 = c.dec(c.r32(3), 32);
            c.w32(3, r14);
            if c.cc_g() { blk = 4; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f211a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f211c); }
            blk = 6; continue;
        }
        6 => {
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f211e); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            return RegfileExit::jump(0x70007f2274);
        }
        7 => {
            return RegfileExit::fault(0x70007f2129);
        }
        8 => {
            return RegfileExit::fault(0x70007f212a);
        }
        9 => {
            return RegfileExit::fault(0x70007f212b);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f2134(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f213e); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2144); };
            c.w32(10, t2);
            c.w32(10, c.r64(10).wrapping_sub(0x80c));
            c.cmp(c.r32(10), 0xeu64, 32);
            if c.cc_be() { return RegfileExit::jump(0x70007f1b94); }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f215a); };
            c.cmp(t3, 0x80bu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f20b4); }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f2166); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f216c); };
            c.cmp(t5, 0xbcu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f49f4); }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(7)) { return RegfileExit::fault(0x70007f2178); }
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f217d); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2183); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xe3)) else { return RegfileExit::fault(0x70007f2189); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            return RegfileExit::jump(0x70007f1d14);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2197); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f219b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f219d); }
            c.w32(0, 0x1u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21c8)) else { return RegfileExit::fault(0x70007f21a3); };
            c.w64(3, t12);
            return c.call(0x700086ed74, 0x70007f21af, 0x70007f21aa);
        }
        7 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f21b2); }
            return RegfileExit::fault(0x70007f21b3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f21d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f21de); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f21e4); };
            c.w32(10, t2);
            c.w32(10, c.r64(10).wrapping_sub(0x80c));
            c.cmp(c.r32(10), 0xeu64, 32);
            if c.cc_be() { return RegfileExit::jump(0x70007f1d14); }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f21fa); };
            c.cmp(t3, 0x80bu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f20b4); }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f2206); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f220c); };
            c.cmp(t5, 0xbcu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f49f4); }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(7)) { return RegfileExit::fault(0x70007f2218); }
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f221d); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2223); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xe3)) else { return RegfileExit::fault(0x70007f2229); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            return RegfileExit::jump(0x70007f1d14);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2237); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f223b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f223d); }
            c.w32(0, 0x1u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21c8)) else { return RegfileExit::fault(0x70007f2243); };
            c.w64(3, t12);
            return c.call(0x700086ed74, 0x70007f224f, 0x70007f224a);
        }
        7 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f2252); }
            return RegfileExit::fault(0x70007f2253);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f2274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f227e); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2284); };
            c.w32(10, t2);
            c.w32(10, c.r64(10).wrapping_sub(0x80c));
            c.cmp(c.r32(10), 0xeu64, 32);
            if c.cc_be() { return RegfileExit::jump(0x70007f1ed4); }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f229a); };
            c.cmp(t3, 0x80bu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f20b4); }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f22a6); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f22ac); };
            c.cmp(t5, 0xbcu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f49f4); }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(7)) { return RegfileExit::fault(0x70007f22b8); }
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f22bd); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f22c3); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xe3)) else { return RegfileExit::fault(0x70007f22c9); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            return RegfileExit::jump(0x70007f1d14);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f22d7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f22db); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f22dd); }
            c.w32(0, 0x1u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21c8)) else { return RegfileExit::fault(0x70007f22e3); };
            c.w64(3, t12);
            return c.call(0x700086ed74, 0x70007f22ef, 0x70007f22ea);
        }
        7 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f22f2); }
            return RegfileExit::fault(0x70007f22f3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f2314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f2317); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f231b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f231f); };
            c.w32(8, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f2323); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(3));
            let r4 = c.shr(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r4);
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2347); };
            c.w32(11, t5);
            let r6 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r6);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2353); }
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f2358); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.w32(12, c.r32(11));
            let r9 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f2370); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f237d); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2394); };
            c.w32(12, t12);
            c.w64(15, c.r64(11));
            let r13 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r13);
            c.logic(r13, 64);
            c.w32(6, c.r32(12));
            let r14 = c.r32(6) & 0x3u64;
            c.w32(6, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f23c9); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f23d0); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f23d8); };
            c.w32(6, t19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f23e9);
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f23ea); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, c.r32(11));
            let r21 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r21);
            c.logic(r21, 32);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f23fc); };
            c.w64(12, t22);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f240c); };
            c.w32(11, t23);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r24 = c.r32(12) & 0x1u64;
            c.w32(12, r24);
            c.logic(r24, 32);
            let r25 = c.r32(15) & 0x3u64;
            c.w32(15, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            let r27 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2438); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f243d); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f2453); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f245a); };
            c.w32(12, t30);
            let r31 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r31);
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2461); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(12, c.r16(12));
            let r33 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r33);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f2484); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f248b); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2492); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2499); };
            c.w32(12, t40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f249e); };
            c.cmp(t41, c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(0));
            let r42 = c.r64(9) | 0x3u64;
            c.w64(9, r42);
            c.logic(r42, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x70007f24af); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r43 = c.r64(11) & c.r64(1);
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f24c3); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f24ce); }
            blk = 53; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f24e2); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f24e9); };
            c.w32(11, t47);
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f24ff); };
            c.w32(11, t49);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t50) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f250c); };
            c.w32(12, t50);
            let r51 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r51);
            let Some(t52) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2513); };
            c.w32(12, t52);
            let r53 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r53);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f252a); };
            c.w32(12, t54);
            let r55 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r55);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            let r56 = c.r64(12) | 0x3u64;
            c.w64(12, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x70007f253e); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r57 = c.r64(9) & c.r64(1);
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2552); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f255d); }
            blk = 53; continue;
        }
        32 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2569); };
            c.w32(11, t59);
            c.w32(12, c.r32(11));
            let r60 = c.r32(12) & 0x3u64;
            c.w32(12, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r61 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r61);
            c.logic(r61, 64);
            let r62 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r62);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f258e); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2593); }
            blk = 35; continue;
        }
        35 => {
            let Some(t63) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f259a); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f25a1); };
            c.w32(11, t65);
            let r66 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r66);
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f25a8); };
            c.w32(12, t67);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f25b8);
        }
        37 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f25b9); };
            c.cmp(t68, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, c.r64(0));
            blk = 43; continue;
        }
        39 => {
            let Some(t69) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f25c4); };
            c.w32(11, t69);
            c.w32(12, c.r32(11));
            let r70 = c.r32(12) & 0x3u64;
            c.w32(12, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r71 = c.r64(11) | 0x2u64;
            c.w64(11, r71);
            c.logic(r71, 64);
            let r72 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r72);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f25e9); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f25ee); }
            blk = 42; continue;
        }
        42 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f25f5); };
            c.w64(11, t73);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, c.r64(11));
            blk = 46; continue;
        }
        45 => {
            let Some(t74) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f260b); };
            c.w32(12, t74);
            let r75 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r75);
            blk = 46; continue;
        }
        46 => {
            let Some(t76) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2613); };
            c.w32(15, t76);
            let r77 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r77);
            let Some(t78) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f261b); };
            c.w32(15, t78);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 45; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(15, c.r16(15));
            let r79 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r79);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f2639); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2641); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2648); };
            c.w32(12, t84);
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2650); };
            c.w32(15, t86);
            let Some(t87) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2654); };
            c.cmp(t87, c.r32(15), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let r88 = c.r64(11) | 0x3u64;
            c.w64(11, r88);
            c.logic(r88, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x70007f265f); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r89 = c.r64(9) & c.r64(1);
            c.w64(9, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2673); };
            c.logic(t90 & 0x4u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f267a); }
            blk = 53; continue;
        }
        52 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f2683); };
            c.w64(8, t91);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70007f268a); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f268f); }
            blk = 53; continue;
        }
        53 => {
            c.w32(3, c.r8(3));
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            return RegfileExit::jump(0x70007f2134);
        }
        54 => {
            c.w32(11, c.r32(11));
            let r92 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r92);
            c.w64(9, 0xfffffffffffc0000u64);
            let r93 = c.r64(9) & c.r64(11);
            c.w64(9, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f26b7); };
            c.logic(t94 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f26c2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f26c3); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f26d0, 0x70007f26cb);
        }
        56 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f26d0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f26d2); }
            blk = 51; continue;
        }
        57 => {
            c.w32(12, c.r32(12));
            let r95 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r95);
            c.w64(9, 0xfffffffffffc0000u64);
            let r96 = c.r64(9) & c.r64(12);
            c.w64(9, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f26e5); };
            c.logic(t97 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f26f4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f26f5); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f2702, 0x70007f26fd);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2702); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2704); }
            blk = 31; continue;
        }
        60 => {
            c.w32(9, c.r32(9));
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            c.w64(11, 0xfffffffffffc0000u64);
            let r99 = c.r64(11) & c.r64(9);
            c.w64(11, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f271a); };
            c.logic(t100 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2729); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f272a); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f2737, 0x70007f2732);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2737); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2739); }
            blk = 23; continue;
        }
        63 => {
            return RegfileExit::jump(0x70007f2740);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f2754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f2757); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f275b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f275f); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f2763); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2780); };
            c.w32(11, t4);
            let r5 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r5);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f278c); }
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f2791); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            c.w32(12, c.r32(11));
            let r8 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f27a9); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f27b6); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f27cd); };
            c.w32(12, t11);
            c.w64(15, c.r64(11));
            let r12 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r12);
            c.logic(r12, 64);
            c.w32(6, c.r32(12));
            let r13 = c.r32(6) & 0x3u64;
            c.w32(6, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2802); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2809); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f2811); };
            c.w32(6, t18);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f2822);
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f2823); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, c.r32(11));
            let r20 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r20);
            c.logic(r20, 32);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f2835); };
            c.w64(12, t21);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2845); };
            c.w32(11, t22);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r23 = c.r32(12) & 0x1u64;
            c.w32(12, r23);
            c.logic(r23, 32);
            let r24 = c.r32(15) & 0x3u64;
            c.w32(15, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r25 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r25);
            c.logic(r25, 64);
            let r26 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r26);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2871); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2876); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f288d); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2894); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f289b); };
            c.w32(12, t31);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(12, c.r16(12));
            let r32 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r32);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f28be); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            let Some(t35) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f28c5); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f28cc); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f28d3); };
            c.w32(12, t39);
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f28d8); };
            c.cmp(t40, c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(0));
            let r41 = c.r64(9) | 0x3u64;
            c.w64(9, r41);
            c.logic(r41, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x70007f28e9); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r42 = c.r64(11) & c.r64(8);
            c.w64(11, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f28fd); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2908); }
            blk = 53; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f291d); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2924); };
            c.w32(11, t46);
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f293a); };
            c.w32(11, t48);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2947); };
            c.w32(12, t49);
            let r50 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r50);
            let Some(t51) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f294e); };
            c.w32(12, t51);
            let r52 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r52);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2965); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            let r55 = c.r64(12) | 0x3u64;
            c.w64(12, r55);
            c.logic(r55, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70007f2979); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r56 = c.r64(9) & c.r64(8);
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f298d); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2998); }
            blk = 53; continue;
        }
        32 => {
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f29a5); };
            c.w32(11, t58);
            c.w32(12, c.r32(11));
            let r59 = c.r32(12) & 0x3u64;
            c.w32(12, r59);
            c.logic(r59, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r60 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r60);
            c.logic(r60, 64);
            let r61 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r61);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f29ca); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f29cf); }
            blk = 35; continue;
        }
        35 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f29d7); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f29de); };
            c.w32(11, t64);
            let r65 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r65);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f29e5); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f29f5);
        }
        37 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f29f6); };
            c.cmp(t67, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, c.r64(0));
            blk = 43; continue;
        }
        39 => {
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2a01); };
            c.w32(11, t68);
            c.w32(12, c.r32(11));
            let r69 = c.r32(12) & 0x3u64;
            c.w32(12, r69);
            c.logic(r69, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r70 = c.r64(11) | 0x2u64;
            c.w64(11, r70);
            c.logic(r70, 64);
            let r71 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r71);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2a26); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2a2b); }
            blk = 42; continue;
        }
        42 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f2a33); };
            c.w64(11, t72);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, c.r64(11));
            blk = 46; continue;
        }
        45 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f2a49); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2a51); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2a59); };
            c.w32(15, t77);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 45; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(15, c.r16(15));
            let r78 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r78);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f2a77); };
            c.w32(12, t79);
            let r80 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2a7f); };
            c.w32(15, t81);
            let r82 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2a86); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2a8e); };
            c.w32(15, t85);
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2a92); };
            c.cmp(t86, c.r32(15), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let r87 = c.r64(11) | 0x3u64;
            c.w64(11, r87);
            c.logic(r87, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f2a9d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r88 = c.r64(9) & c.r64(8);
            c.w64(9, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2ab1); };
            c.logic(t89 & 0x4u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2ab8); }
            blk = 53; continue;
        }
        52 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f2ac2); };
            c.w64(1, t90);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f2ac9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2ace); }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            return RegfileExit::jump(0x70007f2134);
        }
        54 => {
            c.w32(11, c.r32(11));
            let r91 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r91);
            c.w64(9, 0xfffffffffffc0000u64);
            let r92 = c.r64(9) & c.r64(11);
            c.w64(9, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2af4); };
            c.logic(t93 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2aff); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2b00); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f2b0d, 0x70007f2b08);
        }
        56 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2b0d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2b0f); }
            blk = 51; continue;
        }
        57 => {
            c.w32(12, c.r32(12));
            let r94 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r94);
            c.w64(9, 0xfffffffffffc0000u64);
            let r95 = c.r64(9) & c.r64(12);
            c.w64(9, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2b22); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2b31); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2b32); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f2b3f, 0x70007f2b3a);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2b3f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2b41); }
            blk = 31; continue;
        }
        60 => {
            c.w32(9, c.r32(9));
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            c.w64(11, 0xfffffffffffc0000u64);
            let r98 = c.r64(11) & c.r64(9);
            c.w64(11, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2b57); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2b66); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2b67); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f2b74, 0x70007f2b6f);
        }
        62 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2b74); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2b76); }
            blk = 23; continue;
        }
        63 => {
            return RegfileExit::jump(0x70007f2b80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f2b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f2b97); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f2b9b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2b9f); };
            c.w32(8, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f2ba3); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(3));
            let r4 = c.shr(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r4);
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2bc7); };
            c.w32(11, t5);
            let r6 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r6);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2bd3); }
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f2bd8); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.w32(12, c.r32(11));
            let r9 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f2bf0); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f2bfd); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2c14); };
            c.w32(12, t12);
            c.w64(15, c.r64(11));
            let r13 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r13);
            c.logic(r13, 64);
            c.w32(6, c.r32(12));
            let r14 = c.r32(6) & 0x3u64;
            c.w32(6, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2c2b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2c2f); }
            blk = 27; continue;
        }
        7 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2c4c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2c50); }
            blk = 27; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2c57); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2c5e); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f2c66); };
            c.w32(6, t19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007f2c77);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2c78); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2c7c); }
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f2c7e); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, c.r64(5));
            c.w32(11, c.r32(11));
            c.w64(15, 0xfffffffffffffff8u64);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f2c96); };
            c.w64(12, t21);
            let r22 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2ca8); };
            c.w32(11, t23);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r24 = c.r32(12) & 0x1u64;
            c.w32(12, r24);
            c.logic(r24, 32);
            let r25 = c.r32(15) & 0x3u64;
            c.w32(15, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            let r27 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2cd4); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2cd9); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f2cef); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2cf6); };
            c.w32(12, t30);
            let r31 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r31);
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2cfd); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r33 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r33);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f2d20); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2d27); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2d2e); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2d35); };
            c.w32(12, t40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2d3a); };
            c.cmp(t41, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(0));
            let r42 = c.r64(9) | 0x3u64;
            c.w64(9, r42);
            c.logic(r42, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x70007f2d4b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r43 = c.r64(11) & c.r64(1);
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2d5f); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2d6a); }
            blk = 58; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2d76); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2d7a); }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2d84); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2d8b); };
            c.w32(11, t47);
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2da1); };
            c.w32(11, t49);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t50) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2dae); };
            c.w32(12, t50);
            let r51 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r51);
            let Some(t52) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2db5); };
            c.w32(12, t52);
            let r53 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r53);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2dcc); };
            c.w32(12, t54);
            let r55 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r55);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            let r56 = c.r64(12) | 0x3u64;
            c.w64(12, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x70007f2de0); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r57 = c.r64(9) & c.r64(1);
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2df4); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2dff); }
            blk = 58; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2e0b); };
            c.w32(11, t59);
            c.w32(12, c.r32(11));
            let r60 = c.r32(12) & 0x3u64;
            c.w32(12, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r61 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r61);
            c.logic(r61, 64);
            let r62 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r62);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f2e30); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2e35); }
            blk = 38; continue;
        }
        38 => {
            let Some(t63) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2e3c); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2e43); };
            c.w32(11, t65);
            let r66 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r66);
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f2e4a); };
            c.w32(12, t67);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007f2e5a);
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2e5b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2e5f); }
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f2e61); };
            c.cmp(t68, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(0));
            blk = 45; continue;
        }
        42 => {
            c.w64(11, c.r64(5));
            let Some(t69) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f2e6f); };
            c.w32(15, t69);
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t70) = c.ld64(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f2e7b); };
            c.w64(11, t70);
            c.w32(12, c.r32(15));
            let r71 = c.r32(12) & 0x3u64;
            c.w32(12, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r72 = c.r64(12) | 0x2u64;
            c.w64(12, r72);
            c.logic(r72, 64);
            let r73 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r73);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70007f2ea0); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2ea5); }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, c.r64(11));
            blk = 48; continue;
        }
        47 => {
            let Some(t74) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f2ebb); };
            c.w32(12, t74);
            let r75 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r75);
            blk = 48; continue;
        }
        48 => {
            let Some(t76) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2ec3); };
            c.w32(15, t76);
            let r77 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r77);
            let Some(t78) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f2ecb); };
            c.w32(15, t78);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r79 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r79);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f2ee9); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2ef1); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f2ef8); };
            c.w32(12, t84);
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2f00); };
            c.w32(15, t86);
            let Some(t87) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f2f04); };
            c.cmp(t87, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r88 = c.r64(11) | 0x3u64;
            c.w64(11, r88);
            c.logic(r88, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x70007f2f0f); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r89 = c.r64(9) & c.r64(1);
            c.w64(9, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2f23); };
            c.logic(t90 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2f2a); }
            blk = 58; continue;
        }
        54 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f2f33); };
            c.w64(8, t91);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70007f2f3a); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f2f3f); }
            blk = 58; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2f48); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2f4c); }
            blk = 58; continue;
        }
        56 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2f50); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2f54); }
            blk = 58; continue;
        }
        57 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f2f58); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f2f5c); }
            blk = 58; continue;
        }
        58 => {
            c.w32(3, c.r8(3));
            let Some(t92) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f2f61); };
            c.w64(5, t92);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            let r93 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r93);
            return RegfileExit::jump(0x70007f21d4);
        }
        59 => {
            c.w32(11, c.r32(11));
            let r94 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r94);
            c.w64(9, 0xfffffffffffc0000u64);
            let r95 = c.r64(9) & c.r64(11);
            c.w64(9, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2f87); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2f92); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2f93); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f2fa0, 0x70007f2f9b);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2fa0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2fa2); }
            blk = 53; continue;
        }
        62 => {
            c.w32(12, c.r32(12));
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            c.w64(9, 0xfffffffffffc0000u64);
            let r98 = c.r64(9) & c.r64(12);
            c.w64(9, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2fb5); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2fc4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2fc5); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f2fd2, 0x70007f2fcd);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f2fd2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f2fd4); }
            blk = 34; continue;
        }
        65 => {
            c.w32(9, c.r32(9));
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            c.w64(11, 0xfffffffffffc0000u64);
            let r101 = c.r64(11) & c.r64(9);
            c.w64(11, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f2fea); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f2ff9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f2ffa); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f3007, 0x70007f3002);
        }
        67 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3007); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3009); }
            blk = 25; continue;
        }
        68 => {
            return RegfileExit::jump(0x70007f3010);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f3014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f3017); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f301b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f301f); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f3023); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3040); };
            c.w32(11, t4);
            let r5 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r5);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f304c); }
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f3051); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            c.w32(12, c.r32(11));
            let r8 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f3069); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f3076); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f308d); };
            c.w32(12, t11);
            c.w64(15, c.r64(11));
            let r12 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r12);
            c.logic(r12, 64);
            c.w32(6, c.r32(12));
            let r13 = c.r32(6) & 0x3u64;
            c.w32(6, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f30a4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f30a8); }
            blk = 27; continue;
        }
        7 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f30c5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f30c9); }
            blk = 27; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f30d0); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f30d7); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f30df); };
            c.w32(6, t18);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007f30f0);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f30f1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f30f5); }
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f30f7); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, c.r64(5));
            c.w32(11, c.r32(11));
            c.w64(15, 0xfffffffffffffff8u64);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f310f); };
            c.w64(12, t20);
            let r21 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3121); };
            c.w32(11, t22);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r23 = c.r32(12) & 0x1u64;
            c.w32(12, r23);
            c.logic(r23, 32);
            let r24 = c.r32(15) & 0x3u64;
            c.w32(15, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r25 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r25);
            c.logic(r25, 64);
            let r26 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r26);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f314d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3152); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        19 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f3169); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3170); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3177); };
            c.w32(12, t31);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r32 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r32);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f319a); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            let Some(t35) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f31a1); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f31a8); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f31af); };
            c.w32(12, t39);
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f31b4); };
            c.cmp(t40, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(0));
            let r41 = c.r64(9) | 0x3u64;
            c.w64(9, r41);
            c.logic(r41, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x70007f31c5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r42 = c.r64(11) & c.r64(8);
            c.w64(11, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f31d9); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f31e4); }
            blk = 58; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f31f1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f31f5); }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f31ff); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3206); };
            c.w32(11, t46);
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f321c); };
            c.w32(11, t48);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t49) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3229); };
            c.w32(12, t49);
            let r50 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r50);
            let Some(t51) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3230); };
            c.w32(12, t51);
            let r52 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r52);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3247); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            let r55 = c.r64(12) | 0x3u64;
            c.w64(12, r55);
            c.logic(r55, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70007f325b); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r56 = c.r64(9) & c.r64(8);
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f326f); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f327a); }
            blk = 58; continue;
        }
        35 => {
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3287); };
            c.w32(11, t58);
            c.w32(12, c.r32(11));
            let r59 = c.r32(12) & 0x3u64;
            c.w32(12, r59);
            c.logic(r59, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r60 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r60);
            c.logic(r60, 64);
            let r61 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r61);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f32ac); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f32b1); }
            blk = 38; continue;
        }
        38 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f32b9); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f32c0); };
            c.w32(11, t64);
            let r65 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r65);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f32c7); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007f32d7);
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f32d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f32dc); }
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f32de); };
            c.cmp(t67, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(0));
            blk = 45; continue;
        }
        42 => {
            c.w64(11, c.r64(5));
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f32ec); };
            c.w32(15, t68);
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t69) = c.ld64(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f32f8); };
            c.w64(11, t69);
            c.w32(12, c.r32(15));
            let r70 = c.r32(12) & 0x3u64;
            c.w32(12, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r71 = c.r64(12) | 0x2u64;
            c.w64(12, r71);
            c.logic(r71, 64);
            let r72 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r72);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70007f331d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3322); }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, c.r64(11));
            blk = 48; continue;
        }
        47 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f3339); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            blk = 48; continue;
        }
        48 => {
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3341); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3349); };
            c.w32(15, t77);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r78 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r78);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f3367); };
            c.w32(12, t79);
            let r80 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f336f); };
            c.w32(15, t81);
            let r82 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3376); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f337e); };
            c.w32(15, t85);
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3382); };
            c.cmp(t86, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r87 = c.r64(11) | 0x3u64;
            c.w64(11, r87);
            c.logic(r87, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f338d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r88 = c.r64(9) & c.r64(8);
            c.w64(9, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f33a1); };
            c.logic(t89 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f33a8); }
            blk = 58; continue;
        }
        54 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f33b2); };
            c.w64(1, t90);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f33b9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f33be); }
            blk = 58; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f33c8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f33cc); }
            blk = 58; continue;
        }
        56 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f33d0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f33d4); }
            blk = 58; continue;
        }
        57 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f33d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f33dc); }
            blk = 58; continue;
        }
        58 => {
            let Some(t91) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f33de); };
            c.w64(5, t91);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            let r92 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r92);
            return RegfileExit::jump(0x70007f21d4);
        }
        59 => {
            c.w32(11, c.r32(11));
            let r93 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(11);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3404); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f340f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3410); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f341d, 0x70007f3418);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f341d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f341f); }
            blk = 53; continue;
        }
        62 => {
            c.w32(12, c.r32(12));
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            c.w64(9, 0xfffffffffffc0000u64);
            let r97 = c.r64(9) & c.r64(12);
            c.w64(9, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3432); };
            c.logic(t98 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3441); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3442); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f344f, 0x70007f344a);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f344f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3451); }
            blk = 34; continue;
        }
        65 => {
            c.w32(9, c.r32(9));
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            c.w64(11, 0xfffffffffffc0000u64);
            let r100 = c.r64(11) & c.r64(9);
            c.w64(11, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3467); };
            c.logic(t101 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3476); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3477); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f3484, 0x70007f347f);
        }
        67 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3484); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3486); }
            blk = 25; continue;
        }
        68 => {
            return RegfileExit::jump(0x70007f3490);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f3494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f3497); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f349b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f349f); };
            c.w32(8, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f34a3); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(3));
            let r4 = c.shr(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r4);
            c.w64(9, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f34c7); };
            c.w32(11, t5);
            let r6 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r6);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f34d3); }
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f34d8); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            c.w32(12, c.r32(11));
            let r9 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f34f0); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f34fd); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3514); };
            c.w32(12, t12);
            c.w64(15, c.r64(11));
            let r13 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r13);
            c.logic(r13, 64);
            c.w32(6, c.r32(12));
            let r14 = c.r32(6) & 0x3u64;
            c.w32(6, r14);
            c.logic(r14, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f352b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f352f); }
            blk = 27; continue;
        }
        7 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f354c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3550); }
            blk = 27; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3557); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f355e); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f3566); };
            c.w32(6, t19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007f3577);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3578); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f357c); }
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f357e); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, c.r64(5));
            c.w32(11, c.r32(11));
            c.w64(15, 0xfffffffffffffff8u64);
            let Some(t21) = c.ld64(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f3596); };
            c.w64(12, t21);
            let r22 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f35a8); };
            c.w32(11, t23);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r24 = c.r32(12) & 0x1u64;
            c.w32(12, r24);
            c.logic(r24, 32);
            let r25 = c.r32(15) & 0x3u64;
            c.w32(15, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r26 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r26);
            c.logic(r26, 64);
            let r27 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r27);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f35d4); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f35d9); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f35ef); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f35f6); };
            c.w32(12, t30);
            let r31 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r31);
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f35fd); };
            c.w32(12, t32);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r33 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r33);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f3620); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3627); };
            c.w32(12, t36);
            let r37 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f362e); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3635); };
            c.w32(12, t40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f363a); };
            c.cmp(t41, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(0));
            let r42 = c.r64(9) | 0x3u64;
            c.w64(9, r42);
            c.logic(r42, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x70007f364b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r43 = c.r64(11) & c.r64(1);
            c.w64(11, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f365f); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f366a); }
            blk = 58; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3676); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f367a); }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3684); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            let Some(t47) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f368b); };
            c.w32(11, t47);
            let r48 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r48);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f36a1); };
            c.w32(11, t49);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t50) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f36ae); };
            c.w32(12, t50);
            let r51 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r51);
            let Some(t52) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f36b5); };
            c.w32(12, t52);
            let r53 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r53);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f36cc); };
            c.w32(12, t54);
            let r55 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r55);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            let r56 = c.r64(12) | 0x3u64;
            c.w64(12, r56);
            c.logic(r56, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x70007f36e0); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r57 = c.r64(9) & c.r64(1);
            c.w64(9, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f36f4); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f36ff); }
            blk = 58; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f370b); };
            c.w32(11, t59);
            c.w32(12, c.r32(11));
            let r60 = c.r32(12) & 0x3u64;
            c.w32(12, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r61 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r61);
            c.logic(r61, 64);
            let r62 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r62);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f3730); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3735); }
            blk = 38; continue;
        }
        38 => {
            let Some(t63) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f373c); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3743); };
            c.w32(11, t65);
            let r66 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r66);
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f374a); };
            c.w32(12, t67);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007f375a);
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f375b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f375f); }
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f3761); };
            c.cmp(t68, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(0));
            blk = 45; continue;
        }
        42 => {
            c.w64(11, c.r64(5));
            let Some(t69) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f376f); };
            c.w32(15, t69);
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t70) = c.ld64(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f377b); };
            c.w64(11, t70);
            c.w32(12, c.r32(15));
            let r71 = c.r32(12) & 0x3u64;
            c.w32(12, r71);
            c.logic(r71, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r72 = c.r64(12) | 0x2u64;
            c.w64(12, r72);
            c.logic(r72, 64);
            let r73 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r73);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70007f37a0); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f37a5); }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, c.r64(11));
            blk = 48; continue;
        }
        47 => {
            let Some(t74) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f37bb); };
            c.w32(12, t74);
            let r75 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r75);
            blk = 48; continue;
        }
        48 => {
            let Some(t76) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f37c3); };
            c.w32(15, t76);
            let r77 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r77);
            let Some(t78) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f37cb); };
            c.w32(15, t78);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r79 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r79);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f37e9); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f37f1); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            let Some(t84) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f37f8); };
            c.w32(12, t84);
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            let Some(t86) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3800); };
            c.w32(15, t86);
            let Some(t87) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3804); };
            c.cmp(t87, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r88 = c.r64(11) | 0x3u64;
            c.w64(11, r88);
            c.logic(r88, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x70007f380f); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r89 = c.r64(9) & c.r64(1);
            c.w64(9, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3823); };
            c.logic(t90 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f382a); }
            blk = 58; continue;
        }
        54 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f3833); };
            c.w64(8, t91);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70007f383a); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f383f); }
            blk = 58; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3848); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f384c); }
            blk = 58; continue;
        }
        56 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3850); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3854); }
            blk = 58; continue;
        }
        57 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3858); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f385c); }
            blk = 58; continue;
        }
        58 => {
            c.w32(3, c.r8(3));
            let Some(t92) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f3861); };
            c.w64(5, t92);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            let r93 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r93);
            return RegfileExit::jump(0x70007f2274);
        }
        59 => {
            c.w32(11, c.r32(11));
            let r94 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r94);
            c.w64(9, 0xfffffffffffc0000u64);
            let r95 = c.r64(9) & c.r64(11);
            c.w64(9, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3887); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3892); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3893); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f38a0, 0x70007f389b);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f38a0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f38a2); }
            blk = 53; continue;
        }
        62 => {
            c.w32(12, c.r32(12));
            let r97 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r97);
            c.w64(9, 0xfffffffffffc0000u64);
            let r98 = c.r64(9) & c.r64(12);
            c.w64(9, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f38b5); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f38c4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f38c5); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f38d2, 0x70007f38cd);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f38d2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f38d4); }
            blk = 34; continue;
        }
        65 => {
            c.w32(9, c.r32(9));
            let r100 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r100);
            c.w64(11, 0xfffffffffffc0000u64);
            let r101 = c.r64(11) & c.r64(9);
            c.w64(11, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f38ea); };
            c.logic(t102 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f38f9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f38fa); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f3907, 0x70007f3902);
        }
        67 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3907); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3909); }
            blk = 25; continue;
        }
        68 => {
            return RegfileExit::jump(0x70007f3910);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f3914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f3917); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f391b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f391f); };
            c.w32(9, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f3923); };
            c.cmp(t3, c.r32(9), 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3940); };
            c.w32(11, t4);
            let r5 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r5);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f394c); }
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f3951); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            c.w32(12, c.r32(11));
            let r8 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f3969); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f3976); };
            c.cmp(t10, c.r32(11), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f398d); };
            c.w32(12, t11);
            c.w64(15, c.r64(11));
            let r12 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r12);
            c.logic(r12, 64);
            c.w32(6, c.r32(12));
            let r13 = c.r32(6) & 0x3u64;
            c.w32(6, r13);
            c.logic(r13, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f39a4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f39a8); }
            blk = 27; continue;
        }
        7 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f39c5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f39c9); }
            blk = 27; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f39d0); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            let Some(t16) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f39d7); };
            c.w32(12, t16);
            let r17 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r17);
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f39df); };
            c.w32(6, t18);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0xfu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70007f39f0);
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f39f1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f39f5); }
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f39f7); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, c.r64(5));
            c.w32(11, c.r32(11));
            c.w64(15, 0xfffffffffffffff8u64);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f3a0f); };
            c.w64(12, t20);
            let r21 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3a21); };
            c.w32(11, t22);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r23 = c.r32(12) & 0x1u64;
            c.w32(12, r23);
            c.logic(r23, 32);
            let r24 = c.r32(15) & 0x3u64;
            c.w32(15, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r25 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r25);
            c.logic(r25, 64);
            let r26 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r26);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f3a4d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3a52); }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(0));
            blk = 20; continue;
        }
        19 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f3a69); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3a70); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3a77); };
            c.w32(12, t31);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r32 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r32);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f3a9a); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            let Some(t35) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3aa1); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3aa8); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3aaf); };
            c.w32(12, t39);
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3ab4); };
            c.cmp(t40, c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(0));
            let r41 = c.r64(9) | 0x3u64;
            c.w64(9, r41);
            c.logic(r41, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x70007f3ac5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r42 = c.r64(11) & c.r64(8);
            c.w64(11, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3ad9); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3ae4); }
            blk = 58; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3af1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3af5); }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3aff); };
            c.w32(11, t44);
            let r45 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r45);
            let Some(t46) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3b06); };
            c.w32(11, t46);
            let r47 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r47);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3b1c); };
            c.w32(11, t48);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t49) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3b29); };
            c.w32(12, t49);
            let r50 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r50);
            let Some(t51) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3b30); };
            c.w32(12, t51);
            let r52 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r52);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3b47); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            let r55 = c.r64(12) | 0x3u64;
            c.w64(12, r55);
            c.logic(r55, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70007f3b5b); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r56 = c.r64(9) & c.r64(8);
            c.w64(9, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3b6f); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3b7a); }
            blk = 58; continue;
        }
        35 => {
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3b87); };
            c.w32(11, t58);
            c.w32(12, c.r32(11));
            let r59 = c.r32(12) & 0x3u64;
            c.w32(12, r59);
            c.logic(r59, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r60 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r60);
            c.logic(r60, 64);
            let r61 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r61);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f3bac); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3bb1); }
            blk = 38; continue;
        }
        38 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3bb9); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3bc0); };
            c.w32(11, t64);
            let r65 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r65);
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f3bc7); };
            c.w32(12, t66);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007f3bd7);
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3bd8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3bdc); }
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f3bde); };
            c.cmp(t67, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, c.r64(0));
            blk = 45; continue;
        }
        42 => {
            c.w64(11, c.r64(5));
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3bec); };
            c.w32(15, t68);
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t69) = c.ld64(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f3bf8); };
            c.w64(11, t69);
            c.w32(12, c.r32(15));
            let r70 = c.r32(12) & 0x3u64;
            c.w32(12, r70);
            c.logic(r70, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r71 = c.r64(12) | 0x2u64;
            c.w64(12, r71);
            c.logic(r71, 64);
            let r72 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r72);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70007f3c1d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3c22); }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(12, c.r64(11));
            blk = 48; continue;
        }
        47 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f3c39); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            blk = 48; continue;
        }
        48 => {
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3c41); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3c49); };
            c.w32(15, t77);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 47; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(15, c.r16(15));
            let r78 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r78);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f3c67); };
            c.w32(12, t79);
            let r80 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3c6f); };
            c.w32(15, t81);
            let r82 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3c76); };
            c.w32(12, t83);
            let r84 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r84);
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3c7e); };
            c.w32(15, t85);
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3c82); };
            c.cmp(t86, c.r32(15), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            let r87 = c.r64(11) | 0x3u64;
            c.w64(11, r87);
            c.logic(r87, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f3c8d); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r88 = c.r64(9) & c.r64(8);
            c.w64(9, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3ca1); };
            c.logic(t89 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3ca8); }
            blk = 58; continue;
        }
        54 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f3cb2); };
            c.w64(1, t90);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f3cb9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3cbe); }
            blk = 58; continue;
        }
        55 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3cc8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3ccc); }
            blk = 58; continue;
        }
        56 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3cd0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3cd4); }
            blk = 58; continue;
        }
        57 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f3cd8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f3cdc); }
            blk = 58; continue;
        }
        58 => {
            let Some(t91) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f3cde); };
            c.w64(5, t91);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(6, c.r64(2));
            let r92 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r92);
            return RegfileExit::jump(0x70007f2274);
        }
        59 => {
            c.w32(11, c.r32(11));
            let r93 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(11);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3d04); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3d0f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3d10); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f3d1d, 0x70007f3d18);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3d1d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3d1f); }
            blk = 53; continue;
        }
        62 => {
            c.w32(12, c.r32(12));
            let r96 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r96);
            c.w64(9, 0xfffffffffffc0000u64);
            let r97 = c.r64(9) & c.r64(12);
            c.w64(9, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3d32); };
            c.logic(t98 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3d41); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3d42); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f3d4f, 0x70007f3d4a);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3d4f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3d51); }
            blk = 34; continue;
        }
        65 => {
            c.w32(9, c.r32(9));
            let r99 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r99);
            c.w64(11, 0xfffffffffffc0000u64);
            let r100 = c.r64(11) & c.r64(9);
            c.w64(11, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3d67); };
            c.logic(t101 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f3d76); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f3d77); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f1594, 0x70007f3d84, 0x70007f3d7f);
        }
        67 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f3d84); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f3d86); }
            blk = 25; continue;
        }
        68 => {
            return RegfileExit::jump(0x70007f3d90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f3d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3d94); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f3d98); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3db5); };
            c.w32(9, t2);
            let r3 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f3dc1); }
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f3dc6); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            c.w32(11, c.r32(9));
            let r6 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f3dde); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f3deb); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3e02); };
            c.w32(11, t9);
            c.w64(12, c.r64(9));
            let r10 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r10);
            c.logic(r10, 64);
            c.w32(15, c.r32(11));
            let r11 = c.r32(15) & 0x3u64;
            c.w32(15, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3e39); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3e40); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f3e47); };
            c.w32(15, t16);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f3e57);
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f3e58); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(9, c.r32(9));
            let r18 = c.r32(9) & 0xfffffffffffffffdu64;
            c.w32(9, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3e72); };
            c.w32(9, t19);
            c.w32(11, c.r32(0));
            c.w32(12, c.r32(9));
            let r20 = c.r32(11) & 0x1u64;
            c.w32(11, r20);
            c.logic(r20, 32);
            let r21 = c.r32(12) & 0x3u64;
            c.w32(12, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r22 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r22);
            c.logic(r22, 64);
            let r23 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r23);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f3e9e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3ea3); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f3eb9); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3ec0); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3ec7); };
            c.w32(11, t28);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(11, c.r16(11));
            let r29 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r29);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f3ee9); };
            c.w32(9, t30);
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3ef0); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3ef7); };
            c.w32(9, t34);
            let r35 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3efe); };
            c.w32(11, t36);
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3f02); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(0));
            let r38 = c.r64(8) | 0x3u64;
            c.w64(8, r38);
            c.logic(r38, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70007f3f13); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r39 = c.r64(9) & c.r64(2);
            c.w64(9, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3f27); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3f32); }
            blk = 52; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3f46); };
            c.w32(9, t41);
            let r42 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r42);
            let Some(t43) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3f4d); };
            c.w32(9, t43);
            let r44 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r44);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3f63); };
            c.w32(9, t45);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3f70); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f3f78); };
            c.w32(11, t48);
            let r49 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r49);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f3f8e); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let r52 = c.r64(11) | 0x3u64;
            c.w64(11, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f3fa3); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(2);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f3fb7); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3fc2); }
            blk = 52; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f3fce); };
            c.w32(9, t55);
            c.w32(11, c.r32(9));
            let r56 = c.r32(11) & 0x3u64;
            c.w32(11, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r57 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f3ff3); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f3ff8); }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f3fff); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4006); };
            c.w32(9, t61);
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f400d); };
            c.w32(11, t63);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f401d);
        }
        37 => {
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f401e); };
            c.cmp(t64, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(0));
            blk = 42; continue;
        }
        39 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4029); };
            c.w32(9, t65);
            c.w32(11, c.r32(9));
            let r66 = c.r32(11) & 0x3u64;
            c.w32(11, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r67 = c.r64(9) | 0x2u64;
            c.w64(9, r67);
            c.logic(r67, 64);
            let r68 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r68);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f404e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4053); }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(7));
            blk = 45; continue;
        }
        44 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4069); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4070); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            let Some(t73) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4077); };
            c.w32(11, t73);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(11, c.r16(11));
            let r74 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r74);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f4095); };
            c.w32(9, t75);
            let r76 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r76);
            let Some(t77) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f409c); };
            c.w32(11, t77);
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f40a3); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f40aa); };
            c.w32(11, t81);
            let Some(t82) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f40ae); };
            c.cmp(t82, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            let r83 = c.r64(7) | 0x3u64;
            c.w64(7, r83);
            c.logic(r83, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x70007f40b8); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r84 = c.r64(8) & c.r64(2);
            c.w64(8, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f40cb); };
            c.logic(t85 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f40d2); }
            blk = 52; continue;
        }
        51 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f40db); };
            c.w64(1, t86);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f40e2); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f40e7); }
            blk = 52; continue;
        }
        52 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            return RegfileExit::jump(0x70007f2134);
        }
        53 => {
            c.w32(7, c.r32(7));
            let r87 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r87);
            c.w64(8, 0xfffffffffffc0000u64);
            let r88 = c.r64(8) & c.r64(7);
            c.w64(8, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4108); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4113); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4114); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4121, 0x70007f411c);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4121); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4123); }
            blk = 50; continue;
        }
        56 => {
            c.w32(11, c.r32(11));
            let r90 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r90);
            c.w64(8, 0xfffffffffffc0000u64);
            let r91 = c.r64(8) & c.r64(11);
            c.w64(8, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4136); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4145); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4146); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4153, 0x70007f414e);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4153); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4155); }
            blk = 31; continue;
        }
        59 => {
            c.w32(8, c.r32(8));
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(8);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f416b); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f417a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f417b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f4188, 0x70007f4183);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4188); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f418a); }
            blk = 23; continue;
        }
        62 => {
            return RegfileExit::jump(0x70007f4194);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f41b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f41b4); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f41b8); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f41d5); };
            c.w32(9, t2);
            let r3 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f41e1); }
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f41e6); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            c.w32(11, c.r32(9));
            let r6 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f41fe); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f420b); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4222); };
            c.w32(11, t9);
            c.w64(12, c.r64(9));
            let r10 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r10);
            c.logic(r10, 64);
            c.w32(15, c.r32(11));
            let r11 = c.r32(15) & 0x3u64;
            c.w32(15, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4259); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4260); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f4267); };
            c.w32(15, t16);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f4277);
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f4278); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(9, c.r32(9));
            let r18 = c.r32(9) & 0xfffffffffffffffdu64;
            c.w32(9, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4292); };
            c.w32(9, t19);
            c.w32(11, c.r32(0));
            c.w32(12, c.r32(9));
            let r20 = c.r32(11) & 0x1u64;
            c.w32(11, r20);
            c.logic(r20, 32);
            let r21 = c.r32(12) & 0x3u64;
            c.w32(12, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r22 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r22);
            c.logic(r22, 64);
            let r23 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r23);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f42be); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f42c3); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f42d9); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f42e0); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f42e7); };
            c.w32(11, t28);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(11, c.r16(11));
            let r29 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r29);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f4309); };
            c.w32(9, t30);
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4310); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4317); };
            c.w32(9, t34);
            let r35 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f431e); };
            c.w32(11, t36);
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4322); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(0));
            let r38 = c.r64(8) | 0x3u64;
            c.w64(8, r38);
            c.logic(r38, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70007f4333); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r39 = c.r64(9) & c.r64(2);
            c.w64(9, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4347); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4352); }
            blk = 52; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4366); };
            c.w32(9, t41);
            let r42 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r42);
            let Some(t43) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f436d); };
            c.w32(9, t43);
            let r44 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r44);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4383); };
            c.w32(9, t45);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4390); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4398); };
            c.w32(11, t48);
            let r49 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r49);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f43ae); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let r52 = c.r64(11) | 0x3u64;
            c.w64(11, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f43c3); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(2);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f43d7); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f43e2); }
            blk = 52; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f43ee); };
            c.w32(9, t55);
            c.w32(11, c.r32(9));
            let r56 = c.r32(11) & 0x3u64;
            c.w32(11, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r57 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f4413); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4418); }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f441f); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4426); };
            c.w32(9, t61);
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f442d); };
            c.w32(11, t63);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f443d);
        }
        37 => {
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f443e); };
            c.cmp(t64, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(0));
            blk = 42; continue;
        }
        39 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4449); };
            c.w32(9, t65);
            c.w32(11, c.r32(9));
            let r66 = c.r32(11) & 0x3u64;
            c.w32(11, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r67 = c.r64(9) | 0x2u64;
            c.w64(9, r67);
            c.logic(r67, 64);
            let r68 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r68);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f446e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4473); }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(7));
            blk = 45; continue;
        }
        44 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4489); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4490); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            let Some(t73) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4497); };
            c.w32(11, t73);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(11, c.r16(11));
            let r74 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r74);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f44b5); };
            c.w32(9, t75);
            let r76 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r76);
            let Some(t77) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f44bc); };
            c.w32(11, t77);
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f44c3); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f44ca); };
            c.w32(11, t81);
            let Some(t82) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f44ce); };
            c.cmp(t82, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            let r83 = c.r64(7) | 0x3u64;
            c.w64(7, r83);
            c.logic(r83, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x70007f44d8); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r84 = c.r64(8) & c.r64(2);
            c.w64(8, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f44eb); };
            c.logic(t85 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f44f2); }
            blk = 52; continue;
        }
        51 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f44fb); };
            c.w64(1, t86);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f4502); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4507); }
            blk = 52; continue;
        }
        52 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            return RegfileExit::jump(0x70007f21d4);
        }
        53 => {
            c.w32(7, c.r32(7));
            let r87 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r87);
            c.w64(8, 0xfffffffffffc0000u64);
            let r88 = c.r64(8) & c.r64(7);
            c.w64(8, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4528); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4533); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4534); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4541, 0x70007f453c);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4541); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4543); }
            blk = 50; continue;
        }
        56 => {
            c.w32(11, c.r32(11));
            let r90 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r90);
            c.w64(8, 0xfffffffffffc0000u64);
            let r91 = c.r64(8) & c.r64(11);
            c.w64(8, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4556); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4565); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4566); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4573, 0x70007f456e);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4573); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4575); }
            blk = 31; continue;
        }
        59 => {
            c.w32(8, c.r32(8));
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(8);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f458b); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f459a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f459b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f45a8, 0x70007f45a3);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f45a8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f45aa); }
            blk = 23; continue;
        }
        62 => {
            return RegfileExit::jump(0x70007f45b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f45d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f45d4); };
            c.w32(8, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f45d8); };
            c.cmp(t1, c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f45f5); };
            c.w32(9, t2);
            let r3 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f4601); }
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f4606); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            c.w32(11, c.r32(9));
            let r6 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f461e); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f462b); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4642); };
            c.w32(11, t9);
            c.w64(12, c.r64(9));
            let r10 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r10);
            c.logic(r10, 64);
            c.w32(15, c.r32(11));
            let r11 = c.r32(15) & 0x3u64;
            c.w32(15, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4679); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4680); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f4687); };
            c.w32(15, t16);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f4697);
        }
        10 => {
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f4698); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(9, c.r32(9));
            let r18 = c.r32(9) & 0xfffffffffffffffdu64;
            c.w32(9, r18);
            c.logic(r18, 32);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f46b2); };
            c.w32(9, t19);
            c.w32(11, c.r32(0));
            c.w32(12, c.r32(9));
            let r20 = c.r32(11) & 0x1u64;
            c.w32(11, r20);
            c.logic(r20, 32);
            let r21 = c.r32(12) & 0x3u64;
            c.w32(12, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r22 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r22);
            c.logic(r22, 64);
            let r23 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r23);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f46de); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f46e3); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(9, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f46f9); };
            c.w32(9, t24);
            let r25 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4700); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            let Some(t28) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4707); };
            c.w32(11, t28);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(11, c.r16(11));
            let r29 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r29);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f4729); };
            c.w32(9, t30);
            let r31 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r31);
            let Some(t32) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4730); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4737); };
            c.w32(9, t34);
            let r35 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f473e); };
            c.w32(11, t36);
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4742); };
            c.cmp(t37, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(0));
            let r38 = c.r64(8) | 0x3u64;
            c.w64(8, r38);
            c.logic(r38, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70007f4753); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r39 = c.r64(9) & c.r64(2);
            c.w64(9, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4767); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4772); }
            blk = 52; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4786); };
            c.w32(9, t41);
            let r42 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r42);
            let Some(t43) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f478d); };
            c.w32(9, t43);
            let r44 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r44);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f47a3); };
            c.w32(9, t45);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f47b0); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f47b8); };
            c.w32(11, t48);
            let r49 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r49);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f47ce); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let r52 = c.r64(11) | 0x3u64;
            c.w64(11, r52);
            c.logic(r52, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70007f47e3); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r53 = c.r64(8) & c.r64(2);
            c.w64(8, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f47f7); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4802); }
            blk = 52; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f480e); };
            c.w32(9, t55);
            c.w32(11, c.r32(9));
            let r56 = c.r32(11) & 0x3u64;
            c.w32(11, r56);
            c.logic(r56, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r57 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let r58 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r58);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f4833); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4838); }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f483f); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4846); };
            c.w32(9, t61);
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f484d); };
            c.w32(11, t63);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f485d);
        }
        37 => {
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f485e); };
            c.cmp(t64, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(0));
            blk = 42; continue;
        }
        39 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f4869); };
            c.w32(9, t65);
            c.w32(11, c.r32(9));
            let r66 = c.r32(11) & 0x3u64;
            c.w32(11, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r67 = c.r64(9) | 0x2u64;
            c.w64(9, r67);
            c.logic(r67, 64);
            let r68 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r68);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f488e); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4893); }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(7));
            blk = 45; continue;
        }
        44 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f48a9); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f48b0); };
            c.w32(11, t71);
            let r72 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r72);
            let Some(t73) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f48b7); };
            c.w32(11, t73);
            c.cmp(c.r16(11), 0x80bu64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(11, c.r16(11));
            let r74 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r74);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f48d5); };
            c.w32(9, t75);
            let r76 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r76);
            let Some(t77) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f48dc); };
            c.w32(11, t77);
            let r78 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r78);
            let Some(t79) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f48e3); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f48ea); };
            c.w32(11, t81);
            let Some(t82) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f48ee); };
            c.cmp(t82, c.r32(11), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            let r83 = c.r64(7) | 0x3u64;
            c.w64(7, r83);
            c.logic(r83, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x70007f48f8); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r84 = c.r64(8) & c.r64(2);
            c.w64(8, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f490b); };
            c.logic(t85 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4912); }
            blk = 52; continue;
        }
        51 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f491b); };
            c.w64(1, t86);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x70007f4922); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f4927); }
            blk = 52; continue;
        }
        52 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            return RegfileExit::jump(0x70007f2274);
        }
        53 => {
            c.w32(7, c.r32(7));
            let r87 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r87);
            c.w64(8, 0xfffffffffffc0000u64);
            let r88 = c.r64(8) & c.r64(7);
            c.w64(8, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4948); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4953); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4954); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4961, 0x70007f495c);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4961); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4963); }
            blk = 50; continue;
        }
        56 => {
            c.w32(11, c.r32(11));
            let r90 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r90);
            c.w64(8, 0xfffffffffffc0000u64);
            let r91 = c.r64(8) & c.r64(11);
            c.w64(8, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4976); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f4985); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f4986); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70007f4993, 0x70007f498e);
        }
        58 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f4993); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f4995); }
            blk = 31; continue;
        }
        59 => {
            c.w32(8, c.r32(8));
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(8);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f49ab); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f49ba); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f49bb); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f1594, 0x70007f49c8, 0x70007f49c3);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f49c8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f49ca); }
            blk = 23; continue;
        }
        62 => {
            return RegfileExit::jump(0x70007f49d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f49f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f49f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f49f8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70007f49fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007f4a02); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f4a06); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4a10); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4a16); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70007f4a1d); }
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4a21); };
            c.cmp(t6, 0xbau64, 16);
            if c.cc_b() { blk = 33; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4a2e); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f4a34); }
            c.w64(0, c.r64(1));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x4e0)) else { return RegfileExit::fault(0x70007f4a3b); };
            c.w64(3, t9);
            return c.call(0x7000870914, 0x70007f4a47, 0x70007f4a42);
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70007f4a47); }
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f4a4b); };
            c.cmp(t10, c.r32(0), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f4a58); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f4a6c); };
            c.w64(6, t12);
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4a70); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4a77); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f4a7e); };
            c.w32(11, t17);
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t18) = c.ld64(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f4a89); };
            c.w64(1, t18);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70007f4a9a);
        }
        7 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70007f4a9b); };
            c.w32(9, t19);
            let Some(t20) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f4aa2); };
            c.w64(11, (t20 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70007f4aa6); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 37; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f4ac1); };
            c.w64(12, t21);
            let r22 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r22);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r23 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r23);
            if c.cc_o() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r24 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r24);
            if c.cc_o() { blk = 23; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f4aed); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f4af4); };
            c.w64(7, t26);
            c.w64(6, c.r64(7).wrapping_add(c.r64(3)));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f4afb); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70007f4b02); };
            c.cmp(t28, c.r64(6), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 35; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f4b18); };
            c.w64(0, t29);
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70007f4b1f); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f4b26); };
            c.w64(3, t30);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70007f4b2d); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f4b30); }
            c.w64(12, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f4b3e); };
            c.w64(15, t31);
            blk = 16; continue;
        }
        15 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70007f4b47); }
            let r32 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r32);
            let r33 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r33);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t34) = c.ld64(c.r64(8).wrapping_add(c.r64(6).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f4b62); };
            c.w64(3, t34);
            blk = 15; continue;
        }
        20 => {
            c.w64(3, c.r64(15));
            blk = 15; continue;
        }
        21 => {
            return RegfileExit::fault(0x70007f4b6d);
        }
        22 => {
            c.w64(8, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f4b71); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f4b75); };
            c.w64(0, t36);
            blk = 26; continue;
        }
        23 => {
            return RegfileExit::fault(0x70007f4b7b);
        }
        24 => {
            return RegfileExit::fault(0x70007f4b7c);
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f4b7d); };
            c.w64(8, t37);
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f4b84); };
            c.w32(12, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f4b88); };
            c.w64(15, t39);
            let Some(t40) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f4b8f); };
            c.w64(3, t40);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f4b96); };
            c.w64(7, t41);
            let Some(t42) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f4b9d); };
            c.cmp(t42, c.r64(2), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70007f4ba6); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007f4bad); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f4bb1); };
            c.w64(9, t43);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70007f4bb8); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007f4bbc); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70007f4bc0); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007f4bc4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f4bc6); }
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f4bc7); };
            if !c.push64(t44) { return RegfileExit::fault(0x70007f4bc7); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f4bca); };
            if !c.push64(t45) { return RegfileExit::fault(0x70007f4bca); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70007f4bdd, 0x70007f4bd8);
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f4bdd); };
            c.w64(1, t46);
            let r47 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r47);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f4be8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f4be9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f4bef); }
            return c.ret(0x0, 0x70007f4bf1);
        }
        30 => {
            let Some(t48) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f4bf2); };
            c.w64(5, t48);
            let Some(t49) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007f4bf6); };
            c.w64(7, t49);
            let Some(t50) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f4bfb); };
            c.w64(0, t50);
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007f4c00); };
            c.w64(6, t51);
            let r52 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r52);
            return RegfileExit::jump(0x70007f2274);
        }
        31 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f4c0e); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70007f4c12); };
            c.w64(3, t53);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70007f4c23, 0x70007f4c1e);
        }
        32 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f4c23); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f4c27); };
            c.w64(6, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f4c2b); };
            c.w64(7, t56);
            blk = 1; continue;
        }
        33 => {
            c.w32(1, 0x130u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f4c39); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x4e0)) else { return RegfileExit::fault(0x70007f4c3a); };
            c.w64(3, t57);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f4c41); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007f4c42); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007f4c53, 0x70007f4c4e);
        }
        34 => {
            return RegfileExit::fault(0x70007f4c53);
        }
        35 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70007f4c54); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70007f4c58); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70007f4c5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70007f4c60); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70007f4c64); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70007f4c70, 0x70007f4c6b);
        }
        36 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70007f4c70); };
            c.w64(12, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f4c74); };
            c.w64(11, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f4c78); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f4c7c); };
            c.w64(9, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f4c80); };
            c.w64(8, t63);
            c.w32(2, 0x8u64);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f4c89); };
            c.w64(15, t64);
            blk = 14; continue;
        }
        37 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f4c92); };
            c.w64(3, t65);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f4ca2, 0x70007f4c9d);
        }
        38 => {
            return RegfileExit::fault(0x70007f4ca2);
        }
        39 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70007f4ca8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70007f4cac); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70007f4cb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70007f4cb4); }
            return c.call(0x70007febb4, 0x70007f4cbd, 0x70007f4cb8);
        }
        40 => {
            c.w64(15, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f4cc0); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f4cc4); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f4cc8); };
            c.w64(1, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f4ccc); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f4cd0); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f4cd4); };
            c.w64(9, t71);
            blk = 28; continue;
        }
        41 => {
            return RegfileExit::jump(0x70007f4ce0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f4cf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f4cf7); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(1), 64);
            if c.cc_le() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(1).wrapping_mul(8));
            blk = 3; continue;
        }
        2 => {
            let r2 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r2);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f4d1b); }
            let r3 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r3);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            let r4 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r4);
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f4d3e); };
            c.w64(10, t5);
            if !c.st64(c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f4d42); }
            let r6 = c.inc(c.r64(12), 64);
            c.w64(12, r6);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_l() { blk = 5; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4d5a); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f4d62); };
            c.cmp(c.r32(12), t9, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f4d6b); };
            c.w64(12, t10);
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(12)) { return RegfileExit::fault(0x70007f4d72); }
            let r11 = c.inc(c.r32(9), 32);
            c.w32(9, r11);
            blk = 8; continue;
        }
        12 => {
            let r12 = c.add(c.r64(0), c.r64(9), 64);
            c.w64(0, r12);
            return RegfileExit::jump(0x70007f2274);
        }
        13 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f4d85); };
            c.w64(3, t13);
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            return RegfileExit::fault(0x70007f4d91);
        }
        15 => {
            return RegfileExit::fault(0x70007f4d92);
        }
        16 => {
            return RegfileExit::fault(0x70007f4d93);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f4db4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f4db4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f4db8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007f4dbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70007f4dc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f4dc6); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4dd3); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4dda); };
            c.cmp(t3, 0x834u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f4de7); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f4dee); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f4df5); };
            c.w32(11, t8);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70007f4e05);
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f4e06); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f4e0d); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f4e17); };
            c.w64(9, t11);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4e21); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f4e2b); };
            c.w64(9, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4e32); };
            c.cmp(t14, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f4e3c); };
            c.w32(8, t15);
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f4e45); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let r19 = c.r32(8) & 0x3fu64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 30; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 30; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 31; continue;
        }
        11 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4e6f); };
            c.w32(8, t20);
            let r21 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007f4e76); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 42; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f4e94); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f4e9b); };
            c.w64(15, t23);
            c.w64(2, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f4eab); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f4eb2); };
            c.cmp(t25, c.r64(3), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 38; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70007f4ec8); }
            let r26 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r26);
            blk = 16; continue;
        }
        16 => {
            c.w32(3, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f4ed4); };
            c.w64(2, t27);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70007f4edb); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70007f4edf); }
            c.w64(3, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(2, c.r64(3));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f4ef4); };
            c.w64(6, t28);
            blk = 18; continue;
        }
        18 => {
            let r29 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r29);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(6)) { return RegfileExit::fault(0x70007f4eff); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(11).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(2, 0xfff7fffffff7ffffu64);
            blk = 22; continue;
        }
        21 => {
            let r30 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f4f2b); };
            c.w64(9, t31);
            blk = 22; continue;
        }
        22 => {
            let r32 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r32);
            let Some(t33) = c.ld64(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007f4f33); };
            c.cmp(t33, c.r64(2), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x70007f4f39); }
            let Some(t34) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f4f3f); };
            c.w64(6, t34);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            c.w64(9, c.r64(3).wrapping_sub(0x4));
            let Some(t35) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f4f4b); };
            c.cmp(t35, c.r64(7), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(12), c.r64(7)) { return RegfileExit::fault(0x70007f4f54); }
            let r36 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r36);
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f4f5c); };
            c.w64(7, t37);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007f4f63); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f4f66); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(6)) { return RegfileExit::fault(0x70007f4f6b); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r38 = c.r64(7) & c.r64(15);
            c.w64(7, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f4f7f); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f4f8f); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f4f93); };
            c.w64(0, t41);
            blk = 29; continue;
        }
        29 => {
            let Some(t42) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f4f97); };
            c.w64(5, t42);
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f4f9b); };
            c.w64(6, t43);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let r44 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r44);
            return RegfileExit::jump(0x70007f4cf4);
        }
        30 => {
            let Some(t45) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f4faf); };
            c.w32(8, t45);
            let r46 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r46);
            let Some(t47) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f4fb6); };
            c.w64(5, t47);
            c.w64(3, c.r64(9));
            c.w64(1, c.r64(8));
            let r48 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r48);
            return RegfileExit::jump(0x70007f4cf4);
        }
        31 => {
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f4fc9); };
            c.cmp(t49, c.r32(3), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f4fd8); };
            c.cmp(t50, c.r32(3), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70007f4fe7); }
            let r51 = c.r32(8) | c.r32(9);
            c.w32(8, r51);
            c.logic(r51, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(0, c.r64(3));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f4ff7); };
            c.w64(3, t52);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70007f5006, 0x70007f5001);
        }
        33 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f500e); };
            c.w32(8, t53);
            let r54 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r54);
            let Some(t55) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f5015); };
            c.logic(t55 & 0x2u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(3, c.r64(0));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5023); };
            c.w64(0, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5027); };
            c.w64(6, t57);
            return c.call(0x7000845bd4, 0x70007f5030, 0x70007f502b);
        }
        36 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5030); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            let Some(t60) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f5037); };
            c.w32(8, t60);
            let r61 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r61);
            let Some(t62) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5040); };
            c.w32(9, t62);
            let r63 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r63);
            let r64 = c.r32(8) & 0x3fu64;
            c.w32(8, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(3, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5058); };
            c.w64(0, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f505c); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f5060); };
            c.w64(7, t67);
            blk = 11; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007f5069); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70007f506d); }
            return c.call(0x70007febb4, 0x70007f5076, 0x70007f5071);
        }
        39 => {
            c.w64(15, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5079); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f507d); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5081); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5085); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f5089); };
            c.w64(8, t72);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f508d); };
            c.w64(12, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f5094); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f509b); };
            c.w64(11, t75);
            blk = 16; continue;
        }
        40 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007f50a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70007f50ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70007f50b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70007f50b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70007f50b9); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70007f50bd); }
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x70007f50ca, 0x70007f50c5);
        }
        41 => {
            c.w64(6, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f50cd); };
            c.w64(8, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f50d1); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f50d5); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f50d9); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f50dd); };
            c.w64(9, t80);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70007f50e1); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f50e6); };
            c.w64(12, t81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f50ed); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(0x70007f4f1du64) else { return RegfileExit::fault(0x70007f50f4); };
            c.w64(2, t83);
            blk = 25; continue;
        }
        42 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f5100); };
            c.w64(3, t84);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f5110, 0x70007f510b);
        }
        43 => {
            return RegfileExit::fault(0x70007f5110);
        }
        44 => {
            c.w64(3, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5114); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5118); };
            c.w64(6, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f511c); };
            c.w64(7, t87);
            c.w64(8, c.r64(0));
            blk = 30; continue;
        }
        45 => {
            c.w32(8, 0x90u64);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f512e); };
            c.w64(3, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5132); };
            c.w64(6, t89);
            blk = 47; continue;
        }
        46 => {
            c.w32(8, 0xd0u64);
            blk = 47; continue;
        }
        47 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f513e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f5140); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f5141); };
            c.w64(3, t90);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007f5152, 0x70007f514d);
        }
        48 => {
            return RegfileExit::fault(0x70007f5152);
        }
        49 => {
            c.w32(6, c.r32(6));
            let r91 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r91);
            c.w64(7, 0xfffffffffffc0000u64);
            let r92 = c.r64(7) & c.r64(6);
            c.w64(7, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5162); };
            c.logic(t93 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f5170); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f5171); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f517e, 0x70007f5179);
        }
        51 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f517e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f5180); }
            blk = 27; continue;
        }
        52 => {
            return RegfileExit::jump(0x70007f5188);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f5194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f5194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f5198); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f519e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f51a2); };
            c.w64(9, t2);
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f51a6); };
            c.w64(8, t3);
            let Some(t4) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f51aa); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f51ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70007f51b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70007f51b6); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f51ba); };
            c.cmp(t5, c.r32(11), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(11, c.r64(2).wrapping_mul(4));
            c.w64(12, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f51d7); };
            c.w32(15, t6);
            let r7 = c.add(c.r32(15), 0x8u64, 32);
            c.w32(15, r7);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70007f51e3); }
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f51e8); };
            c.w32(15, t8);
            let r9 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r9);
            c.w32(2, c.r32(15));
            let r10 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f51fe); };
            c.cmp(t11, c.r32(15), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f520b); };
            c.cmp(t12, c.r32(15), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5222); };
            c.w32(2, t13);
            c.w64(6, c.r64(15));
            let r14 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r14);
            c.logic(r14, 64);
            c.w32(7, c.r32(2));
            let r15 = c.r32(7) & 0x3u64;
            c.w32(7, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5254); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f525b); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5261); };
            c.w32(7, t20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f5270);
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f5271); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(2, c.r64(5));
            c.w32(15, c.r32(15));
            c.w64(6, 0xfffffffffffffff8u64);
            let Some(t22) = c.ld64(c.r64(6).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f5287); };
            c.w64(2, t22);
            let r23 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(2), c.r32(15), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5299); };
            c.w32(15, t24);
            c.w32(2, c.r32(0));
            c.w32(6, c.r32(15));
            let r25 = c.r32(2) & 0x1u64;
            c.w32(2, r25);
            c.logic(r25, 32);
            let r26 = c.r32(6) & 0x3u64;
            c.w32(6, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r27 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r27);
            c.logic(r27, 64);
            let r28 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r28);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70007f52c1); }
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f52c6); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(15, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t29) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f52dc); };
            c.w32(15, t29);
            let r30 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r30);
            blk = 18; continue;
        }
        18 => {
            let Some(t31) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f52e3); };
            c.w32(2, t31);
            let r32 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r32);
            let Some(t33) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f52ea); };
            c.w32(2, t33);
            c.cmp(c.r16(2), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(2, c.r16(2));
            let r34 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r34);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t35) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f5307); };
            c.w32(15, t35);
            let r36 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r36);
            let Some(t37) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f530e); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5315); };
            c.w32(15, t39);
            let r40 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f531c); };
            c.w32(2, t41);
            let Some(t42) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f531f); };
            c.cmp(t42, c.r32(2), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, c.r64(0));
            let r43 = c.r64(11) | 0x3u64;
            c.w64(11, r43);
            c.logic(r43, 64);
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(11)) { return RegfileExit::fault(0x70007f5330); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r44 = c.r64(15) & c.r64(9);
            c.w64(15, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5344); };
            c.logic(t45 & 0x4u64, 8);
            if c.cc_ne() { blk = 110; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f534f); }
            blk = 52; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5364); };
            c.w32(15, t46);
            let r47 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r47);
            let Some(t48) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f536b); };
            c.w32(15, t48);
            let r49 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r49);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5381); };
            c.w32(15, t50);
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t51) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f538e); };
            c.w32(2, t51);
            let r52 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r52);
            let Some(t53) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5394); };
            c.w32(2, t53);
            let r54 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r54);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t55) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f53a7); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            c.cmp(c.r32(15), c.r32(2), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let r57 = c.r64(2) | 0x3u64;
            c.w64(2, r57);
            c.logic(r57, 64);
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70007f53ba); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r58 = c.r64(11) & c.r64(9);
            c.w64(11, r58);
            c.logic(r58, 64);
            let Some(t59) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f53cd); };
            c.logic(t59 & 0x4u64, 8);
            if c.cc_ne() { blk = 107; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f53d8); }
            blk = 52; continue;
        }
        32 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f53e5); };
            c.w32(15, t60);
            c.w32(2, c.r32(15));
            let r61 = c.r32(2) & 0x3u64;
            c.w32(2, r61);
            c.logic(r61, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r62 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r62);
            c.logic(r62, 64);
            let r63 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r63);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70007f5408); }
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f540d); }
            blk = 35; continue;
        }
        35 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5415); };
            c.w32(15, t64);
            let r65 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r65);
            let Some(t66) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f541c); };
            c.w32(15, t66);
            let r67 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r67);
            let Some(t68) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5423); };
            c.w32(2, t68);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f5433);
        }
        37 => {
            let Some(t69) = c.ld32(c.r64(15).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f5434); };
            c.cmp(t69, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, c.r64(0));
            blk = 42; continue;
        }
        39 => {
            c.w64(15, c.r64(5));
            let Some(t70) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5442); };
            c.w32(6, t70);
            c.w64(2, 0xfffffffffffffff8u64);
            let Some(t71) = c.ld64(c.r64(2).wrapping_add(c.r64(15)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f544e); };
            c.w64(15, t71);
            c.w32(2, c.r32(6));
            let r72 = c.r32(2) & 0x3u64;
            c.w32(2, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r73 = c.r64(2) | 0x2u64;
            c.w64(2, r73);
            c.logic(r73, 64);
            let r74 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r74);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x70007f546f); }
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5474); }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(2, c.r64(15));
            blk = 45; continue;
        }
        44 => {
            let Some(t75) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f548b); };
            c.w32(2, t75);
            let r76 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r76);
            blk = 45; continue;
        }
        45 => {
            let Some(t77) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5491); };
            c.w32(6, t77);
            let r78 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r78);
            let Some(t79) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5497); };
            c.w32(6, t79);
            c.cmp(c.r16(6), 0x80bu64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(6, c.r16(6));
            let r80 = c.sub(c.r32(6), 0x80cu64, 32);
            c.w32(6, r80);
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t81) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f54b0); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            let Some(t83) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f54b6); };
            c.w32(6, t83);
            let r84 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r84);
            let Some(t85) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f54bd); };
            c.w32(2, t85);
            let r86 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r86);
            let Some(t87) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f54c3); };
            c.w32(6, t87);
            let Some(t88) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f54c6); };
            c.cmp(t88, c.r32(6), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            let r89 = c.r64(15) | 0x3u64;
            c.w64(15, r89);
            c.logic(r89, 64);
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x70007f54cf); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r90 = c.r64(11) & c.r64(9);
            c.w64(11, r90);
            c.logic(r90, 64);
            let Some(t91) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f54e3); };
            c.logic(t91 & 0x4u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f54ee); }
            blk = 52; continue;
        }
        51 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f54f8); };
            c.w64(12, t92);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x1f), c.r32(12)) { return RegfileExit::fault(0x70007f54ff); }
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5504); }
            blk = 52; continue;
        }
        52 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5515); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            let Some(t95) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f551c); };
            c.cmp(t95, 0x834u64, 16);
            if c.cc_ne() { blk = 83; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t96) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5529); };
            c.w32(11, t96);
            let r97 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5530); };
            c.w32(11, t98);
            let r99 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r99);
            let Some(t100) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5537); };
            c.w32(12, t100);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            return RegfileExit::fault(0x70007f5547);
        }
        56 => {
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f5548); };
            c.w32(11, t101);
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f554f); };
            c.cmp(t102, c.r32(11), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f5559); };
            c.w64(11, t103);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t104) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5563); };
            c.cmp(t104, c.r32(12), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f556d); };
            c.w64(11, t105);
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5574); };
            c.cmp(t106, c.r32(12), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t107) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f557e); };
            c.w32(9, t107);
            let r108 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r108);
            let Some(t109) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5587); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let r111 = c.r32(9) & 0x3fu64;
            c.w32(9, r111);
            c.logic(r111, 32);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_le() { blk = 82; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_le() { blk = 82; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 83; continue;
        }
        63 => {
            let Some(t112) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f55b1); };
            c.w32(9, t112);
            let r113 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r113);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70007f55b8); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 94; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f55d6); };
            c.w64(15, t114);
            let Some(t115) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f55dd); };
            c.w64(2, t115);
            c.w64(1, c.r64(12).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f55ec); };
            c.w64(7, t116);
            let Some(t117) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f55f3); };
            c.cmp(t117, c.r64(6), 64);
            if c.cc_be() { blk = 90; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 90; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70007f5609); }
            let r118 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r118);
            blk = 68; continue;
        }
        68 => {
            c.w32(1, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f5614); };
            c.w64(6, t119);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f561b); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70007f561e); }
            c.w64(1, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(6, c.r64(1));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f5632); };
            c.w64(8, t120);
            blk = 70; continue;
        }
        70 => {
            let r121 = c.sub(c.r64(6), 0x4u64, 64);
            c.w64(6, r121);
            if !c.st32(c.r64(6).wrapping_add(c.r64(2)), c.r32(8)) { return RegfileExit::fault(0x70007f563d); }
            c.cmp(c.r64(6), 0x7u64, 64);
            if c.cc_ne() { blk = 70; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 81; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            blk = 74; continue;
        }
        73 => {
            let r122 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5669); };
            c.w64(11, t123);
            blk = 74; continue;
        }
        74 => {
            let r124 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r124);
            let Some(t125) = c.ld64(c.r64(8).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70007f5671); };
            c.cmp(t125, c.r64(12), 64);
            if c.cc_e() { blk = 79; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(11))) { return RegfileExit::fault(0x70007f5677); }
            let Some(t126) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f567d); };
            c.w64(6, t126);
            c.w64(3, c.r64(6).wrapping_add(0xc));
            c.w64(11, c.r64(1).wrapping_sub(0x4));
            let Some(t127) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f5688); };
            c.cmp(t127, c.r64(3), 64);
            if c.cc_be() { blk = 92; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70007f5691); }
            c.w64(3, c.r64(6).wrapping_add(0x1));
            blk = 77; continue;
        }
        77 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f5698); };
            c.w64(6, t128);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f569f); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f56a2); }
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(3)) { return RegfileExit::fault(0x70007f56a7); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r129 = c.r64(6) & c.r64(2);
            c.w64(6, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f56ba); };
            c.logic(t130 & 0x4u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 73; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f56ca); };
            c.w64(0, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f56ce); };
            c.w64(3, t132);
            blk = 81; continue;
        }
        81 => {
            let Some(t133) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f56d2); };
            c.w64(5, t133);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(1, c.r64(9));
            c.w64(3, c.r64(2));
            let Some(t134) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f56e2); };
            c.w64(6, t134);
            let r135 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r135);
            return RegfileExit::jump(0x70007f4cf4);
        }
        82 => {
            let Some(t136) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f56f0); };
            c.w32(9, t136);
            let r137 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r137);
            let Some(t138) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f56f7); };
            c.w64(5, t138);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(1, c.r64(9));
            c.w64(3, c.r64(11));
            c.w64(6, c.r64(8));
            let r139 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r139);
            return RegfileExit::jump(0x70007f4cf4);
        }
        83 => {
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f5713); };
            c.cmp(t140, c.r32(1), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f5722); };
            c.cmp(t141, c.r32(1), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007f5731); }
            let r142 = c.r32(9) | c.r32(11);
            c.w32(9, r142);
            c.logic(r142, 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(6, c.r64(8));
            c.w64(0, c.r64(1));
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f5744); };
            c.w64(3, t143);
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70007f5753, 0x70007f574e);
        }
        85 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t144) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f575b); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f5762); };
            c.logic(t146 & 0x2u64, 8);
            if c.cc_e() { blk = 97; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(0));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5770); };
            c.w64(0, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5774); };
            c.w64(6, t148);
            return c.call(0x7000845bd4, 0x70007f577d, 0x70007f5778);
        }
        88 => {
            let Some(t149) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f577d); };
            c.w32(8, t149);
            let r150 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r150);
            let Some(t151) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f5784); };
            c.w32(8, t151);
            let r152 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r152);
            let Some(t153) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f578d); };
            c.w32(11, t153);
            let r154 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r154);
            let r155 = c.r32(8) & 0x3fu64;
            c.w32(8, r155);
            c.logic(r155, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 96; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f57a2); };
            c.w64(3, t156);
            c.w64(1, c.r64(0));
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f57a9); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f57ad); };
            c.w64(0, t158);
            blk = 63; continue;
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007f57b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70007f57ba); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70007f57c6, 0x70007f57c1);
        }
        91 => {
            c.w64(2, c.r64(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f57c9); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f57cd); };
            c.w64(0, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f57d1); };
            c.w64(3, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f57d5); };
            c.w64(11, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f57d9); };
            c.w64(9, t163);
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f57dd); };
            c.w64(15, t164);
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f57e4); };
            c.w64(7, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f57eb); };
            c.w64(12, t166);
            blk = 68; continue;
        }
        92 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70007f57f4); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007f57fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70007f5801); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70007f5805); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70007f5809); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70007f580d); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70007f581a, 0x70007f5815);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f581d); };
            c.w64(9, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f5821); };
            c.w64(2, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f5825); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f5829); };
            c.w64(8, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f582d); };
            c.w64(11, t171);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70007f5831); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f5836); };
            c.w64(15, t172);
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f583d); };
            c.w64(7, t173);
            let Some(t174) = c.ld64(0x70007f565bu64) else { return RegfileExit::fault(0x70007f5844); };
            c.w64(12, t174);
            blk = 77; continue;
        }
        94 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f5850); };
            c.w64(3, t175);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f5860, 0x70007f585b);
        }
        95 => {
            return RegfileExit::fault(0x70007f5860);
        }
        96 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5861); };
            c.w64(3, t176);
            c.w64(1, c.r64(0));
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5868); };
            c.w64(8, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f586c); };
            c.w64(0, t178);
            c.w64(7, c.r64(0));
            blk = 82; continue;
        }
        97 => {
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5878); };
            c.w64(8, t179);
            c.w32(9, 0x90u64);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5882); };
            c.w64(1, t180);
            blk = 99; continue;
        }
        98 => {
            c.w32(9, 0xd0u64);
            blk = 99; continue;
        }
        99 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007f588e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f5890); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f5891); };
            c.w64(3, t181);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(8));
            return c.call(0x700086ed74, 0x70007f58a5, 0x70007f58a0);
        }
        100 => {
            return RegfileExit::fault(0x70007f58a5);
        }
        101 => {
            c.w32(3, c.r32(3));
            let r182 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r182);
            c.w64(6, 0xfffffffffffc0000u64);
            let r183 = c.r64(6) & c.r64(3);
            c.w64(6, r183);
            c.logic(r183, 64);
            let Some(t184) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f58b5); };
            c.logic(t184 & 0x2u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(0, c.r64(2).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f58c3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f58c4); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f58d1, 0x70007f58cc);
        }
        103 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f58d1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f58d3); }
            blk = 79; continue;
        }
        104 => {
            c.w32(15, c.r32(15));
            let r185 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r185);
            c.w64(11, 0xfffffffffffc0000u64);
            let r186 = c.r64(11) & c.r64(15);
            c.w64(11, r186);
            c.logic(r186, 64);
            let Some(t187) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f58e9); };
            c.logic(t187 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f58f8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f58f9); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70007f5906, 0x70007f5901);
        }
        106 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f5906); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f5908); }
            blk = 50; continue;
        }
        107 => {
            c.w32(2, c.r32(2));
            let r188 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r188);
            c.w64(11, 0xfffffffffffc0000u64);
            let r189 = c.r64(11) & c.r64(2);
            c.w64(11, r189);
            c.logic(r189, 64);
            let Some(t190) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f591d); };
            c.logic(t190 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f592c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f592d); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f593a, 0x70007f5935);
        }
        109 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f593a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f593c); }
            blk = 31; continue;
        }
        110 => {
            c.w32(11, c.r32(11));
            let r191 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r191);
            c.w64(15, 0xfffffffffffc0000u64);
            let r192 = c.r64(15) & c.r64(11);
            c.w64(15, r192);
            c.logic(r192, 64);
            let Some(t193) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5952); };
            c.logic(t193 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(2, c.r64(9).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f5961); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f5962); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70007f596f, 0x70007f596a);
        }
        112 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f596f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f5971); }
            blk = 23; continue;
        }
        113 => {
            return RegfileExit::jump(0x70007f5978);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f5994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f5994); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f5998); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f599e); };
            c.w32(8, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f59a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70007f59a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f59aa); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f59ae); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(2).wrapping_mul(4));
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f59cb); };
            c.w32(11, t3);
            let r4 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r4);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f59d7); }
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f59dc); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(12, c.r32(11));
            let r7 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f59f4); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f5a01); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5a18); };
            c.w32(12, t10);
            c.w64(15, c.r64(11));
            let r11 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r11);
            c.logic(r11, 64);
            c.w32(2, c.r32(12));
            let r12 = c.r32(2) & 0x3u64;
            c.w32(2, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5a4d); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5a54); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5a5c); };
            c.w32(2, t17);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f5a6d);
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f5a6e); };
            c.cmp(t18, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, c.r32(11));
            let r19 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            let Some(t20) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f5a80); };
            c.w64(12, t20);
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5a8d); };
            c.w32(11, t21);
            c.w32(12, c.r32(0));
            c.w32(15, c.r32(11));
            let r22 = c.r32(12) & 0x1u64;
            c.w32(12, r22);
            c.logic(r22, 32);
            let r23 = c.r32(15) & 0x3u64;
            c.w32(15, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r24 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r24);
            c.logic(r24, 64);
            let r25 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r25);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f5ab9); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5abe); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5ad4); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5adb); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5ae2); };
            c.w32(12, t30);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(12, c.r16(12));
            let r31 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r31);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f5b05); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5b0c); };
            c.w32(12, t34);
            let r35 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5b13); };
            c.w32(11, t36);
            let r37 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5b1a); };
            c.w32(12, t38);
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5b1f); };
            c.cmp(t39, c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(0));
            let r40 = c.r64(8) | 0x3u64;
            c.w64(8, r40);
            c.logic(r40, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f5b30); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r41 = c.r64(11) & c.r64(7);
            c.w64(11, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5b44); };
            c.logic(t42 & 0x4u64, 8);
            if c.cc_ne() { blk = 111; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5b4f); }
            blk = 53; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5b63); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            let Some(t45) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5b6a); };
            c.w32(11, t45);
            let r46 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r46);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5b80); };
            c.w32(11, t47);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t48) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5b8d); };
            c.w32(12, t48);
            let r49 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r49);
            let Some(t50) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5b94); };
            c.w32(12, t50);
            let r51 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r51);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5bab); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 29; continue;
        }
        29 => {
            let r54 = c.r64(12) | 0x3u64;
            c.w64(12, r54);
            c.logic(r54, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70007f5bbf); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r55 = c.r64(8) & c.r64(7);
            c.w64(8, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5bd3); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 108; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5bde); }
            blk = 53; continue;
        }
        32 => {
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5bea); };
            c.w32(11, t57);
            c.w32(12, c.r32(11));
            let r58 = c.r32(12) & 0x3u64;
            c.w32(12, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r59 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r59);
            c.logic(r59, 64);
            let r60 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r60);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f5c0f); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5c14); }
            blk = 35; continue;
        }
        35 => {
            let Some(t61) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5c1b); };
            c.w32(11, t61);
            let r62 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r62);
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5c22); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5c29); };
            c.w32(12, t65);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f5c39);
        }
        37 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f5c3a); };
            c.cmp(t66, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(12, c.r64(0));
            blk = 43; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f5c45); };
            c.w32(11, t67);
            c.w32(12, c.r32(11));
            let r68 = c.r32(12) & 0x3u64;
            c.w32(12, r68);
            c.logic(r68, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r69 = c.r64(11) | 0x2u64;
            c.w64(11, r69);
            c.logic(r69, 64);
            let r70 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r70);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f5c6a); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5c6f); }
            blk = 42; continue;
        }
        42 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f5c76); };
            c.w64(12, t71);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(12));
            blk = 46; continue;
        }
        45 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5c89); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5c90); };
            c.w32(15, t74);
            let r75 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r75);
            let Some(t76) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5c97); };
            c.w32(15, t76);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 45; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w32(15, c.r16(15));
            let r77 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r77);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f5cb5); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5cbc); };
            c.w32(15, t80);
            let r81 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r81);
            let Some(t82) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5cc3); };
            c.w32(11, t82);
            let r83 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r83);
            let Some(t84) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5cca); };
            c.w32(15, t84);
            let Some(t85) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5cce); };
            c.cmp(t85, c.r32(15), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            let r86 = c.r64(12) | 0x3u64;
            c.w64(12, r86);
            c.logic(r86, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70007f5cd8); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r87 = c.r64(8) & c.r64(7);
            c.w64(8, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5cec); };
            c.logic(t88 & 0x4u64, 8);
            if c.cc_ne() { blk = 105; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5cf7); }
            blk = 53; continue;
        }
        52 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f5d00); };
            c.w64(9, t89);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70007f5d07); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f5d0c); }
            blk = 53; continue;
        }
        53 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 84; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5d1c); };
            c.w32(8, t90);
            let r91 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r91);
            let Some(t92) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5d23); };
            c.cmp(t92, 0x834u64, 16);
            if c.cc_ne() { blk = 84; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t93) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5d30); };
            c.w32(9, t93);
            let r94 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r94);
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f5d37); };
            c.w32(9, t95);
            let r96 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r96);
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f5d3e); };
            c.w32(11, t97);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x70007f5d4e);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f5d4f); };
            c.w32(9, t98);
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f5d56); };
            c.cmp(t99, c.r32(9), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f5d60); };
            c.w64(9, t100);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5d6a); };
            c.cmp(t101, c.r32(11), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f5d74); };
            c.w64(9, t102);
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5d7b); };
            c.cmp(t103, c.r32(11), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t104) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f5d85); };
            c.w32(8, t104);
            let r105 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r105);
            let Some(t106) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5d8e); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            let r108 = c.r32(8) & 0x3fu64;
            c.w32(8, r108);
            c.logic(r108, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 83; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 83; continue; }
            blk = 63; continue;
        }
        63 => {
            blk = 84; continue;
        }
        64 => {
            let Some(t109) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5db8); };
            c.w32(8, t109);
            let r110 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r110);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007f5dbf); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 95; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f5ddd); };
            c.w64(12, t111);
            let Some(t112) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f5de4); };
            c.w64(15, t112);
            c.w64(2, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f5df4); };
            c.w64(7, t113);
            let Some(t114) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f5dfb); };
            c.cmp(t114, c.r64(1), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 91; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x70007f5e11); }
            let r115 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r115);
            blk = 69; continue;
        }
        69 => {
            c.w32(2, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f5e1d); };
            c.w64(1, t116);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f5e24); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70007f5e28); }
            c.w64(2, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(1, c.r64(2));
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f5e3d); };
            c.w64(6, t117);
            blk = 71; continue;
        }
        71 => {
            let r118 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r118);
            if !c.st32(c.r64(1).wrapping_add(c.r64(15)), c.r32(6)) { return RegfileExit::fault(0x70007f5e48); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 71; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(11, c.r64(11).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 82; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(1, 0xfff7fffffff7ffffu64);
            blk = 75; continue;
        }
        74 => {
            let r119 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5e74); };
            c.w64(9, t120);
            blk = 75; continue;
        }
        75 => {
            let r121 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r121);
            let Some(t122) = c.ld64(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007f5e7c); };
            c.cmp(t122, c.r64(1), 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 76; continue;
        }
        76 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x70007f5e82); }
            let Some(t123) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f5e88); };
            c.w64(6, t123);
            c.w64(3, c.r64(6).wrapping_add(0xc));
            c.w64(9, c.r64(2).wrapping_sub(0x4));
            let Some(t124) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f5e94); };
            c.cmp(t124, c.r64(3), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70007f5e9d); }
            c.w64(3, c.r64(6).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f5ea5); };
            c.w64(6, t125);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f5eac); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f5eaf); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(3)) { return RegfileExit::fault(0x70007f5eb4); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r126 = c.r64(6) & c.r64(15);
            c.w64(6, r126);
            c.logic(r126, 64);
            let Some(t127) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f5ec7); };
            c.logic(t127 & 0x4u64, 8);
            if c.cc_ne() { blk = 102; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 74; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f5ed7); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5edb); };
            c.w64(3, t129);
            blk = 82; continue;
        }
        82 => {
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f5edf); };
            c.w64(5, t130);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(1, c.r64(8));
            c.w64(3, c.r64(15));
            let Some(t131) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f5eef); };
            c.w64(6, t131);
            let Some(t132) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f5ef4); };
            c.w64(10, t132);
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(10)) { return RegfileExit::fault(0x70007f5ef9); }
            let r133 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r133);
            return RegfileExit::jump(0x70007f4cf4);
        }
        83 => {
            let Some(t134) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f5f07); };
            c.w32(8, t134);
            let r135 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r135);
            let Some(t136) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f5f0e); };
            c.w64(5, t136);
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(1, c.r64(8));
            c.w64(3, c.r64(9));
            let Some(t137) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x70007f5f1e); };
            c.w64(10, t137);
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(10)) { return RegfileExit::fault(0x70007f5f23); }
            let r138 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(0x70007f4cf4);
        }
        84 => {
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f5f31); };
            c.cmp(t139, c.r32(1), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f5f40); };
            c.cmp(t140, c.r32(1), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007f5f4f); }
            let r141 = c.r32(8) | c.r32(9);
            c.w32(8, r141);
            c.logic(r141, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f5f5c); };
            c.w64(3, t142);
            c.w64(0, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70007f5f6e, 0x70007f5f69);
        }
        86 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t143) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5f76); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f5f7d); };
            c.logic(t145 & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5f8b); };
            c.w64(0, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5f8f); };
            c.w64(6, t147);
            return c.call(0x7000845bd4, 0x70007f5f98, 0x70007f5f93);
        }
        89 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f5f98); };
            c.w32(8, t148);
            let r149 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r149);
            let Some(t150) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f5f9f); };
            c.w32(8, t150);
            let r151 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r151);
            let Some(t152) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f5fa8); };
            c.w32(9, t152);
            let r153 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r153);
            let r154 = c.r32(8) & 0x3fu64;
            c.w32(8, r154);
            c.logic(r154, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 97; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5fbd); };
            c.w64(3, t155);
            c.w64(1, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5fc4); };
            c.w64(6, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f5fc8); };
            c.w64(0, t157);
            blk = 64; continue;
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007f5fd1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70007f5fd5); }
            return c.call(0x70007febb4, 0x70007f5fde, 0x70007f5fd9);
        }
        92 => {
            c.w64(15, c.r64(0));
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f5fe1); };
            c.w64(6, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f5fe5); };
            c.w64(0, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f5fe9); };
            c.w64(3, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f5fed); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f5ff1); };
            c.w64(8, t162);
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f5ff5); };
            c.w64(12, t163);
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f5ffc); };
            c.w64(7, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f6003); };
            c.w64(11, t165);
            blk = 69; continue;
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70007f600c); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007f6015); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70007f6019); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70007f601d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70007f6021); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70007f6025); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70007f6032, 0x70007f602d);
        }
        94 => {
            c.w64(3, c.r64(0));
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f6035); };
            c.w64(8, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f6039); };
            c.w64(15, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f603d); };
            c.w64(2, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f6041); };
            c.w64(11, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f6045); };
            c.w64(9, t170);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70007f6049); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f604e); };
            c.w64(12, t171);
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f6055); };
            c.w64(7, t172);
            let Some(t173) = c.ld64(0x70007f5e66u64) else { return RegfileExit::fault(0x70007f605c); };
            c.w64(1, t173);
            blk = 78; continue;
        }
        95 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f6068); };
            c.w64(3, t174);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f6078, 0x70007f6073);
        }
        96 => {
            return RegfileExit::fault(0x70007f6078);
        }
        97 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f6079); };
            c.w64(3, t175);
            c.w64(1, c.r64(0));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6080); };
            c.w64(6, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6084); };
            c.w64(0, t177);
            c.w64(7, c.r64(0));
            blk = 83; continue;
        }
        98 => {
            c.w32(8, 0x90u64);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6096); };
            c.w64(1, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f609a); };
            c.w64(6, t179);
            blk = 100; continue;
        }
        99 => {
            c.w32(8, 0xd0u64);
            blk = 100; continue;
        }
        100 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f60a6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f60a8); }
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f60a9); };
            c.w64(3, t180);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007f60ba, 0x70007f60b5);
        }
        101 => {
            return RegfileExit::fault(0x70007f60ba);
        }
        102 => {
            c.w32(3, c.r32(3));
            let r181 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r181);
            c.w64(6, 0xfffffffffffc0000u64);
            let r182 = c.r64(6) & c.r64(3);
            c.w64(6, r182);
            c.logic(r182, 64);
            let Some(t183) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f60ca); };
            c.logic(t183 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f60d8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f60d9); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f60e6, 0x70007f60e1);
        }
        104 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f60e6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f60e8); }
            blk = 80; continue;
        }
        105 => {
            c.w32(12, c.r32(12));
            let r184 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r184);
            c.w64(8, 0xfffffffffffc0000u64);
            let r185 = c.r64(8) & c.r64(12);
            c.w64(8, r185);
            c.logic(r185, 64);
            let Some(t186) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f60fe); };
            c.logic(t186 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f610d); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007f6117, 0x70007f6112);
        }
        107 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6117); }
            blk = 51; continue;
        }
        108 => {
            c.w32(12, c.r32(12));
            let r187 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r187);
            c.w64(8, 0xfffffffffffc0000u64);
            let r188 = c.r64(8) & c.r64(12);
            c.w64(8, r188);
            c.logic(r188, 64);
            let Some(t189) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f612e); };
            c.logic(t189 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f613d); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007f6147, 0x70007f6142);
        }
        110 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6147); }
            blk = 31; continue;
        }
        111 => {
            c.w32(8, c.r32(8));
            let r190 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r190);
            c.w64(11, 0xfffffffffffc0000u64);
            let r191 = c.r64(11) & c.r64(8);
            c.w64(11, r191);
            c.logic(r191, 64);
            let Some(t192) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f615e); };
            c.logic(t192 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f616d); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f6177, 0x70007f6172);
        }
        113 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6177); }
            blk = 23; continue;
        }
        114 => {
            return RegfileExit::jump(0x70007f6180);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f6194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f6198); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f619e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f61a2); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f61b0); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f61b7); };
            c.logic(t3 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f61cb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f61d2); };
            c.w32(8, t6);
            let r7 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f61d9); };
            c.w32(9, t8);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f61e0); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70007f61f3);
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70007f61f4); };
            c.cmp(t11, c.r32(11), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(9), 0xb0u64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70007f6206);
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70007f6207); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6210); };
            c.cmp(t13, 0x834u64, 16);
            if c.cc_ne() { blk = 43; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f621d); };
            c.w32(12, t14);
            c.w64(15, c.r64(12));
            let r15 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r15);
            let r16 = c.r32(15) & 0x3fu64;
            c.w32(15, r16);
            c.logic(r16, 32);
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_g() { blk = 43; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6237); };
            c.w32(15, t17);
            let r18 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r18);
            let Some(t19) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f623e); };
            c.w32(0, t19);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 12; continue;
        }
        12 => {
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            blk = 15; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f624f); };
            c.w32(8, t21);
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6253); };
            c.w32(15, t22);
            let r23 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r23);
            let Some(t24) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f625a); };
            c.cmp(t24, c.r32(8), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 14; continue;
        }
        14 => {
            let r25 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r25);
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f626f); };
            c.w64(15, t26);
            blk = 17; continue;
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6276); };
            c.w32(8, t27);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70007f627a); }
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70007f627e); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 49; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f629b); };
            c.w64(9, t29);
            let Some(t30) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f62a2); };
            c.w64(11, t30);
            c.w64(2, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f62b1); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007f62b8); };
            c.cmp(t32, c.r64(12), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 45; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70007f62ce); }
            let r33 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r33);
            blk = 22; continue;
        }
        22 => {
            c.w32(12, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f62d9); };
            c.w64(2, t34);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70007f62e0); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f62e4); }
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(2, c.r64(12));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f62f9); };
            c.w64(1, t35);
            blk = 24; continue;
        }
        24 => {
            let r36 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r36);
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x70007f6304); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(8, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(2, 0xfff7fffffff7ffffu64);
            blk = 28; continue;
        }
        27 => {
            let r37 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f6330); };
            c.w64(15, t38);
            blk = 28; continue;
        }
        28 => {
            let r39 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r39);
            let Some(t40) = c.ld64(c.r64(8).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70007f6338); };
            c.cmp(t40, c.r64(2), 64);
            if c.cc_e() { blk = 33; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(15))) { return RegfileExit::fault(0x70007f633e); }
            let Some(t41) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f6344); };
            c.w64(1, t41);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            c.w64(7, c.r64(12).wrapping_sub(0x4));
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007f6350); };
            c.cmp(t42, c.r64(6), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(6)) { return RegfileExit::fault(0x70007f6359); }
            let r43 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r43);
            blk = 31; continue;
        }
        31 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f6360); };
            c.w64(6, t44);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f6367); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f636a); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70007f636f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r45 = c.r64(6) & c.r64(11);
            c.w64(6, r45);
            c.logic(r45, 64);
            let Some(t46) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6382); };
            c.logic(t46 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6392); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6396); };
            c.w64(6, t48);
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f639a); };
            c.w64(5, t49);
            c.w64(3, c.r64(11));
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let r50 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r50);
            return RegfileExit::jump(0x70007f4cf4);
        }
        36 => {
            let Some(t51) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f63af); };
            c.w64(5, t51);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let r52 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r52);
            return RegfileExit::jump(0x70007f4cf4);
        }
        37 => {
            blk = 51; continue;
        }
        38 => {
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70007f63cf);
        }
        40 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f63d0); };
            c.w32(8, t53);
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f63d7); };
            c.cmp(t54, c.r32(8), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f63dd); };
            c.w64(8, t55);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f63e7); };
            c.cmp(t56, c.r32(9), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let r57 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r57);
            blk = 15; continue;
        }
        43 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f63f4); }
            c.w32(0, 0x1u64);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2108)) else { return RegfileExit::fault(0x70007f63fa); };
            c.w64(3, t58);
            return c.call(0x700086ed74, 0x70007f6406, 0x70007f6401);
        }
        44 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6406); };
            c.w32(8, t59);
            let r60 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r60);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6413); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6417); };
            c.w64(6, t62);
            blk = 15; continue;
        }
        45 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f6420); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f6424); }
            return c.call(0x70007febb4, 0x70007f642d, 0x70007f6428);
        }
        46 => {
            c.w64(11, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6430); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6434); };
            c.w64(6, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6438); };
            c.w64(0, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f643c); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f6440); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f6447); };
            c.w64(3, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f644e); };
            c.w64(8, t69);
            blk = 22; continue;
        }
        47 => {
            c.w32(1, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f645c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70007f6460); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70007f6464); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70007f6468); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70007f646c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70007f6470); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70007f647d, 0x70007f6478);
        }
        48 => {
            c.w64(1, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6480); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f6484); };
            c.w64(11, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f6488); };
            c.w64(12, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f648c); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f6490); };
            c.w64(7, t74);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70007f6494); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f6499); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f64a0); };
            c.w64(3, t76);
            let Some(t77) = c.ld64(0x70007f6322u64) else { return RegfileExit::fault(0x70007f64a7); };
            c.w64(2, t77);
            blk = 31; continue;
        }
        49 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f64b3); };
            c.w64(3, t78);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f64c3, 0x70007f64be);
        }
        50 => {
            return RegfileExit::fault(0x70007f64c3);
        }
        51 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f64c4); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x21c0)) else { return RegfileExit::fault(0x70007f64c5); };
            c.w64(3, t79);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f64d6, 0x70007f64d1);
        }
        52 => {
            return RegfileExit::fault(0x70007f64d6);
        }
        53 => {
            c.w32(1, c.r32(1));
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            c.w64(6, 0xfffffffffffc0000u64);
            let r81 = c.r64(6) & c.r64(1);
            c.w64(6, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f64e6); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f64f4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f64f5); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f6502, 0x70007f64fd);
        }
        55 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6502); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f6504); }
            blk = 33; continue;
        }
        56 => {
            return RegfileExit::jump(0x70007f650c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f6514); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f6518); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f651e); };
            c.w32(8, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007f6522); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f6526); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f652a); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(1).wrapping_mul(4));
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f6547); };
            c.w32(11, t3);
            let r4 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r4);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f6553); }
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f6558); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(12, c.r32(11));
            let r7 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f6570); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f657d); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f6594); };
            c.w32(12, t10);
            c.w64(15, c.r64(11));
            let r11 = c.r64(15) & 0xfffffffffffffffdu64;
            c.w64(15, r11);
            c.logic(r11, 64);
            c.w32(1, c.r32(12));
            let r12 = c.r32(1) & 0x3u64;
            c.w32(1, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f65c9); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f65d0); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f65d8); };
            c.w32(1, t17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xfu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f65e9);
        }
        10 => {
            c.w32(1, c.r32(0));
            let r18 = c.r32(1) & 0x1u64;
            c.w32(1, r18);
            c.logic(r18, 32);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f65ef); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, c.r32(11));
            let r20 = c.r32(11) & 0xfffffffffffffffdu64;
            c.w32(11, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f660a); };
            c.w32(11, t21);
            c.w32(12, c.r32(11));
            let r22 = c.r32(12) & 0x3u64;
            c.w32(12, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r23 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r23);
            c.logic(r23, 64);
            let r24 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r24);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f662f); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f6634); }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(0));
            blk = 18; continue;
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6649); };
            c.w32(11, t25);
            let r26 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r26);
            blk = 18; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6650); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6657); };
            c.w32(12, t29);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 17; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(12, c.r16(12));
            let r30 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r30);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f667a); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6681); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6688); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f668f); };
            c.w32(12, t37);
            let Some(t38) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f6694); };
            c.cmp(t38, c.r32(12), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, c.r64(0));
            let r39 = c.r64(8) | 0x3u64;
            c.w64(8, r39);
            c.logic(r39, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f66a5); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r40 = c.r64(11) & c.r64(2);
            c.w64(11, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f66b9); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f66c4); }
            blk = 52; continue;
        }
        24 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t42) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f66d8); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            let Some(t44) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f66df); };
            c.w32(11, t44);
            let r45 = c.sub(c.r32(11), 0x80cu64, 32);
            c.w32(11, r45);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f66f5); };
            c.w32(11, t46);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t47) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6702); };
            c.w32(12, t47);
            let r48 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r48);
            let Some(t49) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6709); };
            c.w32(12, t49);
            let r50 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r50);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t51) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f6720); };
            c.w32(12, t51);
            let r52 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r52);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 29; continue;
        }
        29 => {
            let r53 = c.r64(12) | 0x3u64;
            c.w64(12, r53);
            c.logic(r53, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70007f6734); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r54 = c.r64(8) & c.r64(2);
            c.w64(8, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6748); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 111; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f6753); }
            blk = 52; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f675f); };
            c.w32(11, t56);
            c.w32(12, c.r32(11));
            let r57 = c.r32(12) & 0x3u64;
            c.w32(12, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r58 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r58);
            c.logic(r58, 64);
            let r59 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r59);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f6784); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f6789); }
            blk = 35; continue;
        }
        35 => {
            let Some(t60) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6790); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f6797); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f679e); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xfu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f67ae);
        }
        37 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x43)) else { return RegfileExit::fault(0x70007f67af); };
            c.cmp(t65, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(7, c.r64(0));
            blk = 42; continue;
        }
        39 => {
            let Some(t66) = c.ld32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f67ba); };
            c.w32(11, t66);
            c.w32(12, c.r32(11));
            let r67 = c.r32(12) & 0x3u64;
            c.w32(12, r67);
            c.logic(r67, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r68 = c.r64(11) | 0x2u64;
            c.w64(11, r68);
            c.logic(r68, 64);
            let r69 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r69);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f67df); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f67e4); }
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, c.r64(7));
            blk = 45; continue;
        }
        44 => {
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f67fa); };
            c.w32(11, t70);
            let r71 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r71);
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6801); };
            c.w32(12, t72);
            let r73 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r73);
            let Some(t74) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6808); };
            c.w32(12, t74);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(12, c.r16(12));
            let r75 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r75);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f6827); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            let Some(t78) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f682e); };
            c.w32(12, t78);
            let r79 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r79);
            let Some(t80) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6835); };
            c.w32(11, t80);
            let r81 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r81);
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f683c); };
            c.w32(12, t82);
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f6841); };
            c.cmp(t83, c.r32(12), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 48; continue;
        }
        48 => {
            let r84 = c.r64(7) | 0x3u64;
            c.w64(7, r84);
            c.logic(r84, 64);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(7)) { return RegfileExit::fault(0x70007f684b); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r85 = c.r64(8) & c.r64(2);
            c.w64(8, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f685f); };
            c.logic(t86 & 0x4u64, 8);
            if c.cc_ne() { blk = 108; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f686a); }
            blk = 52; continue;
        }
        51 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f6873); };
            c.w64(9, t87);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70007f687a); }
            if !c.st32(c.r64(2).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f687f); }
            blk = 52; continue;
        }
        52 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f688e); };
            c.w32(8, t88);
            let r89 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r89);
            let Some(t90) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f6895); };
            c.logic(t90 & 0x2u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 54; continue;
        }
        54 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t91) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f68a9); };
            c.w32(8, t91);
            let r92 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r92);
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f68b0); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f68b7); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t96) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f68be); };
            c.w32(11, t96);
            let r97 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r97);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            return RegfileExit::fault(0x70007f68d1);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70007f68d2); };
            c.cmp(t98, c.r32(11), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r64(9), 0xb0u64, 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            return RegfileExit::fault(0x70007f68e4);
        }
        60 => {
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70007f68e5); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t100) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f68ee); };
            c.cmp(t100, 0x834u64, 16);
            if c.cc_ne() { blk = 95; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t101) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f68fb); };
            c.w32(12, t101);
            c.w64(15, c.r64(12));
            let r102 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r102);
            let r103 = c.r32(15) & 0x3fu64;
            c.w32(15, r103);
            c.logic(r103, 32);
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_g() { blk = 95; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t104) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6915); };
            c.w32(15, t104);
            let r105 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r105);
            let Some(t106) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f691c); };
            c.w32(2, t106);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 64; continue;
        }
        64 => {
            let r107 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r107);
            blk = 67; continue;
        }
        65 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f692d); };
            c.w32(8, t108);
            let Some(t109) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6931); };
            c.w32(15, t109);
            let r110 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r110);
            let Some(t111) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6938); };
            c.cmp(t111, c.r32(8), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 66; continue;
        }
        66 => {
            let r112 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r112);
            c.w64(2, c.r64(8));
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f694d); };
            c.w64(15, t113);
            blk = 69; continue;
        }
        69 => {
            let Some(t114) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6954); };
            c.w32(8, t114);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70007f6958); }
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70007f695c); };
            c.cmp(t115, c.r32(8), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 101; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f6979); };
            c.w64(9, t116);
            let Some(t117) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f6980); };
            c.w64(11, t117);
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f698f); };
            c.w64(1, t118);
            let Some(t119) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f6996); };
            c.cmp(t119, c.r64(3), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 97; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70007f69ac); }
            let r120 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r120);
            blk = 74; continue;
        }
        74 => {
            c.w32(12, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f69b7); };
            c.w64(3, t121);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70007f69be); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f69c2); }
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(3, c.r64(12));
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f69d7); };
            c.w64(7, t122);
            blk = 76; continue;
        }
        76 => {
            let r123 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r123);
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(7)) { return RegfileExit::fault(0x70007f69e2); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(8, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 87; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(3, 0xfff7fffffff7ffffu64);
            blk = 80; continue;
        }
        79 => {
            let r124 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f6a0e); };
            c.w64(15, t125);
            blk = 80; continue;
        }
        80 => {
            let r126 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r126);
            let Some(t127) = c.ld64(c.r64(8).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70007f6a16); };
            c.cmp(t127, c.r64(3), 64);
            if c.cc_e() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(15))) { return RegfileExit::fault(0x70007f6a1c); }
            let Some(t128) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f6a22); };
            c.w64(7, t128);
            c.w64(6, c.r64(7).wrapping_add(0xc));
            c.w64(0, c.r64(12).wrapping_sub(0x4));
            let Some(t129) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f6a2e); };
            c.cmp(t129, c.r64(6), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(9), c.r64(6)) { return RegfileExit::fault(0x70007f6a37); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 83; continue;
        }
        83 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f6a3e); };
            c.w64(7, t130);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007f6a45); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f6a48); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(0)), c.r32(6)) { return RegfileExit::fault(0x70007f6a4d); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r131 = c.r64(7) & c.r64(11);
            c.w64(7, r131);
            c.logic(r131, 64);
            let Some(t132) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6a61); };
            c.logic(t132 & 0x4u64, 8);
            if c.cc_ne() { blk = 105; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 79; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6a71); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6a75); };
            c.w64(6, t134);
            blk = 87; continue;
        }
        87 => {
            let Some(t135) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f6a79); };
            c.w64(5, t135);
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(2));
            let r136 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r136);
            return RegfileExit::jump(0x70007f4cf4);
        }
        88 => {
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f6a91); };
            c.w64(5, t137);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(2));
            c.w64(3, c.r64(15));
            let r138 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(0x70007f4cf4);
        }
        89 => {
            blk = 103; continue;
        }
        90 => {
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            return RegfileExit::fault(0x70007f6ab4);
        }
        92 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f6ab5); };
            c.w32(8, t139);
            let Some(t140) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f6abc); };
            c.cmp(t140, c.r32(8), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f6ac2); };
            c.w64(8, t141);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6acc); };
            c.cmp(t142, c.r32(9), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let r143 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r143);
            blk = 67; continue;
        }
        95 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f6ad9); }
            c.w32(0, 0x1u64);
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2108)) else { return RegfileExit::fault(0x70007f6adf); };
            c.w64(3, t144);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70007f6aee, 0x70007f6ae9);
        }
        96 => {
            let Some(t145) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6aee); };
            c.w32(8, t145);
            let r146 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r146);
            c.w64(15, c.r64(0));
            c.w64(2, c.r64(8));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6afb); };
            c.w64(0, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6aff); };
            c.w64(6, t148);
            blk = 67; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70007f6b08); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f6b0c); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70007f6b18, 0x70007f6b13);
        }
        98 => {
            c.w64(11, c.r64(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6b1b); };
            c.w64(0, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6b1f); };
            c.w64(6, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6b23); };
            c.w64(2, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f6b27); };
            c.w64(15, t152);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f6b2b); };
            c.w64(9, t153);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f6b32); };
            c.w64(1, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f6b39); };
            c.w64(8, t155);
            blk = 74; continue;
        }
        99 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70007f6b42); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70007f6b4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70007f6b4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70007f6b53); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70007f6b57); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70007f6b5b); }
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x70007f6b68, 0x70007f6b63);
        }
        100 => {
            c.w64(6, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6b6b); };
            c.w64(2, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f6b6f); };
            c.w64(11, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f6b73); };
            c.w64(12, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f6b77); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f6b7b); };
            c.w64(0, t160);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70007f6b7f); }
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f6b84); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f6b8b); };
            c.w64(1, t162);
            let Some(t163) = c.ld64(0x70007f6a00u64) else { return RegfileExit::fault(0x70007f6b92); };
            c.w64(3, t163);
            blk = 83; continue;
        }
        101 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f6b9e); };
            c.w64(3, t164);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f6bae, 0x70007f6ba9);
        }
        102 => {
            return RegfileExit::fault(0x70007f6bae);
        }
        103 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f6baf); }
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x21c0)) else { return RegfileExit::fault(0x70007f6bb0); };
            c.w64(3, t165);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f6bc1, 0x70007f6bbc);
        }
        104 => {
            return RegfileExit::fault(0x70007f6bc1);
        }
        105 => {
            c.w32(6, c.r32(6));
            let r166 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r166);
            c.w64(7, 0xfffffffffffc0000u64);
            let r167 = c.r64(7) & c.r64(6);
            c.w64(7, r167);
            c.logic(r167, 64);
            let Some(t168) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6bd1); };
            c.logic(t168 & 0x2u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f6bdf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6be0); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f6bed, 0x70007f6be8);
        }
        107 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6bed); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f6bef); }
            blk = 85; continue;
        }
        108 => {
            c.w32(7, c.r32(7));
            let r169 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r169);
            c.w64(8, 0xfffffffffffc0000u64);
            let r170 = c.r64(8) & c.r64(7);
            c.w64(8, r170);
            c.logic(r170, 64);
            let Some(t171) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6c04); };
            c.logic(t171 & 0x2u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f6c13); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6c14); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007f6c21, 0x70007f6c1c);
        }
        110 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6c21); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f6c23); }
            blk = 50; continue;
        }
        111 => {
            c.w32(12, c.r32(12));
            let r172 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r172);
            c.w64(8, 0xfffffffffffc0000u64);
            let r173 = c.r64(8) & c.r64(12);
            c.w64(8, r173);
            c.logic(r173, 64);
            let Some(t174) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6c39); };
            c.logic(t174 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f6c48); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6c49); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007f6c56, 0x70007f6c51);
        }
        113 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6c56); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f6c58); }
            blk = 31; continue;
        }
        114 => {
            c.w32(8, c.r32(8));
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            c.w64(11, 0xfffffffffffc0000u64);
            let r176 = c.r64(11) & c.r64(8);
            c.w64(11, r176);
            c.logic(r176, 64);
            let Some(t177) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f6c6e); };
            c.logic(t177 & 0x2u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f6c7d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6c7e); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f6c8b, 0x70007f6c86);
        }
        116 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f6c8b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f6c8d); }
            blk = 23; continue;
        }
        117 => {
            return RegfileExit::jump(0x70007f6c94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6cb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6cb4); };
            c.w64(8, t0);
            let r1 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r1);
            if c.cc_le() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(10, c.r64(4));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f6cc4); };
            let r3 = c.sub(c.r64(10), t2, 64);
            c.w64(10, r3);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(8), 64);
            if c.cc_le() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(4));
            c.w64(10, c.r64(8).wrapping_mul(8));
            blk = 4; continue;
        }
        3 => {
            let r4 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r4);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f6ce8); }
            let r5 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r5);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 3; continue; }
            blk = 5; continue;
        }
        5 => {
            let r6 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r6);
            c.w64(12, c.r64(0).wrapping_add(0x2));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f6d0b); };
            c.w64(10, t7);
            if !c.st64(c.r64(4).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f6d0f); }
            let r8 = c.inc(c.r64(15), 64);
            c.w64(15, r8);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_l() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            let r9 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r9);
            c.w64(1, c.r64(1).wrapping_mul(8).wrapping_add(0x18));
            let r10 = c.add(c.r64(1), c.r64(5), 64);
            c.w64(1, r10);
            blk = 9; continue;
        }
        9 => {
            let r11 = c.dec(c.r64(8), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f6d30); };
            c.w64(10, t12);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f6d34); }
            if c.cc_ne() { blk = 9; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f6d3e); };
            c.w64(3, t13);
            return RegfileExit::jump(0x700086ed74);
        }
        12 => {
            return RegfileExit::jump(0x70007f2274);
        }
        13 => {
            return RegfileExit::fault(0x70007f6d4f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6d54); };
            c.w64(8, t0);
            let r1 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r1);
            if c.cc_le() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(10, c.r64(4));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f6d64); };
            let r3 = c.sub(c.r64(10), t2, 64);
            c.w64(10, r3);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(8), 64);
            if c.cc_le() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(4));
            c.w64(10, c.r64(8).wrapping_mul(8));
            blk = 4; continue;
        }
        3 => {
            let r4 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r4);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f6d88); }
            let r5 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r5);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 3; continue; }
            blk = 5; continue;
        }
        5 => {
            let r6 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r6);
            c.w64(12, c.r64(0).wrapping_add(0x2));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f6dab); };
            c.w64(10, t7);
            if !c.st64(c.r64(4).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f6daf); }
            let r8 = c.inc(c.r64(15), 64);
            c.w64(15, r8);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_l() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            let r9 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r9);
            c.w64(1, c.r64(1).wrapping_mul(8).wrapping_add(0x18));
            let r10 = c.add(c.r64(1), c.r64(5), 64);
            c.w64(1, r10);
            blk = 9; continue;
        }
        9 => {
            let r11 = c.dec(c.r64(8), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f6dd0); };
            c.w64(10, t12);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f6dd4); }
            if c.cc_ne() { blk = 9; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f6dde); };
            c.w64(3, t13);
            return RegfileExit::jump(0x700086ed74);
        }
        12 => {
            return RegfileExit::jump(0x70007f1ed4);
        }
        13 => {
            return RegfileExit::fault(0x70007f6def);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6df4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f6df4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f6df8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t1) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f6e08); };
            c.w64(8, t1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6e0d); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f6e14); };
            c.logic(t4 & 0x20u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70007f6e1b); };
            c.w32(9, t5);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f6e2b); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6e32); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6e39); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6e40); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f6e44); };
            c.w64(5, t13);
            c.w64(7, c.r64(8));
            let r14 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r14);
            return RegfileExit::jump(0x70007fe8d4);
        }
        3 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6e54); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007f6e56); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70007f6e5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70007f6e5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007f6e62); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x20c8)) else { return RegfileExit::fault(0x70007f6e66); };
            c.w64(3, t15);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f6e77, 0x70007f6e72);
        }
        4 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f6e77); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f6e7b); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f6e7f); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f6e83); };
            c.w64(2, t19);
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70007f6e8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6e94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f6e94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f6e98); }
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t0) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f6ea4); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f6ea9); };
            c.w32(9, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f6ead); };
            c.cmp(t2, c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6eba); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            c.w64(12, c.r64(11));
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f6ec4); };
            c.w32(12, t5);
            let r6 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r6);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6ed6); };
            c.w32(15, t7);
            let r8 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r8);
            let Some(t9) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6ede); };
            c.w32(15, t9);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(15), 0xacu64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 9; continue;
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6ef9); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6f01); };
            c.w32(12, t12);
            let r13 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r13);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 9; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6f11); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6f1e); };
            c.w32(15, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70007f6f23); };
            c.cmp(t17, c.r32(15), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f6f2c); };
            c.w32(12, t18);
            let r19 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f6f34); };
            c.cmp(t20, c.r32(12), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 8; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f6f3f); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6f46); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f6f4d); };
            c.w32(3, t25);
            let r26 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r26);
            let Some(t27) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f6f54); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f6f58); };
            c.w64(5, t28);
            c.w64(7, c.r64(8));
            let r29 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r29);
            return RegfileExit::jump(0x70007fe8d4);
        }
        14 => {
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f6f68); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f6f6f); };
            c.cmp(t32, 0xbeu64, 16);
            if c.cc_e() { blk = 1; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(8, 0x72u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f6f82); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007f6f84); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f6f95, 0x70007f6f90);
        }
        16 => {
            return RegfileExit::fault(0x70007f6f95);
        }
        17 => {
            return RegfileExit::jump(0x70007f6f98);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f6fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f6fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f6fb8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t1) = c.ld64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70007f6fc8); };
            c.w64(8, t1);
            let Some(t2) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f6fcd); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f6fd4); };
            c.logic(t4 & 0x20u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70007f6fdb); };
            c.w32(9, t5);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f6fef); };
            c.w32(9, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f6ff3); };
            c.cmp(t7, c.r32(9), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7000); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            c.w64(12, c.r64(11));
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f700a); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f701c); };
            c.w32(15, t12);
            let r13 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r13);
            let Some(t14) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7024); };
            c.w32(15, t14);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(15), 0xacu64, 16);
            if c.cc_e() { blk = 4; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f703f); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f7047); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 11; continue;
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f705b); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7068); };
            c.w32(15, t21);
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70007f706d); };
            c.cmp(t22, c.r32(15), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f7076); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f707e); };
            c.cmp(t25, c.r32(12), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 10; continue;
        }
        15 => {
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f7089); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7090); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7097); };
            c.w32(3, t30);
            let r31 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r31);
            let Some(t32) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f709e); };
            c.w64(2, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f70a2); };
            c.w64(5, t33);
            c.w64(7, c.r64(8));
            let r34 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(0x70007fe8d4);
        }
        16 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f70b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70007f70b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f70b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70007f70bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70007f70c0); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x20c8)) else { return RegfileExit::fault(0x70007f70c4); };
            c.w64(3, t35);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f70d5, 0x70007f70d0);
        }
        17 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f70d5); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f70d9); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f70dd); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f70e1); };
            c.w64(8, t39);
            blk = 2; continue;
        }
        18 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f70ea); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f70f1); };
            c.cmp(t42, 0xbeu64, 16);
            if c.cc_e() { blk = 3; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(8, 0x72u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7104); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007f7106); };
            c.w64(3, t43);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f7117, 0x70007f7112);
        }
        20 => {
            return RegfileExit::fault(0x70007f7117);
        }
        21 => {
            return RegfileExit::jump(0x70007f711c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7134(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f7134); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f713b); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f7141); };
            c.logic(t3 & 0x1000000u64, 32);
            if c.cc_ne() { return RegfileExit::jump(0x70007f9ab4); }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::jump(0x70007f98b4);
        }
        2 => {
            return RegfileExit::fault(0x70007f7153);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7174); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f717a); };
            c.w64(3, (t2 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.logic(c.r32(3) & c.r32(3), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            if let Some(sr) = c.shl(c.r64(3), 0x3u64, 64) { c.w64(3, sr); }
            c.w64(10, c.r64(4));
            let r3 = c.sub(c.r64(10), c.r64(3), 64);
            c.w64(10, r3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f7193); };
            c.cmp(c.r64(10), t4, 64);
            if c.cc_ae() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f7199); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f719d); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f71a1); };
            c.w64(3, t5);
            return c.call(0x700086ed74, 0x70007f71ad, 0x70007f71a8);
        }
        3 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f71ad); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007f71af); }
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f71b1); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f71b7); };
            c.w64(3, (t8 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r9 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r9);
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f71c1); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007f71c9); }
            let r12 = c.dec(c.r32(3), 32);
            c.w32(3, r12);
            if c.cc_g() { blk = 4; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007f71cf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f71d1); }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f71d8); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f71de); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            return RegfileExit::jump(0x70007f7214);
        }
        9 => {
            return RegfileExit::fault(0x70007f71e9);
        }
        10 => {
            return RegfileExit::fault(0x70007f71ea);
        }
        11 => {
            return RegfileExit::fault(0x70007f71eb);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f71f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f71f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f71f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f71fa); }
            c.w32(0, 0x1u64);
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x21d0)) else { return RegfileExit::fault(0x70007f7200); };
            c.w64(3, t0);
            return c.call(0x700086ed74, 0x70007f720c, 0x70007f7207);
        }
        1 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f720f); }
            return RegfileExit::jump(0x70007f7210);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f721e); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f7224); };
            c.logic(t2 & 0x40u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f722e); };
            c.w32(10, t3);
            c.w32(10, c.r64(10).wrapping_sub(0x80c));
            c.cmp(c.r32(10), 0xeu64, 32);
            if c.cc_be() { return RegfileExit::jump(0x70007f7134); }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7244); };
            c.cmp(t4, 0x80bu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007f7174); }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7250); };
            c.cmp(t5, 0xbcu64, 16);
            if c.cc_e() { return RegfileExit::jump(0x70007fa074); }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(7)) { return RegfileExit::fault(0x70007f725c); }
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7261); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7267); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xdf)) else { return RegfileExit::fault(0x70007f726d); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            return RegfileExit::jump(0x70007f1ed4);
        }
        6 => {
            return RegfileExit::jump(0x70007f71f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f7297); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(1), 64);
            if c.cc_le() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(1).wrapping_mul(8));
            blk = 3; continue;
        }
        2 => {
            let r2 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r2);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f72bb); }
            let r3 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r3);
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            let r4 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r4);
            c.w64(9, c.r64(0).wrapping_add(0x2));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f72de); };
            c.w64(10, t5);
            if !c.st64(c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f72e2); }
            let r6 = c.inc(c.r64(12), 64);
            c.w64(12, r6);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_l() { blk = 5; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f72fa); };
            c.w32(12, t7);
            let r8 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f7302); };
            c.cmp(c.r32(12), t9, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f730b); };
            c.w64(12, t10);
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(12)) { return RegfileExit::fault(0x70007f7312); }
            let r11 = c.inc(c.r32(9), 32);
            c.w32(9, r11);
            blk = 8; continue;
        }
        12 => {
            let r12 = c.add(c.r64(0), c.r64(9), 64);
            c.w64(0, r12);
            return RegfileExit::jump(0x70007f7214);
        }
        13 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f7325); };
            c.w64(3, t13);
            return RegfileExit::jump(0x700086ed74);
        }
        14 => {
            return RegfileExit::fault(0x70007f7331);
        }
        15 => {
            return RegfileExit::fault(0x70007f7332);
        }
        16 => {
            return RegfileExit::fault(0x70007f7333);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f7354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f7358); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70007f735e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007f7362); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70007f7366); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f736a); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7377); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f737e); };
            c.cmp(t3, 0x834u64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f738b); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7392); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f7399); };
            c.w32(11, t8);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70007f73a9);
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f73aa); };
            c.w32(9, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f73b1); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f73bb); };
            c.w64(9, t11);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f73c5); };
            c.cmp(t12, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f73cf); };
            c.w64(9, t13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f73d6); };
            c.cmp(t14, c.r32(11), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f73e0); };
            c.w32(8, t15);
            let r16 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r16);
            let Some(t17) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f73e9); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let r19 = c.r32(8) & 0x3fu64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 30; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 30; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 31; continue;
        }
        11 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7413); };
            c.w32(8, t20);
            let r21 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007f741a); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 42; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f7438); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f743f); };
            c.w64(15, t23);
            c.w64(3, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f744f); };
            c.w64(6, t24);
            let Some(t25) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70007f7456); };
            c.cmp(t25, c.r64(1), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 38; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(12), c.r64(1)) { return RegfileExit::fault(0x70007f746c); }
            let r26 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r26);
            blk = 16; continue;
        }
        16 => {
            c.w32(3, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f7478); };
            c.w64(1, t27);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f747f); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70007f7483); }
            c.w64(3, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(1, c.r64(3));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f7498); };
            c.w64(2, t28);
            blk = 18; continue;
        }
        18 => {
            let r29 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r29);
            if !c.st32(c.r64(1).wrapping_add(c.r64(15)), c.r32(2)) { return RegfileExit::fault(0x70007f74a3); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(11).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(2, 0xfff7fffffff7ffffu64);
            blk = 22; continue;
        }
        21 => {
            let r30 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f74cf); };
            c.w64(9, t31);
            blk = 22; continue;
        }
        22 => {
            let r32 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r32);
            let Some(t33) = c.ld64(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007f74d7); };
            c.cmp(t33, c.r64(2), 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x70007f74dd); }
            let Some(t34) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f74e3); };
            c.w64(1, t34);
            c.w64(7, c.r64(1).wrapping_add(0xc));
            c.w64(9, c.r64(3).wrapping_sub(0x4));
            let Some(t35) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70007f74ef); };
            c.cmp(t35, c.r64(7), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(12), c.r64(7)) { return RegfileExit::fault(0x70007f74f8); }
            let r36 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r36);
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f7500); };
            c.w64(7, t37);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007f7507); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f750a); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x70007f750f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r38 = c.r64(7) & c.r64(15);
            c.w64(7, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7522); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f7532); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7536); };
            c.w64(0, t41);
            blk = 29; continue;
        }
        29 => {
            let Some(t42) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f753a); };
            c.w64(5, t42);
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007f753e); };
            c.w64(6, t43);
            let Some(t44) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007f7543); };
            c.w64(2, t44);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(8));
            let r45 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r45);
            return RegfileExit::jump(0x70007f7294);
        }
        30 => {
            let Some(t46) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7557); };
            c.w32(8, t46);
            let r47 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r47);
            let Some(t48) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f755e); };
            c.w64(5, t48);
            c.w64(3, c.r64(9));
            c.w64(1, c.r64(8));
            let r49 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r49);
            return RegfileExit::jump(0x70007f7294);
        }
        31 => {
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f7571); };
            c.cmp(t50, c.r32(3), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f7580); };
            c.cmp(t51, c.r32(3), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70007f758f); }
            let r52 = c.r32(8) | c.r32(9);
            c.w32(8, r52);
            c.logic(r52, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(0, c.r64(3));
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f759f); };
            c.w64(3, t53);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70007f75ae, 0x70007f75a9);
        }
        33 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f75b6); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            let Some(t56) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f75bd); };
            c.logic(t56 & 0x2u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(3, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f75cb); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f75cf); };
            c.w64(6, t58);
            return c.call(0x7000845bd4, 0x70007f75d8, 0x70007f75d3);
        }
        36 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f75d8); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f75df); };
            c.w32(8, t61);
            let r62 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r62);
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f75e8); };
            c.w32(9, t63);
            let r64 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r64);
            let r65 = c.r32(8) & 0x3fu64;
            c.w32(8, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(3, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7600); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7604); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f7608); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f760c); };
            c.w64(2, t69);
            blk = 11; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f7615); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70007f7619); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70007f7625, 0x70007f7620);
        }
        39 => {
            c.w64(15, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f7628); };
            c.w64(7, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f762c); };
            c.w64(2, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7630); };
            c.w64(0, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7634); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f7638); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f763c); };
            c.w64(12, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f7643); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f764a); };
            c.w64(11, t77);
            blk = 16; continue;
        }
        40 => {
            c.w32(1, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f7658); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70007f765c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70007f7660); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70007f7664); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70007f7668); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x70007f766c); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70007f7679, 0x70007f7674);
        }
        41 => {
            c.w64(1, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f767c); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f7680); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f7684); };
            c.w64(3, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f7688); };
            c.w64(11, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70007f768c); };
            c.w64(9, t82);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x70007f7690); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f7695); };
            c.w64(12, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f769c); };
            c.w64(6, t84);
            let Some(t85) = c.ld64(0x70007f74c1u64) else { return RegfileExit::fault(0x70007f76a3); };
            c.w64(2, t85);
            blk = 25; continue;
        }
        42 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f76af); };
            c.w64(3, t86);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f76bf, 0x70007f76ba);
        }
        43 => {
            return RegfileExit::fault(0x70007f76bf);
        }
        44 => {
            c.w64(3, c.r64(0));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f76c3); };
            c.w64(0, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f76c7); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f76cb); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f76cf); };
            c.w64(2, t90);
            c.w64(8, c.r64(0));
            blk = 30; continue;
        }
        45 => {
            c.w32(8, 0x90u64);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f76e1); };
            c.w64(3, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f76e5); };
            c.w64(6, t92);
            blk = 47; continue;
        }
        46 => {
            c.w32(8, 0xd0u64);
            blk = 47; continue;
        }
        47 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f76f1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f76f3); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f76f4); };
            c.w64(3, t93);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007f7705, 0x70007f7700);
        }
        48 => {
            return RegfileExit::fault(0x70007f7705);
        }
        49 => {
            c.w32(1, c.r32(1));
            let r94 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r94);
            c.w64(7, 0xfffffffffffc0000u64);
            let r95 = c.r64(7) & c.r64(1);
            c.w64(7, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7715); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7723); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7724); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f7731, 0x70007f772c);
        }
        51 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f7731); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f7733); }
            blk = 27; continue;
        }
        52 => {
            return RegfileExit::jump(0x70007f773c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f7754); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f7758); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f775e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f7762); };
            c.w64(9, t2);
            let Some(t3) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7766); };
            c.w64(8, t3);
            c.w64(11, c.r64(2).wrapping_mul(4));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f7772); };
            c.w32(12, t4);
            let r5 = c.add(c.r32(12), 0x8u64, 32);
            c.w32(12, r5);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x23), c.r32(12)) { return RegfileExit::fault(0x70007f777e); }
            c.w64(12, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f778b); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            c.w32(2, c.r32(15));
            let r8 = c.r32(2) & 0xfffffffffffffffdu64;
            c.w32(2, r8);
            c.logic(r8, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f7799); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70007f779d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70007f77a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70007f77a5); }
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f77b1); };
            c.cmp(t9, c.r32(15), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f77be); };
            c.cmp(t10, c.r32(15), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, c.r32(15));
            let r11 = c.r32(2) & 0x3u64;
            c.w32(2, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f77d8); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f77df); };
            c.cmp(t14, 0x5au64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f77ee); };
            c.w32(12, t15);
            let r16 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f77f5); };
            c.w32(12, t17);
            let r18 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f77fd); };
            c.w32(15, t19);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70007f780e);
        }
        9 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f780f); };
            c.cmp(t20, c.r32(3), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 25; continue;
        }
        11 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7830); };
            c.w32(15, t21);
            let r22 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r22);
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7837); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f783e); };
            c.w32(2, t25);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2bu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70007f784e);
        }
        15 => {
            let Some(t26) = c.ld32(c.r64(15).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f784f); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(15, c.r64(3));
            blk = 19; continue;
        }
        18 => {
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f786a); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            blk = 19; continue;
        }
        19 => {
            let Some(t29) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7871); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7878); };
            c.w32(2, t31);
            c.cmp(c.r16(2), 0x80bu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, c.r16(2));
            let r32 = c.sub(c.r32(2), 0x80cu64, 32);
            c.w32(2, r32);
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t33) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f7891); };
            c.w32(15, t33);
            let r34 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r34);
            let Some(t35) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7898); };
            c.w32(2, t35);
            let r36 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r36);
            let Some(t37) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f789f); };
            c.w32(15, t37);
            let r38 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r38);
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f78a6); };
            c.w32(2, t39);
            let Some(t40) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f78a9); };
            c.cmp(t40, c.r32(2), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, c.r64(3));
            let r41 = c.r64(11) | 0x3u64;
            c.w64(11, r41);
            c.logic(r41, 64);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(11)) { return RegfileExit::fault(0x70007f78b6); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r42 = c.r64(15) & c.r64(8);
            c.w64(15, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f78ca); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f78d5); }
            blk = 33; continue;
        }
        25 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f78e2); };
            c.w64(12, t44);
            if !c.st32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x1f), c.r32(12)) { return RegfileExit::fault(0x70007f78e9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f78ee); }
            blk = 33; continue;
        }
        26 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70007f78fb); };
            c.w64(11, t45);
            let Some(t46) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f7902); };
            c.w64(15, t46);
            c.w64(2, c.r64(15).wrapping_add(0x1c));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70007f7909); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70007f7910); };
            c.cmp(t48, c.r64(2), 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70007f7919); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70007f7920); };
            c.w64(15, t49);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70007f7927); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70007f792e); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70007f7932); }
            if !c.st32(c.r64(11).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70007f7936); }
            if !c.st32(c.r64(11).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70007f793e); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70007f7946); };
            c.w64(15, t50);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70007f794d); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70007f7951); };
            c.w64(15, t51);
            let Some(t52) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f7958); };
            c.w64(2, t52);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70007f795b); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r53 = c.r64(6) & c.r64(11);
            c.w64(6, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f796e); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(15), c.r64(11)) { return RegfileExit::fault(0x70007f7978); }
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(11)) { return RegfileExit::fault(0x70007f797b); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r55 = c.r64(15) & c.r64(8);
            c.w64(15, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f798f); };
            c.logic(t56 & 0x4u64, 8);
            if c.cc_ne() { blk = 87; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f799a); }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t57) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f79ac); };
            c.w32(8, t57);
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            let Some(t59) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f79b3); };
            c.cmp(t59, 0x834u64, 16);
            if c.cc_ne() { blk = 66; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f79c0); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f79c7); };
            c.w32(11, t62);
            let r63 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r63);
            let Some(t64) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f79ce); };
            c.w32(12, t64);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f79de);
        }
        37 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f79df); };
            c.w32(11, t65);
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f79e6); };
            c.cmp(t66, c.r32(11), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f79f0); };
            c.w64(11, t67);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f79fa); };
            c.cmp(t68, c.r32(12), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f7a04); };
            c.w64(11, t69);
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7a0b); };
            c.cmp(t70, c.r32(12), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t71) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f7a15); };
            c.w32(8, t71);
            let r72 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r72);
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7a1e); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            let r75 = c.r32(8) & 0x3fu64;
            c.w32(8, r75);
            c.logic(r75, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 66; continue;
        }
        44 => {
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7a48); };
            c.w32(8, t76);
            let r77 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r77);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70007f7a4f); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 77; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f7a6d); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f7a74); };
            c.w64(0, t79);
            c.w64(2, c.r64(12).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f7a83); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f7a8a); };
            c.cmp(t81, c.r64(6), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 73; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70007f7aa0); }
            let r82 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r82);
            blk = 49; continue;
        }
        49 => {
            c.w32(2, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f7aab); };
            c.w64(6, t83);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f7ab2); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70007f7ab5); }
            c.w64(2, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(6, c.r64(2));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f7ac9); };
            c.w64(9, t84);
            blk = 51; continue;
        }
        51 => {
            let r85 = c.sub(c.r64(6), 0x4u64, 64);
            c.w64(6, r85);
            if !c.st32(c.r64(6).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x70007f7ad4); }
            c.cmp(c.r64(6), 0x7u64, 64);
            if c.cc_ne() { blk = 51; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(9, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 62; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            blk = 55; continue;
        }
        54 => {
            let r86 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7b00); };
            c.w64(11, t87);
            blk = 55; continue;
        }
        55 => {
            let r88 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r88);
            let Some(t89) = c.ld64(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70007f7b08); };
            c.cmp(t89, c.r64(12), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11))) { return RegfileExit::fault(0x70007f7b0e); }
            let Some(t90) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f7b14); };
            c.w64(6, t90);
            c.w64(3, c.r64(6).wrapping_add(0xc));
            c.w64(1, c.r64(2).wrapping_sub(0x4));
            let Some(t91) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f7b1f); };
            c.cmp(t91, c.r64(3), 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70007f7b28); }
            c.w64(3, c.r64(6).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f7b2f); };
            c.w64(6, t92);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f7b36); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f7b39); }
            if !c.st32(c.r64(0).wrapping_add(c.r64(1)), c.r32(3)) { return RegfileExit::fault(0x70007f7b3e); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r93 = c.r64(6) & c.r64(0);
            c.w64(6, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7b50); };
            c.logic(t94 & 0x4u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 54; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7b60); };
            c.w64(3, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7b64); };
            c.w64(1, t96);
            blk = 62; continue;
        }
        62 => {
            let Some(t97) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f7b68); };
            c.w64(5, t97);
            let Some(t98) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f7b6c); };
            c.w64(7, t98);
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            let Some(t99) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007f7b7d); };
            c.w64(6, t99);
            let r100 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r100);
            return RegfileExit::jump(0x70007f7294);
        }
        63 => {
            let Some(t101) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7b8b); };
            c.w32(8, t101);
            let r102 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r102);
            let Some(t103) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f7b92); };
            c.w64(5, t103);
            let Some(t104) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f7b96); };
            c.w64(7, t104);
            c.w64(2, c.r64(3));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(3, c.r64(11));
            c.w64(6, c.r64(9));
            let r105 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r105);
            return RegfileExit::jump(0x70007f7294);
        }
        64 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f7bb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f7bbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70007f7bc0); }
            return c.call(0x70007fec74, 0x70007f7bc9, 0x70007f7bc4);
        }
        65 => {
            c.w64(11, c.r64(0));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7bcc); };
            c.w64(7, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7bd0); };
            c.w64(9, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7bd4); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7bd8); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f7bdc); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f7be0); };
            c.w64(12, t111);
            blk = 28; continue;
        }
        66 => {
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f7be9); };
            c.cmp(t112, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f7bf8); };
            c.cmp(t113, c.r32(7), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f7c07); }
            let r114 = c.r32(8) | c.r32(11);
            c.w32(8, r114);
            c.logic(r114, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(6, c.r64(9));
            c.w64(0, c.r64(7));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f7c1a); };
            c.w64(3, t115);
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70007f7c29, 0x70007f7c24);
        }
        68 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t116) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7c31); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f7c38); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(3, c.r64(0));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7c46); };
            c.w64(0, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7c4a); };
            c.w64(6, t120);
            return c.call(0x7000845bd4, 0x70007f7c53, 0x70007f7c4e);
        }
        71 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7c53); };
            c.w32(8, t121);
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            let Some(t123) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f7c5a); };
            c.w32(8, t123);
            let r124 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r124);
            let Some(t125) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7c63); };
            c.w32(11, t125);
            let r126 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r126);
            let r127 = c.r32(8) & 0x3fu64;
            c.w32(8, r127);
            c.logic(r127, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 79; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7c78); };
            c.w64(1, t128);
            c.w64(7, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7c7f); };
            c.w64(9, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7c83); };
            c.w64(3, t130);
            blk = 44; continue;
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f7c8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70007f7c90); }
            return c.call(0x70007febb4, 0x70007f7c99, 0x70007f7c94);
        }
        74 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f7c99); };
            c.w64(12, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7c9d); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7ca1); };
            c.w64(3, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7ca5); };
            c.w64(1, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7ca9); };
            c.w64(11, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f7cad); };
            c.w64(8, t136);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f7cb1); };
            c.w64(15, t137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f7cb8); };
            c.w64(7, t138);
            blk = 49; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70007f7cc4); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f7ccd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70007f7cd1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70007f7cd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70007f7cd9); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x70007f7cdd); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70007f7cea, 0x70007f7ce5);
        }
        76 => {
            c.w64(3, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f7ced); };
            c.w64(8, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f7cf1); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f7cf5); };
            c.w64(2, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f7cf9); };
            c.w64(9, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70007f7cfd); };
            c.w64(1, t143);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x70007f7d01); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f7d06); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f7d0d); };
            c.w64(7, t145);
            let Some(t146) = c.ld64(0x70007f7af2u64) else { return RegfileExit::fault(0x70007f7d14); };
            c.w64(12, t146);
            blk = 58; continue;
        }
        77 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f7d20); };
            c.w64(3, t147);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f7d30, 0x70007f7d2b);
        }
        78 => {
            return RegfileExit::fault(0x70007f7d30);
        }
        79 => {
            c.w64(7, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f7d34); };
            c.w64(1, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7d38); };
            c.w64(9, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f7d3c); };
            c.w64(3, t150);
            blk = 63; continue;
        }
        80 => {
            c.w32(8, 0x90u64);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f7d4b); };
            c.w64(7, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f7d4f); };
            c.w64(9, t152);
            blk = 82; continue;
        }
        81 => {
            c.w32(8, 0xd0u64);
            blk = 82; continue;
        }
        82 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7d5b); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7d5d); }
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f7d5e); };
            c.w64(3, t153);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x70007f7d72, 0x70007f7d6d);
        }
        83 => {
            return RegfileExit::fault(0x70007f7d72);
        }
        84 => {
            c.w32(3, c.r32(3));
            let r154 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r154);
            c.w64(6, 0xfffffffffffc0000u64);
            let r155 = c.r64(6) & c.r64(3);
            c.w64(6, r155);
            c.logic(r155, 64);
            let Some(t156) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7d82); };
            c.logic(t156 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(11, c.r64(0).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7d90); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7d91); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70007f7d9e, 0x70007f7d99);
        }
        86 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f7d9e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f7da0); }
            blk = 60; continue;
        }
        87 => {
            c.w32(11, c.r32(11));
            let r157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r157);
            c.w64(15, 0xfffffffffffc0000u64);
            let r158 = c.r64(15) & c.r64(11);
            c.w64(15, r158);
            c.logic(r158, 64);
            let Some(t159) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7db6); };
            c.logic(t159 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7dc5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7dc6); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f7dd3, 0x70007f7dce);
        }
        89 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f7dd3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f7dd5); }
            blk = 32; continue;
        }
        90 => {
            c.w32(2, c.r32(2));
            let r160 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r160);
            c.w64(6, 0xfffffffffffc0000u64);
            let r161 = c.r64(6) & c.r64(2);
            c.w64(6, r161);
            c.logic(r161, 64);
            let Some(t162) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7dea); };
            c.logic(t162 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(0, c.r64(11).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7df8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7df9); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f7e06, 0x70007f7e01);
        }
        92 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f7e06); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f7e08); }
            blk = 30; continue;
        }
        93 => {
            c.w32(11, c.r32(11));
            let r163 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r163);
            c.w64(15, 0xfffffffffffc0000u64);
            let r164 = c.r64(15) & c.r64(11);
            c.w64(15, r164);
            c.logic(r164, 64);
            let Some(t165) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7e1e); };
            c.logic(t165 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(2, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f7e2d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f7e2e); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70007f7e3b, 0x70007f7e36);
        }
        95 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f7e3b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f7e3d); }
            blk = 24; continue;
        }
        96 => {
            return RegfileExit::jump(0x70007f7e44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f7e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f7e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f7e58); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            c.w64(8, c.r64(2).wrapping_mul(4));
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f7e66); };
            c.w64(9, t1);
            let Some(t2) = c.ld32(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f7e6a); };
            c.w32(11, t2);
            let r3 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r3);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f7e76); }
            c.w64(11, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f7e83); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            c.w32(15, c.r32(12));
            let r6 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r6);
            c.logic(r6, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f7e92); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70007f7e96); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70007f7e9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f7e9e); }
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f7eab); };
            c.cmp(t7, c.r32(12), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f7eb8); };
            c.cmp(t8, c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(15, c.r32(12));
            let r9 = c.r32(15) & 0x3u64;
            c.w32(15, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7ed4); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7edc); };
            c.cmp(t12, 0x5au64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7eec); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7ef3); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f7efa); };
            c.w32(12, t17);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70007f7f0a);
        }
        9 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f7f0b); };
            c.cmp(t18, c.r32(3), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 25; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7f2b); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7f32); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f7f3a); };
            c.w32(15, t23);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70007f7f4b);
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f7f4c); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, c.r64(3));
            blk = 19; continue;
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f7f68); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            blk = 19; continue;
        }
        19 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7f70); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f7f78); };
            c.w32(15, t29);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(15, c.r16(15));
            let r30 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r30);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f7f96); };
            c.w32(12, t31);
            let r32 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r32);
            let Some(t33) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7f9e); };
            c.w32(15, t33);
            let r34 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f7fa5); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7fad); };
            c.w32(15, t37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f7fb1); };
            c.cmp(t38, c.r32(15), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(3));
            let r39 = c.r64(8) | 0x3u64;
            c.w64(8, r39);
            c.logic(r39, 64);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(8)) { return RegfileExit::fault(0x70007f7fbf); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r40 = c.r64(12) & c.r64(9);
            c.w64(12, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f7fd3); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f7fdf); }
            blk = 33; continue;
        }
        25 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f7fec); };
            c.w64(11, t42);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70007f7ff3); }
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f7ff8); }
            blk = 33; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70007f8005); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007f800c); };
            c.w64(12, t44);
            c.w64(15, c.r64(12).wrapping_add(0x1c));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70007f8014); };
            c.w64(2, t45);
            let Some(t46) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70007f801b); };
            c.cmp(t46, c.r64(15), 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70007f8024); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70007f802c); };
            c.w64(12, t47);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70007f8033); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f803a); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70007f803e); }
            if !c.st32(c.r64(8).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70007f8042); }
            if !c.st32(c.r64(8).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70007f804a); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70007f8052); };
            c.w64(12, t48);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70007f8059); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70007f805d); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f8064); };
            c.w64(15, t50);
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70007f8068); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r51 = c.r64(2) & c.r64(8);
            c.w64(2, r51);
            c.logic(r51, 64);
            let Some(t52) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f807c); };
            c.logic(t52 & 0x4u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x70007f8086); }
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(8)) { return RegfileExit::fault(0x70007f808a); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r53 = c.r64(12) & c.r64(9);
            c.w64(12, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f809e); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 87; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f80aa); }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t55) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f80bc); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            let Some(t57) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f80c3); };
            c.cmp(t57, 0x834u64, 16);
            if c.cc_ne() { blk = 66; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t58) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f80d0); };
            c.w32(9, t58);
            let r59 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r59);
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f80d7); };
            c.w32(9, t60);
            let r61 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r61);
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f80de); };
            c.w32(11, t62);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4cu64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70007f80ee);
        }
        37 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f80ef); };
            c.w32(9, t63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f80f6); };
            c.cmp(t64, c.r32(9), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f8100); };
            c.w64(9, t65);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f810a); };
            c.cmp(t66, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70007f8114); };
            c.w64(9, t67);
            let Some(t68) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f811b); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t69) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f8125); };
            c.w32(8, t69);
            let r70 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r70);
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f812e); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let r73 = c.r32(8) & 0x3fu64;
            c.w32(8, r73);
            c.logic(r73, 32);
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_le() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_le() { blk = 63; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 66; continue;
        }
        44 => {
            let Some(t74) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f8158); };
            c.w32(8, t74);
            let r75 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r75);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70007f815f); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 77; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f817d); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f8184); };
            c.w64(15, t77);
            c.w64(2, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8194); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f819b); };
            c.cmp(t79, c.r64(0), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 73; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70007f81b1); }
            let r80 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r80);
            blk = 49; continue;
        }
        49 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f81bd); };
            c.w64(2, t81);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70007f81c4); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70007f81c8); }
            c.w64(0, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(2, c.r64(0));
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f81dd); };
            c.w64(6, t82);
            blk = 51; continue;
        }
        51 => {
            let r83 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r83);
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(6)) { return RegfileExit::fault(0x70007f81e8); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 51; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(11, c.r64(11).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 62; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(2, 0xfff7fffffff7ffffu64);
            blk = 55; continue;
        }
        54 => {
            let r84 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8214); };
            c.w64(9, t85);
            blk = 55; continue;
        }
        55 => {
            let r86 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r86);
            let Some(t87) = c.ld64(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70007f821c); };
            c.cmp(t87, c.r64(2), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x70007f8222); }
            let Some(t88) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f8228); };
            c.w64(6, t88);
            c.w64(3, c.r64(6).wrapping_add(0xc));
            c.w64(1, c.r64(0).wrapping_sub(0x4));
            let Some(t89) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f8234); };
            c.cmp(t89, c.r64(3), 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70007f823d); }
            c.w64(3, c.r64(6).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f8245); };
            c.w64(6, t90);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f824c); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f824f); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(1)), c.r32(3)) { return RegfileExit::fault(0x70007f8254); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r91 = c.r64(6) & c.r64(15);
            c.w64(6, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8267); };
            c.logic(t92 & 0x4u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 54; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8277); };
            c.w64(3, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f827b); };
            c.w64(1, t94);
            blk = 62; continue;
        }
        62 => {
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f827f); };
            c.w64(5, t95);
            let Some(t96) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f8283); };
            c.w64(7, t96);
            c.w64(2, c.r64(3));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(3, c.r64(15));
            let Some(t97) = c.ld64(c.r64(4).wrapping_add(0x48)) else { return RegfileExit::fault(0x70007f8294); };
            c.w64(6, t97);
            let Some(t98) = c.ld64(c.r64(4).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007f8299); };
            c.w64(10, t98);
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(10)) { return RegfileExit::fault(0x70007f829e); }
            let r99 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r99);
            return RegfileExit::jump(0x70007f7294);
        }
        63 => {
            let Some(t100) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f82ac); };
            c.w32(8, t100);
            let r101 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r101);
            let Some(t102) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f82b3); };
            c.w64(5, t102);
            let Some(t103) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f82b7); };
            c.w64(7, t103);
            c.w64(2, c.r64(3));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(3, c.r64(9));
            let Some(t104) = c.ld64(c.r64(4).wrapping_add(0x68)) else { return RegfileExit::fault(0x70007f82c8); };
            c.w64(10, t104);
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(10)) { return RegfileExit::fault(0x70007f82cd); }
            let r105 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r105);
            return RegfileExit::jump(0x70007f7294);
        }
        64 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f82e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70007f82e4); }
            return c.call(0x70007fec74, 0x70007f82ed, 0x70007f82e8);
        }
        65 => {
            c.w64(8, c.r64(0));
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f82f0); };
            c.w64(7, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f82f4); };
            c.w64(6, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f82f8); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f82fc); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f8300); };
            c.w64(9, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f8304); };
            c.w64(11, t111);
            blk = 28; continue;
        }
        66 => {
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007f830d); };
            c.cmp(t112, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t113) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f831c); };
            c.cmp(t113, c.r32(7), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f832b); }
            let r114 = c.r32(8) | c.r32(9);
            c.w32(8, r114);
            c.logic(r114, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70007f8338); };
            c.w64(3, t115);
            c.w64(0, c.r64(7));
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70007f834a, 0x70007f8345);
        }
        68 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t116) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8352); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f8359); };
            c.logic(t118 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(3, c.r64(0));
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8367); };
            c.w64(0, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f836b); };
            c.w64(6, t120);
            return c.call(0x7000845bd4, 0x70007f8374, 0x70007f836f);
        }
        71 => {
            let Some(t121) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8374); };
            c.w32(8, t121);
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            let Some(t123) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f837b); };
            c.w32(8, t123);
            let r124 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r124);
            let Some(t125) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8384); };
            c.w32(9, t125);
            let r126 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r126);
            let r127 = c.r32(8) & 0x3fu64;
            c.w32(8, r127);
            c.logic(r127, 32);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_l() { blk = 79; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f8399); };
            c.w64(1, t128);
            c.w64(7, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f83a0); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f83a4); };
            c.w64(3, t130);
            blk = 44; continue;
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f83ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70007f83b1); }
            return c.call(0x70007febb4, 0x70007f83ba, 0x70007f83b5);
        }
        74 => {
            c.w64(15, c.r64(0));
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f83bd); };
            c.w64(6, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f83c1); };
            c.w64(3, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f83c5); };
            c.w64(1, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f83c9); };
            c.w64(9, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f83cd); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f83d1); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f83d8); };
            c.w64(7, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f83df); };
            c.w64(11, t138);
            blk = 49; continue;
        }
        75 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70007f83ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70007f83f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70007f83f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70007f83f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70007f83fd); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x70007f8401); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70007f840e, 0x70007f8409);
        }
        76 => {
            c.w64(3, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f8411); };
            c.w64(8, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f8415); };
            c.w64(15, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f8419); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f841d); };
            c.w64(11, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70007f8421); };
            c.w64(1, t143);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x70007f8425); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f842a); };
            c.w64(12, t144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8431); };
            c.w64(7, t145);
            let Some(t146) = c.ld64(0x70007f8206u64) else { return RegfileExit::fault(0x70007f8438); };
            c.w64(2, t146);
            blk = 58; continue;
        }
        77 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f8444); };
            c.w64(3, t147);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f8454, 0x70007f844f);
        }
        78 => {
            return RegfileExit::fault(0x70007f8454);
        }
        79 => {
            c.w64(7, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f8458); };
            c.w64(1, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f845c); };
            c.w64(6, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8460); };
            c.w64(3, t150);
            blk = 63; continue;
        }
        80 => {
            c.w32(8, 0x90u64);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f846f); };
            c.w64(7, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f8473); };
            c.w64(6, t152);
            blk = 82; continue;
        }
        81 => {
            c.w32(8, 0xd0u64);
            blk = 82; continue;
        }
        82 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f847f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f8481); }
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x21f8)) else { return RegfileExit::fault(0x70007f8482); };
            c.w64(3, t153);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007f8493, 0x70007f848e);
        }
        83 => {
            return RegfileExit::fault(0x70007f8493);
        }
        84 => {
            c.w32(3, c.r32(3));
            let r154 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r154);
            c.w64(6, 0xfffffffffffc0000u64);
            let r155 = c.r64(6) & c.r64(3);
            c.w64(6, r155);
            c.logic(r155, 64);
            let Some(t156) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f84a3); };
            c.logic(t156 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f84b1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f84b2); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70007f84bf, 0x70007f84ba);
        }
        86 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f84bf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f84c1); }
            blk = 60; continue;
        }
        87 => {
            c.w32(8, c.r32(8));
            let r157 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r157);
            c.w64(12, 0xfffffffffffc0000u64);
            let r158 = c.r64(12) & c.r64(8);
            c.w64(12, r158);
            c.logic(r158, 64);
            let Some(t159) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f84d7); };
            c.logic(t159 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f84e7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f84e8); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f84f5, 0x70007f84f0);
        }
        89 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f84f5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f84f7); }
            blk = 32; continue;
        }
        90 => {
            c.w32(15, c.r32(15));
            let r160 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r160);
            c.w64(2, 0xfffffffffffc0000u64);
            let r161 = c.r64(2) & c.r64(15);
            c.w64(2, r161);
            c.logic(r161, 64);
            let Some(t162) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f850d); };
            c.logic(t162 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(0, c.r64(8).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f851b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f851c); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70007f8529, 0x70007f8524);
        }
        92 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8529); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f852b); }
            blk = 30; continue;
        }
        93 => {
            c.w32(8, c.r32(8));
            let r163 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r163);
            c.w64(12, 0xfffffffffffc0000u64);
            let r164 = c.r64(12) & c.r64(8);
            c.w64(12, r164);
            c.logic(r164, 64);
            let Some(t165) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8541); };
            c.logic(t165 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f8551); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8552); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f855f, 0x70007f855a);
        }
        95 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f855f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f8561); }
            blk = 24; continue;
        }
        96 => {
            return RegfileExit::jump(0x70007f8568);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f8574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f8574); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f8578); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f857e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70007f8582); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f8586); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8594); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f859b); };
            c.logic(t3 & 0x40u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f85af); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f85b6); };
            c.logic(t6 & 0x40u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f85ca); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f85d1); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f85d8); };
            c.w32(9, t11);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f85df); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70007f85f2);
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70007f85f3); };
            c.cmp(t14, c.r32(11), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r64(9), 0xb0u64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f8605);
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70007f8606); };
            c.cmp(t15, c.r32(11), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f860f); };
            c.cmp(t16, 0x834u64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f861c); };
            c.w32(12, t17);
            c.w64(15, c.r64(12));
            let r18 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r18);
            let r19 = c.r32(15) & 0x3fu64;
            c.w32(15, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_g() { blk = 45; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8636); };
            c.w32(15, t20);
            let r21 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r21);
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f863d); };
            c.w32(0, t22);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 14; continue;
        }
        14 => {
            let r23 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            blk = 17; continue;
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f864e); };
            c.w32(8, t24);
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8652); };
            c.w32(15, t25);
            let r26 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r26);
            let Some(t27) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f8659); };
            c.cmp(t27, c.r32(8), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 16; continue;
        }
        16 => {
            let r28 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r28);
            c.w64(0, c.r64(8));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f866e); };
            c.w64(15, t29);
            blk = 19; continue;
        }
        19 => {
            let Some(t30) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8675); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70007f8679); }
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70007f867d); };
            c.cmp(t31, c.r32(8), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 51; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f869a); };
            c.w64(9, t32);
            let Some(t33) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f86a1); };
            c.w64(11, t33);
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f86b0); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f86b7); };
            c.cmp(t35, c.r64(3), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 47; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70007f86cd); }
            let r36 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r36);
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f86d8); };
            c.w64(3, t37);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70007f86df); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f86e3); }
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(3, c.r64(12));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f86f8); };
            c.w64(6, t38);
            blk = 26; continue;
        }
        26 => {
            let r39 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(11)), c.r32(6)) { return RegfileExit::fault(0x70007f8703); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(8, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(3, 0xfff7fffffff7ffffu64);
            blk = 30; continue;
        }
        29 => {
            let r40 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f872f); };
            c.w64(15, t41);
            blk = 30; continue;
        }
        30 => {
            let r42 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r42);
            let Some(t43) = c.ld64(c.r64(8).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70007f8737); };
            c.cmp(t43, c.r64(3), 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(15))) { return RegfileExit::fault(0x70007f873d); }
            let Some(t44) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f8743); };
            c.w64(6, t44);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            c.w64(2, c.r64(12).wrapping_sub(0x4));
            let Some(t45) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70007f874f); };
            c.cmp(t45, c.r64(7), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70007f8758); }
            let r46 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r46);
            blk = 33; continue;
        }
        33 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f875f); };
            c.w64(7, t47);
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70007f8766); }
            if !c.movsd_mr(c.r64(6).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f8769); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(2)), c.r32(6)) { return RegfileExit::fault(0x70007f876e); }
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r48 = c.r64(7) & c.r64(11);
            c.w64(7, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8782); };
            c.logic(t49 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 29; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8792); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8796); };
            c.w64(2, t51);
            blk = 37; continue;
        }
        37 => {
            let Some(t52) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f879a); };
            c.w64(5, t52);
            let Some(t53) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f879e); };
            c.w64(6, t53);
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(11));
            let r54 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r54);
            return RegfileExit::jump(0x70007f7294);
        }
        38 => {
            let Some(t55) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f87b4); };
            c.w64(5, t55);
            c.w64(3, c.r64(15));
            c.w64(1, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let r56 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r56);
            return RegfileExit::jump(0x70007f7294);
        }
        39 => {
            blk = 55; continue;
        }
        40 => {
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70007f87d4);
        }
        42 => {
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f87d5); };
            c.w32(8, t57);
            let Some(t58) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f87dc); };
            c.cmp(t58, c.r32(8), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f87e2); };
            c.w64(8, t59);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t60) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f87ec); };
            c.cmp(t60, c.r32(9), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r61 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r61);
            blk = 17; continue;
        }
        45 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f87f9); }
            c.w32(0, 0x1u64);
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2108)) else { return RegfileExit::fault(0x70007f87ff); };
            c.w64(3, t62);
            return c.call(0x700086ed74, 0x70007f880b, 0x70007f8806);
        }
        46 => {
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f880b); };
            c.w32(8, t63);
            let r64 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r64);
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(8));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8818); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f881c); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8820); };
            c.w64(2, t67);
            blk = 17; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70007f8829); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70007f882d); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70007f8839, 0x70007f8834);
        }
        48 => {
            c.w64(11, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f883c); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f8840); };
            c.w64(6, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8844); };
            c.w64(2, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8848); };
            c.w64(0, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f884c); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f8850); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8857); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f885e); };
            c.w64(8, t75);
            blk = 24; continue;
        }
        49 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70007f8867); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70007f8870); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70007f8874); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70007f8878); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70007f887c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70007f8880); }
            c.w64(6, c.r64(2));
            return c.call(0x70007febf4, 0x70007f888d, 0x70007f8888);
        }
        50 => {
            c.w64(6, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8890); };
            c.w64(0, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f8894); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f8898); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f889c); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f88a0); };
            c.w64(2, t80);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70007f88a4); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f88a9); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f88b0); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(0x70007f8721u64) else { return RegfileExit::fault(0x70007f88b7); };
            c.w64(3, t83);
            blk = 33; continue;
        }
        51 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f88c3); };
            c.w64(3, t84);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f88d3, 0x70007f88ce);
        }
        52 => {
            return RegfileExit::fault(0x70007f88d3);
        }
        53 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f88d4); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f88d5); };
            c.w64(3, t85);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f88e6, 0x70007f88e1);
        }
        54 => {
            return RegfileExit::fault(0x70007f88e6);
        }
        55 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f88e7); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f88e8); };
            c.w64(3, t86);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f88f9, 0x70007f88f4);
        }
        56 => {
            return RegfileExit::fault(0x70007f88f9);
        }
        57 => {
            c.w32(6, c.r32(6));
            let r87 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r87);
            c.w64(7, 0xfffffffffffc0000u64);
            let r88 = c.r64(7) & c.r64(6);
            c.w64(7, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8909); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f8917); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8918); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f8925, 0x70007f8920);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8925); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f8927); }
            blk = 35; continue;
        }
        60 => {
            return RegfileExit::jump(0x70007f8930);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f8934(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f8934); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f8938); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f893e); };
            c.w32(8, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70007f8942); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007f8946); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70007f894a); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f894e); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(2).wrapping_mul(4));
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f8963); };
            c.w32(9, t3);
            let r4 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r4);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f896f); }
            c.w64(9, c.r64(2).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f897c); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(12, c.r32(11));
            let r7 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(3), c.r32(12), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f8994); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f89a1); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(12, c.r32(11));
            let r10 = c.r32(12) & 0x3u64;
            c.w32(12, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f89bd); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            let Some(t13) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f89c4); };
            c.cmp(t13, 0x5au64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f89d5); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f89dc); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f89e3); };
            c.w32(11, t18);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f89f3);
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f89f4); };
            c.cmp(t19, c.r32(3), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 26; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8a14); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f8a1b); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f8a22); };
            c.w32(12, t24);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x70007f8a32);
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f8a33); };
            c.cmp(t25, c.r32(3), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(3));
            blk = 20; continue;
        }
        19 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f8a4e); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            blk = 20; continue;
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8a55); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8a5c); };
            c.w32(12, t30);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r31 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r31);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f8a7b); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8a82); };
            c.w32(12, t34);
            let r35 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8a89); };
            c.w32(11, t36);
            let r37 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f8a90); };
            c.w32(12, t38);
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f8a95); };
            c.cmp(t39, c.r32(12), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(3));
            let r40 = c.r64(8) | 0x3u64;
            c.w64(8, r40);
            c.logic(r40, 64);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f8aa2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r41 = c.r64(11) & c.r64(7);
            c.w64(11, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8ab6); };
            c.logic(t42 & 0x4u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f8ac1); }
            blk = 34; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f8acd); };
            c.w64(9, t43);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70007f8ad4); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f8ad9); }
            blk = 34; continue;
        }
        27 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70007f8ae5); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007f8aec); };
            c.w64(11, t45);
            c.w64(12, c.r64(11).wrapping_add(0x1c));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70007f8af3); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f8afa); };
            c.cmp(t47, c.r64(12), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70007f8b03); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70007f8b0a); };
            c.w64(11, t48);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007f8b11); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70007f8b18); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70007f8b1c); }
            if !c.st32(c.r64(8).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70007f8b20); }
            if !c.st32(c.r64(8).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70007f8b28); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70007f8b30); };
            c.w64(11, t49);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70007f8b37); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70007f8b3b); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f8b42); };
            c.w64(12, t51);
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70007f8b45); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r52 = c.r64(15) & c.r64(8);
            c.w64(15, r52);
            c.logic(r52, 64);
            let Some(t53) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8b59); };
            c.logic(t53 & 0x4u64, 8);
            if c.cc_ne() { blk = 98; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x70007f8b64); }
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f8b67); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r54 = c.r64(11) & c.r64(7);
            c.w64(11, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8b7b); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 95; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f8b86); }
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8b95); };
            c.w32(8, t56);
            let r57 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r57);
            let Some(t58) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f8b9c); };
            c.logic(t58 & 0x40u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t59) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8bb0); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f8bb7); };
            c.logic(t61 & 0x40u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8bcb); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f8bd2); };
            c.w32(8, t64);
            let r65 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r65);
            let Some(t66) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f8bd9); };
            c.w32(9, t66);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t67) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8be0); };
            c.w32(11, t67);
            let r68 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r68);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x70007f8bf3);
        }
        41 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70007f8bf4); };
            c.cmp(t69, c.r32(11), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(9), 0xb0u64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x70007f8c06);
        }
        44 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70007f8c07); };
            c.cmp(t70, c.r32(11), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8c10); };
            c.cmp(t71, 0x834u64, 16);
            if c.cc_ne() { blk = 80; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t72) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70007f8c1d); };
            c.w32(12, t72);
            c.w64(15, c.r64(12));
            let r73 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r73);
            let r74 = c.r32(15) & 0x3fu64;
            c.w32(15, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_g() { blk = 80; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8c37); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f8c3e); };
            c.w32(2, t77);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 48; continue;
        }
        48 => {
            let r78 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r78);
            blk = 51; continue;
        }
        49 => {
            let Some(t79) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f8c4f); };
            c.w32(8, t79);
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f8c53); };
            c.w32(15, t80);
            let r81 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r81);
            let Some(t82) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f8c5a); };
            c.cmp(t82, c.r32(8), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 50; continue;
        }
        50 => {
            let r83 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r83);
            c.w64(2, c.r64(8));
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f8c6f); };
            c.w64(15, t84);
            blk = 53; continue;
        }
        53 => {
            let Some(t85) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f8c76); };
            c.w32(8, t85);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70007f8c7a); }
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70007f8c7e); };
            c.cmp(t86, c.r32(8), 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(8, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 86; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f8c9b); };
            c.w64(9, t87);
            let Some(t88) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f8ca2); };
            c.w64(11, t88);
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8cb1); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f8cb8); };
            c.cmp(t90, c.r64(1), 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 82; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(9), c.r64(1)) { return RegfileExit::fault(0x70007f8cce); }
            let r91 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r91);
            blk = 58; continue;
        }
        58 => {
            c.w32(12, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007f8cd9); };
            c.w64(1, t92);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f8ce0); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f8ce4); }
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(1, c.r64(12));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f8cf9); };
            c.w64(6, t93);
            blk = 60; continue;
        }
        60 => {
            let r94 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r94);
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(6)) { return RegfileExit::fault(0x70007f8d04); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 60; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(8, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(1, 0xfff7fffffff7ffffu64);
            blk = 64; continue;
        }
        63 => {
            let r95 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f8d30); };
            c.w64(15, t96);
            blk = 64; continue;
        }
        64 => {
            let r97 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r97);
            let Some(t98) = c.ld64(c.r64(8).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70007f8d38); };
            c.cmp(t98, c.r64(1), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(15))) { return RegfileExit::fault(0x70007f8d3e); }
            let Some(t99) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f8d44); };
            c.w64(6, t99);
            c.w64(0, c.r64(6).wrapping_add(0xc));
            c.w64(3, c.r64(12).wrapping_sub(0x4));
            let Some(t100) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f8d50); };
            c.cmp(t100, c.r64(0), 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x70007f8d59); }
            c.w64(0, c.r64(6).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70007f8d60); };
            c.w64(6, t101);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70007f8d67); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70007f8d6a); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(3)), c.r32(0)) { return RegfileExit::fault(0x70007f8d6f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r102 = c.r64(6) & c.r64(11);
            c.w64(6, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8d81); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 63; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8d91); };
            c.w64(0, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8d95); };
            c.w64(3, t105);
            blk = 71; continue;
        }
        71 => {
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f8d99); };
            c.w64(5, t106);
            let Some(t107) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007f8d9d); };
            c.w64(6, t107);
            c.w64(7, c.r64(0));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(11));
            let r108 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r108);
            return RegfileExit::jump(0x70007f7294);
        }
        72 => {
            let Some(t109) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007f8db9); };
            c.w64(5, t109);
            c.w64(7, c.r64(0));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(15));
            let r110 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r110);
            return RegfileExit::jump(0x70007f7294);
        }
        73 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70007f8dd9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70007f8ddd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70007f8de1); }
            return c.call(0x70007fec74, 0x70007f8dea, 0x70007f8de5);
        }
        74 => {
            c.w64(8, c.r64(0));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8ded); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f8df1); };
            c.w64(6, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8df5); };
            c.w64(3, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8df9); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f8dfd); };
            c.w64(7, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f8e01); };
            c.w64(9, t116);
            blk = 29; continue;
        }
        75 => {
            c.cmp(c.r64(9), 0x4cu64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70007f8e10);
        }
        77 => {
            let Some(t117) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x70007f8e11); };
            c.w32(8, t117);
            let Some(t118) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f8e18); };
            c.cmp(t118, c.r32(8), 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70007f8e1e); };
            c.w64(8, t119);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f8e28); };
            c.cmp(t120, c.r32(9), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            let r121 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r121);
            blk = 51; continue;
        }
        80 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f8e35); }
            c.w32(0, 0x1u64);
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2108)) else { return RegfileExit::fault(0x70007f8e3b); };
            c.w64(3, t122);
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x70007f8e4d, 0x70007f8e48);
        }
        81 => {
            let Some(t123) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f8e4d); };
            c.w32(8, t123);
            let r124 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r124);
            c.w64(15, c.r64(0));
            c.w64(2, c.r64(8));
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8e5a); };
            c.w64(0, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f8e5e); };
            c.w64(6, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8e62); };
            c.w64(3, t127);
            blk = 51; continue;
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007f8e6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70007f8e6f); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70007f8e7b, 0x70007f8e76);
        }
        83 => {
            c.w64(11, c.r64(0));
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f8e7e); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f8e82); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f8e86); };
            c.w64(3, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8e8a); };
            c.w64(2, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f8e8e); };
            c.w64(15, t132);
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f8e92); };
            c.w64(9, t133);
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8e99); };
            c.w64(7, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f8ea0); };
            c.w64(8, t135);
            blk = 58; continue;
        }
        84 => {
            c.w32(0, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007f8eae); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70007f8eb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70007f8eb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70007f8eba); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70007f8ebe); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70007f8ec2); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70007f8ecf, 0x70007f8eca);
        }
        85 => {
            let Some(t136) = c.ld64(0x70007f8d22u64) else { return RegfileExit::fault(0x70007f8ecf); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f8ed6); };
            c.w64(2, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f8eda); };
            c.w64(11, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007f8ede); };
            c.w64(12, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007f8ee2); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007f8ee6); };
            c.w64(3, t141);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70007f8eea); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f8eef); };
            c.w64(9, t142);
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f8ef6); };
            c.w64(7, t143);
            blk = 67; continue;
        }
        86 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007f8f02); };
            c.w64(3, t144);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007f8f12, 0x70007f8f0d);
        }
        87 => {
            return RegfileExit::fault(0x70007f8f12);
        }
        88 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f8f13); }
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f8f14); };
            c.w64(3, t145);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f8f25, 0x70007f8f20);
        }
        89 => {
            return RegfileExit::fault(0x70007f8f25);
        }
        90 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f8f26); }
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f8f27); };
            c.w64(3, t146);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f8f38, 0x70007f8f33);
        }
        91 => {
            return RegfileExit::fault(0x70007f8f38);
        }
        92 => {
            c.w32(0, c.r32(0));
            let r147 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r147);
            c.w64(6, 0xfffffffffffc0000u64);
            let r148 = c.r64(6) & c.r64(0);
            c.w64(6, r148);
            c.logic(r148, 64);
            let Some(t149) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8f48); };
            c.logic(t149 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f8f56); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8f57); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f8f64, 0x70007f8f5f);
        }
        94 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8f64); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f8f66); }
            blk = 69; continue;
        }
        95 => {
            c.w32(8, c.r32(8));
            let r150 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r150);
            c.w64(11, 0xfffffffffffc0000u64);
            let r151 = c.r64(11) & c.r64(8);
            c.w64(11, r151);
            c.logic(r151, 64);
            let Some(t152) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8f7c); };
            c.logic(t152 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8f8b); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f8f95, 0x70007f8f90);
        }
        97 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8f95); }
            blk = 33; continue;
        }
        98 => {
            c.w32(12, c.r32(12));
            let r153 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r153);
            c.w64(15, 0xfffffffffffc0000u64);
            let r154 = c.r64(15) & c.r64(12);
            c.w64(15, r154);
            c.logic(r154, 64);
            let Some(t155) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8fac); };
            c.logic(t155 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(2, c.r64(8).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f8fbb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8fbc); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70007f8fc9, 0x70007f8fc4);
        }
        100 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8fc9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f8fcb); }
            blk = 31; continue;
        }
        101 => {
            c.w32(8, c.r32(8));
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            c.w64(11, 0xfffffffffffc0000u64);
            let r157 = c.r64(11) & c.r64(8);
            c.w64(11, r157);
            c.logic(r157, 64);
            let Some(t158) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f8fe1); };
            c.logic(t158 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(12, c.r64(7).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f8ff0); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f8ffa, 0x70007f8ff5);
        }
        103 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f8ffa); }
            blk = 25; continue;
        }
        104 => {
            return RegfileExit::jump(0x70007f9004);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f9014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9019); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f901f); };
            c.logic(t2 & 0x40u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9025); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f9029); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f902b); }
            c.w32(0, 0x1u64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f9031); };
            c.w64(3, t3);
            return c.call(0x700086ed74, 0x70007f903d, 0x70007f9038);
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f903d); };
            c.w64(8, t4);
            let r5 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r5);
            if c.cc_le() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(10, c.r64(4));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f904d); };
            let r7 = c.sub(c.r64(10), t6, 64);
            c.w64(10, r7);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(8), 64);
            if c.cc_le() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(4));
            c.w64(10, c.r64(8).wrapping_mul(8));
            blk = 7; continue;
        }
        6 => {
            let r8 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r8);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f9071); }
            let r9 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r9);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            let r10 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r10);
            c.w64(12, c.r64(0).wrapping_add(0x2));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f9094); };
            c.w64(10, t11);
            if !c.st64(c.r64(4).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f9098); }
            let r12 = c.inc(c.r64(15), 64);
            c.w64(15, r12);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_l() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            let r13 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r13);
            c.w64(1, c.r64(1).wrapping_mul(8).wrapping_add(0x18));
            let r14 = c.add(c.r64(1), c.r64(5), 64);
            c.w64(1, r14);
            blk = 12; continue;
        }
        12 => {
            let r15 = c.dec(c.r64(8), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f90b9); };
            c.w64(10, t16);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f90bd); }
            if c.cc_ne() { blk = 12; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 15; continue;
        }
        14 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f90c7); };
            c.w64(3, t17);
            return RegfileExit::jump(0x700086ed74);
        }
        15 => {
            return RegfileExit::jump(0x70007f7214);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f90f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f90f9); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f90ff); };
            c.logic(t2 & 0x40u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9105); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f9109); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f910b); }
            c.w32(0, 0x1u64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2208)) else { return RegfileExit::fault(0x70007f9111); };
            c.w64(3, t3);
            return c.call(0x700086ed74, 0x70007f911d, 0x70007f9118);
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f911d); };
            c.w64(8, t4);
            let r5 = c.sub(c.r32(8), c.r32(1), 32);
            c.w32(8, r5);
            if c.cc_le() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(10, c.r64(4));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f912d); };
            let r7 = c.sub(c.r64(10), t6, 64);
            c.w64(10, r7);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(8), 64);
            if c.cc_le() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(4));
            c.w64(10, c.r64(8).wrapping_mul(8));
            blk = 7; continue;
        }
        6 => {
            let r8 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r8);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f9151); }
            let r9 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r9);
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            let r10 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r10);
            c.w64(12, c.r64(0).wrapping_add(0x2));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f9174); };
            c.w64(10, t11);
            if !c.st64(c.r64(4).wrapping_add(c.r64(15).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f9178); }
            let r12 = c.inc(c.r64(15), 64);
            c.w64(15, r12);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_l() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            let r13 = c.add(c.r32(0), c.r32(8), 32);
            c.w32(0, r13);
            c.w64(1, c.r64(1).wrapping_mul(8).wrapping_add(0x18));
            let r14 = c.add(c.r64(1), c.r64(5), 64);
            c.w64(1, r14);
            blk = 12; continue;
        }
        12 => {
            let r15 = c.dec(c.r64(8), 64);
            c.w64(8, r15);
            let Some(t16) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f9199); };
            c.w64(10, t16);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f919d); }
            if c.cc_ne() { blk = 12; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 15; continue;
        }
        14 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f91a7); };
            c.w64(3, t17);
            return RegfileExit::jump(0x700086ed74);
        }
        15 => {
            return RegfileExit::jump(0x70007f7134);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f91d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(5));
            let Some(t0) = c.ld64(c.r64(1).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70007f91d7); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(1).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f91db); };
            c.w64(1, t1);
            c.w64(9, c.r64(3).wrapping_mul(4));
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f91e7); };
            c.w32(11, t2);
            let r3 = c.add(c.r32(11), 0x8u64, 32);
            c.w32(11, r3);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x23), c.r32(11)) { return RegfileExit::fault(0x70007f91f3); }
            c.w64(11, c.r64(3).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f9200); };
            c.w32(12, t4);
            let r5 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r5);
            c.w32(15, c.r32(12));
            let r6 = c.r32(15) & 0xfffffffffffffffdu64;
            c.w32(15, r6);
            c.logic(r6, 32);
            c.cmp(c.r32(2), c.r32(15), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f9218); };
            c.cmp(t7, c.r32(12), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f9225); };
            c.cmp(t8, c.r32(12), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(15, c.r32(12));
            let r9 = c.r32(15) & 0x3u64;
            c.w32(15, r9);
            c.logic(r9, 32);
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9241); };
            c.w32(15, t10);
            let r11 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r11);
            let Some(t12) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f9249); };
            c.cmp(t12, 0x5au64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9259); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f9260); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9267); };
            c.w32(15, t17);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70007f9277);
        }
        9 => {
            let Some(t18) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f9278); };
            c.cmp(t18, c.r32(2), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 25; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9298); };
            c.w32(12, t19);
            let r20 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r20);
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f929f); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f92a7); };
            c.w32(15, t23);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2bu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70007f92b8);
        }
        15 => {
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f92b9); };
            c.cmp(t24, c.r32(2), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(12, c.r64(2));
            blk = 19; continue;
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f92d5); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            blk = 19; continue;
        }
        19 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f92dd); };
            c.w32(15, t27);
            let r28 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r28);
            let Some(t29) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f92e5); };
            c.w32(15, t29);
            c.cmp(c.r16(15), 0x80bu64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(15, c.r16(15));
            let r30 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r30);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f9303); };
            c.w32(12, t31);
            let r32 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f930b); };
            c.w32(15, t33);
            let r34 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9312); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f931a); };
            c.w32(15, t37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f931e); };
            c.cmp(t38, c.r32(15), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, c.r64(2));
            let r39 = c.r64(9) | 0x3u64;
            c.w64(9, r39);
            c.logic(r39, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(9)) { return RegfileExit::fault(0x70007f932c); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r40 = c.r64(12) & c.r64(1);
            c.w64(12, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9340); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f934c); }
            blk = 34; continue;
        }
        25 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f9358); };
            c.w64(11, t42);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70007f935f); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f9364); }
            blk = 34; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70007f9370); };
            c.w64(9, t43);
            let Some(t44) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70007f9377); };
            c.w64(12, t44);
            c.w64(15, c.r64(12).wrapping_add(0x1c));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70007f937f); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007f9386); };
            c.cmp(t46, c.r64(15), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70007f938f); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70007f9397); };
            c.w64(12, t47);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70007f939e); }
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70007f93a5); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70007f93a9); }
            if !c.st32(c.r64(9).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70007f93ad); }
            if !c.st32(c.r64(9).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70007f93b5); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70007f93bd); };
            c.w64(12, t48);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70007f93c4); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70007f93c8); };
            c.w64(12, t49);
            let Some(t50) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70007f93cf); };
            c.w64(15, t50);
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70007f93d3); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r51 = c.r64(3) & c.r64(9);
            c.w64(3, r51);
            c.logic(r51, 64);
            let Some(t52) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f93e7); };
            c.logic(t52 & 0x4u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(12), c.r64(9)) { return RegfileExit::fault(0x70007f93f1); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(9)) { return RegfileExit::fault(0x70007f93f5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r53 = c.r64(12) & c.r64(1);
            c.w64(12, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9409); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f9411); }
            c.w64(12, c.r64(9));
            blk = 33; continue;
        }
        33 => {
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(12));
            return RegfileExit::jump(0x7000824374);
        }
        34 => {
            c.w64(6, c.r64(8));
            return RegfileExit::jump(0x70007f7214);
        }
        35 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f942e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f9432); }
            let r55 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r55);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007f9438); }
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f9441); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f9445); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70007f9449); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70007f944d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70007f9451); }
            c.w64(9, c.r64(2));
            return c.call(0x70007fec74, 0x70007f945d, 0x70007f9458);
        }
        36 => {
            c.w64(9, c.r64(0));
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f9460); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f9464); };
            c.w64(2, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f9468); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f946c); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f9470); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007f9474); };
            c.w64(11, t61);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f947b); }
            blk = 28; continue;
        }
        37 => {
            c.w32(9, c.r32(9));
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            c.w64(12, 0xfffffffffffc0000u64);
            let r63 = c.r64(12) & c.r64(9);
            c.w64(12, r63);
            c.logic(r63, 64);
            let Some(t64) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9491); };
            c.logic(t64 & 0x2u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(15, c.r64(1).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f94a1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f94a2); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f94af, 0x70007f94aa);
        }
        39 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f94af); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f94b1); }
            blk = 32; continue;
        }
        40 => {
            c.w32(15, c.r32(15));
            let r65 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r65);
            c.w64(3, 0xfffffffffffc0000u64);
            let r66 = c.r64(3) & c.r64(15);
            c.w64(3, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f94c7); };
            c.logic(t67 & 0x2u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(6, c.r64(9).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f94d5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f94d6); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70007f94e3, 0x70007f94de);
        }
        42 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f94e3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f94e5); }
            blk = 30; continue;
        }
        43 => {
            c.w32(9, c.r32(9));
            let r68 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r68);
            c.w64(12, 0xfffffffffffc0000u64);
            let r69 = c.r64(12) & c.r64(9);
            c.w64(12, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f94fb); };
            c.logic(t70 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(15, c.r64(1).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f950b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f950c); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70007f9519, 0x70007f9514);
        }
        45 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f9519); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f951b); }
            blk = 24; continue;
        }
        46 => {
            return RegfileExit::jump(0x70007f9524);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f9534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9534); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9539); };
            c.w32(8, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70007f953d); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(3).wrapping_mul(4));
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70007f9552); };
            c.w32(9, t3);
            let r4 = c.add(c.r32(9), 0x8u64, 32);
            c.w32(9, r4);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x23), c.r32(9)) { return RegfileExit::fault(0x70007f955e); }
            c.w64(9, c.r64(3).wrapping_mul(4).wrapping_add(0x1f));
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f956b); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            c.w32(12, c.r32(11));
            let r7 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(2), c.r32(12), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f9583); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70007f9590); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(12, c.r32(11));
            let r10 = c.r32(12) & 0x3u64;
            c.w32(12, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f95ac); };
            c.w32(12, t11);
            let r12 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r12);
            let Some(t13) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f95b3); };
            c.cmp(t13, 0x5au64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f95c4); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f95cb); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f95d2); };
            c.w32(12, t18);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            return RegfileExit::fault(0x70007f95e2);
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f95e3); };
            c.cmp(t19, c.r32(2), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 26; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9603); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f960a); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9611); };
            c.w32(12, t24);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x2bu64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x70007f9621);
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70007f9622); };
            c.cmp(t25, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(11, c.r64(2));
            blk = 20; continue;
        }
        19 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f963d); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            blk = 20; continue;
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9644); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f964b); };
            c.w32(12, t30);
            c.cmp(c.r16(12), 0x80bu64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(12, c.r16(12));
            let r31 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r31);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t32) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f966a); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9671); };
            c.w32(12, t34);
            let r35 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9678); };
            c.w32(11, t36);
            let r37 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r37);
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f967f); };
            c.w32(12, t38);
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f9684); };
            c.cmp(t39, c.r32(12), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, c.r64(2));
            let r40 = c.r64(8) | 0x3u64;
            c.w64(8, r40);
            c.logic(r40, 64);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f9691); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r41 = c.r64(11) & c.r64(1);
            c.w64(11, r41);
            c.logic(r41, 64);
            let Some(t42) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f96a5); };
            c.logic(t42 & 0x4u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f96b0); }
            blk = 35; continue;
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x70007f96bc); };
            c.w64(9, t43);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x1f), c.r32(9)) { return RegfileExit::fault(0x70007f96c3); }
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f96c8); }
            blk = 35; continue;
        }
        27 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70007f96d4); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007f96db); };
            c.w64(11, t45);
            c.w64(12, c.r64(11).wrapping_add(0x1c));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70007f96e2); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70007f96e9); };
            c.cmp(t47, c.r64(12), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70007f96f2); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70007f96f9); };
            c.w64(11, t48);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70007f9700); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70007f9707); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70007f970b); }
            if !c.st32(c.r64(8).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70007f970f); }
            if !c.st32(c.r64(8).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70007f9717); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70007f971f); };
            c.w64(11, t49);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70007f9726); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70007f972a); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f9731); };
            c.w64(12, t51);
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70007f9734); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r52 = c.r64(15) & c.r64(8);
            c.w64(15, r52);
            c.logic(r52, 64);
            let Some(t53) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9748); };
            c.logic(t53 & 0x4u64, 8);
            if c.cc_ne() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(11), c.r64(8)) { return RegfileExit::fault(0x70007f9753); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(8)) { return RegfileExit::fault(0x70007f9756); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r54 = c.r64(11) & c.r64(1);
            c.w64(11, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f976a); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70007f9775); }
            c.w64(11, c.r64(8));
            blk = 34; continue;
        }
        34 => {
            let Some(t56) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007f977f); };
            c.w64(10, t56);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x70007f9783); }
            c.w64(3, c.r64(11));
            let r57 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r57);
            return RegfileExit::jump(0x7000824374);
        }
        35 => {
            let Some(t58) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007f9794); };
            c.w64(10, t58);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x70007f9798); }
            let r59 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r59);
            return RegfileExit::jump(0x70007f7214);
        }
        36 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f97a6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f97aa); }
            let r60 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r60);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007f97b0); }
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f97b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007f97bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007f97c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70007f97c5); }
            c.w64(8, c.r64(2));
            return c.call(0x70007fec74, 0x70007f97d1, 0x70007f97cc);
        }
        37 => {
            c.w64(8, c.r64(0));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f97d4); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f97d8); };
            c.w64(2, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f97dc); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f97e0); };
            c.w64(6, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f97e4); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007f97e8); };
            c.w64(9, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f97ef); }
            blk = 29; continue;
        }
        38 => {
            c.w32(8, c.r32(8));
            let r67 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r67);
            c.w64(11, 0xfffffffffffc0000u64);
            let r68 = c.r64(11) & c.r64(8);
            c.w64(11, r68);
            c.logic(r68, 64);
            let Some(t69) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9805); };
            c.logic(t69 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9814); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f9815); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f9822, 0x70007f981d);
        }
        40 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f9822); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f9824); }
            blk = 33; continue;
        }
        41 => {
            c.w32(12, c.r32(12));
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            c.w64(15, 0xfffffffffffc0000u64);
            let r71 = c.r64(15) & c.r64(12);
            c.w64(15, r71);
            c.logic(r71, 64);
            let Some(t72) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f983a); };
            c.logic(t72 & 0x2u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(3, c.r64(8).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9849); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f984a); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70007f9857, 0x70007f9852);
        }
        43 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f9857); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f9859); }
            blk = 31; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let r73 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r73);
            c.w64(11, 0xfffffffffffc0000u64);
            let r74 = c.r64(11) & c.r64(8);
            c.w64(11, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f986f); };
            c.logic(t75 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(12, c.r64(1).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f987e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f987f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70007f988c, 0x70007f9887);
        }
        46 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70007f988c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f988e); }
            blk = 25; continue;
        }
        47 => {
            return RegfileExit::jump(0x70007f9898);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f98b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f98b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x26u64) { return RegfileExit::fault(0x70007f98b8); }
            c.w32(1, c.r32(0));
            let r0 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r0);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f98be); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f98bf); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f98c0); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f98c1); };
            if !c.push64(t1) { return RegfileExit::fault(0x70007f98c1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f98c8); }
            let Some(t2) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f98c9); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f98cf); };
            c.w32(3, t4);
            let r5 = c.r64(3) & 0x1fu64;
            c.w64(3, r5);
            c.logic(r5, 64);
            c.w32(10, c.r64(3).wrapping_sub(0x5));
            c.cmp(c.r32(10), 0x1u64, 32);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            return c.call(0x70007f9c54, 0x70007f98e5, 0x70007f98e0);
        }
        2 => {
            blk = 4; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f98e7); };
            c.w64(0, t6);
            blk = 4; continue;
        }
        4 => {
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f98ee); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f98ef); }
            c.w64(8, c.r64(0));
            c.w64(3, c.r64(5).wrapping_add(0x18));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f98f7); };
            c.w64(7, t7);
            let Some(t8) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f98fb); };
            c.w64(0, (t8 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(10, c.r64(4));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f9905); };
            let r10 = c.sub(c.r64(10), t9, 64);
            c.w64(10, r10);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 37; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(1, c.r64(0));
            blk = 7; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f991e); };
            if !c.push64(t11) { return RegfileExit::fault(0x70007f991e); }
            blk = 7; continue;
        }
        7 => {
            let r12 = c.dec(c.r64(1), 64);
            c.w64(1, r12);
            if c.cc_ge() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70007f9926); }
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f9928); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld16(c.r64(3).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f992e); };
            c.w32(3, t15);
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f9932); };
            c.w32(6, t16);
            let r17 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r17);
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007f9938); };
            c.w64(10, t18);
            let Some(t19) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007f993f); };
            c.cmp(t19, 0x0u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0xffffu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 10; continue;
        }
        10 => {
            let r20 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r20);
            if c.cc_le() { blk = 22; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(10, c.r64(4));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f9961); };
            let r22 = c.sub(c.r64(10), t21, 64);
            c.w64(10, r22);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(3), 64);
            if c.cc_le() { blk = 20; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(3).wrapping_mul(8));
            blk = 14; continue;
        }
        13 => {
            let r23 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r23);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f9989); }
            let r24 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r24);
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 13; continue; }
            blk = 15; continue;
        }
        15 => {
            let r25 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r25);
            c.w64(9, c.r64(0).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f99a7); };
            c.w64(10, t26);
            if !c.st64(c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f99ab); }
            let r27 = c.inc(c.r64(11), 64);
            c.w64(11, r27);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_l() { blk = 16; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f99bb); };
            c.w64(10, t28);
            blk = 18; continue;
        }
        18 => {
            let r29 = c.dec(c.r64(3), 64);
            c.w64(3, r29);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f99c5); }
            if c.cc_g() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 22; continue;
        }
        20 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f99d2); };
            c.w64(3, t30);
            return c.call(0x700086ed74, 0x70007f99de, 0x70007f99d9);
        }
        21 => {
            return RegfileExit::fault(0x70007f99de);
        }
        22 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f99df); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f99e5); };
            c.logic(t33 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r34 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r34);
            blk = 25; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f99fb); };
            c.w32(1, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007f99fe); };
            c.w64(1, t36);
            blk = 25; continue;
        }
        25 => {
            return c.call(c.r64(1), 0x70007f9a08, 0x70007f9a06);
        }
        26 => {
            blk = 29; continue;
        }
        27 => {
            let r37 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r37);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f9a0c); }
            let r38 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r38);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f9a0f); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f9a16); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9a17); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9a18); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f9a19); };
            if !c.push64(t39) { return RegfileExit::fault(0x70007f9a19); }
            c.w32(0, 0x2u64);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007f9a21); };
            c.w64(3, t40);
            return c.call(0x700086ed74, 0x70007f9a2d, 0x70007f9a28);
        }
        28 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f9a2d); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f9a2e); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f9a2f); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f9a36); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 9; continue;
        }
        29 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9a42); };
            c.cmp(c.r32(0), t41, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t42) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70007f9a4b); };
            c.w64(0, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f9a4f); };
            c.cmp(c.r32(0), t43, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f9a58); };
            c.w64(3, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9a5f); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f9a60); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(4).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f9a69); }
            return c.ret(0x0, 0x70007f9a6a);
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9a6f); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f9a75); };
            c.cmp(t47, 0xbau64, 16);
            if c.cc_ae() { blk = 31; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 30; continue;
        }
        35 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f9a7f); };
            c.w64(6, t48);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x21d8)) else { return RegfileExit::fault(0x70007f9a85); };
            c.w64(3, t49);
            return c.call(0x700086ed74, 0x70007f9a91, 0x70007f9a8c);
        }
        36 => {
            return RegfileExit::fault(0x70007f9a91);
        }
        37 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f9a92); };
            c.w64(6, t50);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f9a98); };
            c.w64(3, t51);
            return c.call(0x700086ed74, 0x70007f9aa4, 0x70007f9a9f);
        }
        38 => {
            return RegfileExit::fault(0x70007f9aa4);
        }
        39 => {
            return RegfileExit::fault(0x70007f9aa5);
        }
        40 => {
            return RegfileExit::fault(0x70007f9aa6);
        }
        41 => {
            return RegfileExit::fault(0x70007f9aa7);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f9ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(10, c.r64(4));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f9ab7); };
            let r1 = c.sub(c.r64(10), t0, 64);
            c.w64(10, r1);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9ac8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x26u64) { return RegfileExit::fault(0x70007f9acc); }
            c.w32(1, c.r32(0));
            let r2 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007f9ad2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f9ad3); }
            c.w64(3, c.r64(5).wrapping_add(0x18));
            c.w64(1, c.r64(0));
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f9ae0); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007f9ae0); }
            blk = 3; continue;
        }
        3 => {
            let r4 = c.dec(c.r64(1), 64);
            c.w64(1, r4);
            if c.cc_ge() { blk = 2; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f9ae8); };
            if !c.push64(t5) { return RegfileExit::fault(0x70007f9ae8); }
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f9aef); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld16(c.r64(3).wrapping_add(0x15)) else { return RegfileExit::fault(0x70007f9af5); };
            c.w32(3, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70007f9af9); };
            c.w32(6, t9);
            let r10 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2e28)) else { return RegfileExit::fault(0x70007f9aff); };
            c.w64(10, t11);
            let Some(t12) = c.ld8(c.r64(10)) else { return RegfileExit::fault(0x70007f9b06); };
            c.cmp(t12, 0x0u64, 8);
            if c.cc_ne() { blk = 23; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(3), 0xffffu64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let r13 = c.sub(c.r64(3), c.r64(0), 64);
            c.w64(3, r13);
            if c.cc_le() { blk = 18; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(10, c.r64(4));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007f9b28); };
            let r15 = c.sub(c.r64(10), t14, 64);
            c.w64(10, r15);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(3), 64);
            if c.cc_le() { blk = 16; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, c.r64(4));
            c.w64(10, c.r64(3).wrapping_mul(8));
            blk = 10; continue;
        }
        9 => {
            let r16 = c.sub(c.r64(4), 0x1000u64, 64);
            c.w64(4, r16);
            if !c.st8(c.r64(4), 0x0u64) { return RegfileExit::fault(0x70007f9b50); }
            let r17 = c.sub(c.r64(10), 0x1000u64, 64);
            c.w64(10, r17);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(10), 0x1000u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            let r18 = c.sub(c.r64(4), c.r64(10), 64);
            c.w64(4, r18);
            c.w64(9, c.r64(0).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        12 => {
            let Some(t19) = c.ld64(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007f9b6e); };
            c.w64(10, t19);
            if !c.st64(c.r64(4).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f9b72); }
            let r20 = c.inc(c.r64(11), 64);
            c.w64(11, r20);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_l() { blk = 12; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(8, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9b82); };
            c.w64(10, t21);
            blk = 14; continue;
        }
        14 => {
            let r22 = c.dec(c.r64(3), 64);
            c.w64(3, r22);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(10)) { return RegfileExit::fault(0x70007f9b8c); }
            if c.cc_g() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 18; continue;
        }
        16 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f9b99); };
            c.w64(3, t23);
            return c.call(0x700086ed74, 0x70007f9ba5, 0x70007f9ba0);
        }
        17 => {
            return RegfileExit::fault(0x70007f9ba5);
        }
        18 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70007f9ba6); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f9bac); };
            c.logic(t26 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            let r27 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r27);
            blk = 21; continue;
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70007f9bc2); };
            c.w32(1, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70007f9bc5); };
            c.w64(1, t29);
            blk = 21; continue;
        }
        21 => {
            return c.call(c.r64(1), 0x70007f9bcf, 0x70007f9bcd);
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            let r30 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r30);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007f9bd3); }
            let r31 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r31);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007f9bd6); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f9bdd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9bde); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007f9bdf); }
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70007f9be0); };
            if !c.push64(t32) { return RegfileExit::fault(0x70007f9be0); }
            c.w32(0, 0x2u64);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x1ec0)) else { return RegfileExit::fault(0x70007f9be8); };
            c.w64(3, t33);
            return c.call(0x700086ed74, 0x70007f9bf4, 0x70007f9bef);
        }
        24 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007f9bf4); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x70007f9bf5); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x70007f9bf6); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007f9bfd); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 5; continue;
        }
        25 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f9c09); };
            c.w64(3, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9c10); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x70007f9c11); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(4).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x8));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f9c1a); }
            return c.ret(0x0, 0x70007f9c1b);
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9c1c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007f9c20); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007f9c24); };
            c.w64(3, t35);
            return c.call(0x700086ed74, 0x70007f9c30, 0x70007f9c2b);
        }
        27 => {
            return RegfileExit::fault(0x70007f9c30);
        }
        28 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9c34); }
            return RegfileExit::fault(0x70007f9c35);
        }
        29 => {
            return RegfileExit::fault(0x70007f9c36);
        }
        30 => {
            return RegfileExit::fault(0x70007f9c37);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f9c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9c54); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f9c5a); };
            c.logic(t2 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f9c64); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9c73); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f9c7a); };
            c.cmp(t7, 0xacu64, 16);
            if c.cc_ne() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f9c87); };
            c.cmp(t8, c.r32(7), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f9c90); };
            c.logic(t9 & 0x200000u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f9c99); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f9ca0); };
            c.w64(8, t11);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f9ca9); };
            c.w64(7, t12);
            blk = 16; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f9cb5); };
            c.w64(7, t13);
            let Some(t14) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f9cbc); };
            c.w64(8, t14);
            c.w64(9, c.r64(8).wrapping_add(0x4c));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f9cc3); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f9cca); };
            c.cmp(t16, c.r64(9), 64);
            if c.cc_be() { blk = 32; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70007f9cd3); }
            let r17 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r17);
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70007f9cda); };
            c.w64(9, t18);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70007f9ce1); }
            c.w32(9, 0x22u64);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70007f9ceb); }
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9cef); };
            c.w32(9, t19);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70007f9d05); }
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9d09); };
            c.w32(9, t20);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70007f9d1d); }
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9d21); };
            c.w32(9, t21);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(9, 0x8u64);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70007f9d3b); }
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9d3f); };
            c.w32(9, t22);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(9, 0x2u64);
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70007f9d59); }
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9d5d); };
            c.w32(9, t23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x70007f9d71); }
            c.w64(9, c.r64(8).wrapping_add(0x4b));
            c.w64(12, c.r64(8).wrapping_add(0x1b));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9d82); };
            c.w64(15, t24);
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(12), c.r32(15)) { return RegfileExit::fault(0x70007f9d89); }
            let r25 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r25);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 15; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9d96); };
            c.w32(9, t26);
            c.w64(2, c.r64(9).wrapping_mul(4));
            let Some(t27) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007f9da3); };
            c.w64(9, t27);
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t28) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f9daa); };
            c.cmp(t28, c.r64(12), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70007f9db3); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f9dbe); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70007f9dc1); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f9dc5); };
            c.w64(8, t29);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007f9dcc); }
            let Some(t30) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007f9dd0); };
            c.w32(8, t30);
            c.w64(11, c.r64(8));
            let r31 = c.r32(11) & 0xe0000000u64;
            c.w32(11, r31);
            c.logic(r31, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9de6); };
            c.w64(1, t32);
            blk = 21; continue;
        }
        21 => {
            let r33 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r33);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70007f9df1); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 30; continue;
        }
        23 => {
            let r34 = c.sub(c.r32(8), 0x20000000u64, 32);
            c.w32(8, r34);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70007f9e04); }
            let Some(t35) = c.ld8(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x70007f9e08); };
            c.w32(8, t35);
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70007f9e1a); };
            c.w64(12, t36);
            blk = 25; continue;
        }
        25 => {
            let r37 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r37);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(12)) { return RegfileExit::fault(0x70007f9e25); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ne() { blk = 25; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9e34); };
            c.w64(9, t38);
            blk = 28; continue;
        }
        28 => {
            let r39 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r39);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x70007f9e3f); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 28; continue; }
            blk = 29; continue;
        }
        29 => {
            let r40 = c.add(c.r32(11), 0xe0000000u64, 32);
            c.w32(11, r40);
            if c.cc_e() { blk = 41; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70007f9e59);
        }
        31 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70007f9e5a); }
            let Some(t41) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70007f9e5b); };
            if !c.push64(t41) { return RegfileExit::fault(0x70007f9e5b); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x70007f9e5f); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f9e6b); }
            return RegfileExit::jump(0x700086ed74);
        }
        32 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9e75); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f9e79); }
            let r43 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r43);
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007f9e84); }
            return c.call(0x70007febf4, 0x70007f9e8d, 0x70007f9e88);
        }
        33 => {
            c.w64(8, c.r64(0));
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f9e90); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f9e94); };
            c.w64(7, t45);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f9e9b); };
            c.w64(11, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9ea5); }
            blk = 8; continue;
        }
        34 => {
            return RegfileExit::fault(0x70007f9eab);
        }
        35 => {
            return RegfileExit::fault(0x70007f9eac);
        }
        36 => {
            return RegfileExit::fault(0x70007f9ead);
        }
        37 => {
            return RegfileExit::fault(0x70007f9eae);
        }
        38 => {
            return RegfileExit::fault(0x70007f9eaf);
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9eb0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f9eb4); }
            let r47 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r47);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70007f9eba); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70007f9ebe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70007f9ec2); }
            return c.call(0x70007febf4, 0x70007f9ecb, 0x70007f9ec6);
        }
        40 => {
            c.w64(7, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007f9ece); };
            c.w64(1, t48);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007f9ed4); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007f9ed8); };
            c.w64(2, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9edf); }
            blk = 18; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007f9ee5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007f9ee9); }
            let r51 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r51);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007f9eef); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007f9ef0); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70007f9ef4); };
            c.w64(3, t52);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70007f9f05, 0x70007f9f00);
        }
        42 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007f9f05); };
            c.w64(7, t53);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007f9f0c); }
            blk = 30; continue;
        }
        43 => {
            return RegfileExit::jump(0x70007f9f14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007f9f34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9f34); };
            c.w32(1, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x218)) else { return RegfileExit::fault(0x70007f9f37); };
            c.cmp(t1, c.r32(1), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x228)) else { return RegfileExit::fault(0x70007f9f40); };
            c.cmp(t2, c.r32(1), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1f8)) else { return RegfileExit::fault(0x70007f9f49); };
            c.w64(1, t3);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f9f50); }
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x228)) else { return RegfileExit::fault(0x70007f9f55); };
            c.w64(1, t4);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f9f5c); }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70007f9f5f); };
            c.w32(1, t5);
            if let Some(sr) = c.shr(c.r64(1), 0xeu64, 64) { c.w64(1, sr); }
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007f9f66); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let r8 = c.r64(1) & 0x1fu64;
            c.w64(1, r8);
            c.logic(r8, 64);
            let r9 = c.add(c.r64(1), 0xb7u64, 64);
            c.w64(1, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007f9f77); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9f7d); };
            c.w32(8, t12);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_b() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70007f9f8c);
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007f9f8d); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007f9f94); };
            c.w32(7, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007f9f98); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70007f9f9f); };
            c.w64(9, t17);
            c.w64(2, c.r64(7).wrapping_mul(4));
            c.w64(7, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007f9fae); };
            c.w64(11, t18);
            let Some(t19) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007f9fb5); };
            c.cmp(t19, c.r64(7), 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x70007f9fba); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007f9fc1); }
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            c.cmp(c.r64(8), 0x1bu64, 64);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007f9fce); };
            c.w64(9, t20);
            blk = 10; continue;
        }
        10 => {
            let r21 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r21);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x70007f9fd9); }
            c.cmp(c.r64(8), 0x1bu64, 64);
            if c.cc_ne() { blk = 10; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007f9fe3); };
            c.w64(8, t22);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70007f9fea); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70007f9fee); }
            let Some(t23) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70007f9ff2); };
            c.logic(t23 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70007f9ff8); };
            c.w64(1, t24);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x70007f9fff); }
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70007fa002); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70007fa005); }
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x70007fa008); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70007fa00b); };
            c.w64(3, t25);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70007fa012); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70007fa018);
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa019); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fa01d); }
            let r26 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70007fa023); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70007fa027); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70007fa02b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70007fa02f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70007fa033); }
            return c.call(0x70007febf4, 0x70007fa03c, 0x70007fa037);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fa03f); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fa043); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fa047); };
            c.w64(3, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fa04b); };
            c.w64(1, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fa04f); };
            c.w64(2, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa056); }
            blk = 8; continue;
        }
        16 => {
            return RegfileExit::jump(0x70007fa060);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa074); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70007fa078); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70007fa07e); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fa084); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70007fa08b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70007fa08f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70007fa093); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70007fa097); }
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fa09b); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fa0a8); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70007fa0ae); }
            c.w64(0, c.r64(1));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x600)) else { return RegfileExit::fault(0x70007fa0b5); };
            c.w64(3, t8);
            return c.call(0x7000870914, 0x70007fa0c1, 0x70007fa0bc);
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fa0c1); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70007fa0ce); };
            c.cmp(t10, c.r32(0), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fa0db); };
            c.w64(6, t11);
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fa0df); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70007fa0e5); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fa0eb); };
            c.w32(8, t16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70007fa0fb);
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70007fa0fc); };
            c.w32(1, t17);
            let Some(t18) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fa102); };
            c.w64(8, (t18 as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70007fa106); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 36; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fa121); };
            c.w64(9, t19);
            let r20 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r20);
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(12, c.r32(12));
            let r21 = c.imul2(c.r32(12), 0x4u64, 32);
            c.w32(12, r21);
            if c.cc_o() { blk = 23; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0x8u64);
            let r22 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r22);
            if c.cc_o() { blk = 22; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fa150); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70007fa157); };
            c.w64(3, t24);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fa15e); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70007fa165); };
            c.cmp(t26, c.r64(2), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 34; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fa17b); };
            c.w64(12, t27);
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70007fa182); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70007fa18a); };
            c.w64(3, t28);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70007fa191); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70007fa196); }
            c.w64(9, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70007fa1a6); };
            c.w64(3, t29);
            blk = 15; continue;
        }
        14 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70007fa1af); }
            let r30 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r30);
            let r31 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r31);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(2, c.r64(5));
            let r32 = c.add(c.r64(2), 0x18u64, 64);
            c.w64(2, r32);
            let Some(t33) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fa1d2); };
            c.w64(2, t33);
            blk = 14; continue;
        }
        19 => {
            c.w64(2, c.r64(3));
            blk = 14; continue;
        }
        20 => {
            return RegfileExit::fault(0x70007fa1dd);
        }
        21 => {
            blk = 25; continue;
        }
        22 => {
            return RegfileExit::fault(0x70007fa1e0);
        }
        23 => {
            return RegfileExit::fault(0x70007fa1e1);
        }
        24 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007fa1e2); };
            c.w64(9, t34);
            c.w64(12, c.r64(9));
            blk = 25; continue;
        }
        25 => {
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70007fa1ec); };
            c.w32(9, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70007fa1f1); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70007fa1f8); };
            c.w64(15, t37);
            c.w64(3, c.r64(15).wrapping_add(0x10));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70007fa1ff); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70007fa206); };
            c.cmp(t39, c.r64(3), 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70007fa20f); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 27; continue;
        }
        27 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70007fa216); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70007fa21a); };
            c.w64(1, t40);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70007fa221); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70007fa225); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70007fa229); }
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70007fa22d); };
            if !c.push64(t41) { return RegfileExit::fault(0x70007fa22d); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70007fa230); }
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70007fa232); };
            if !c.push64(t42) { return RegfileExit::fault(0x70007fa232); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70007fa235); };
            if !c.push64(t43) { return RegfileExit::fault(0x70007fa235); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70007fa248, 0x70007fa243);
        }
        28 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70007fa250); };
            c.w32(1, t44);
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            let Some(t46) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70007fa256); };
            c.cmp(t46, 0xbau64, 16);
            if c.cc_b() { blk = 40; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fa262); };
            c.w64(1, t47);
            let r48 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r48);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa26d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70007fa26e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70007fa274); }
            return c.ret(0x0, 0x70007fa276);
        }
        31 => {
            let Some(t49) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fa277); };
            c.w64(5, t49);
            let Some(t50) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x70007fa27b); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa280); };
            c.w64(2, t51);
            let Some(t52) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fa285); };
            c.w64(0, t52);
            let Some(t53) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70007fa28a); };
            c.w64(6, t53);
            let r54 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r54);
            return RegfileExit::jump(0x70007f7214);
        }
        32 => {
            c.w32(1, 0x130u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fa29d); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x600)) else { return RegfileExit::fault(0x70007fa29e); };
            c.w64(3, t55);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fa2a5); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007fa2a6); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70007fa2b7, 0x70007fa2b2);
        }
        33 => {
            return RegfileExit::fault(0x70007fa2b7);
        }
        34 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70007fa2b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70007fa2bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70007fa2c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70007fa2c4); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70007fa2d0, 0x70007fa2cb);
        }
        35 => {
            c.w64(12, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fa2d3); };
            c.w64(6, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70007fa2d7); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007fa2db); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007fa2df); };
            c.w64(1, t60);
            c.w32(15, 0x8u64);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fa2e9); };
            c.w64(11, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007fa2ed); };
            c.w64(9, t62);
            blk = 13; continue;
        }
        36 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70007fa2f6); };
            c.w64(3, t63);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70007fa306, 0x70007fa301);
        }
        37 => {
            return RegfileExit::fault(0x70007fa306);
        }
        38 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70007fa30c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70007fa310); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70007fa314); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70007fa318); }
            return c.call(0x70007febb4, 0x70007fa321, 0x70007fa31c);
        }
        39 => {
            c.w64(11, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fa324); };
            c.w64(6, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70007fa328); };
            c.w64(0, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70007fa32c); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70007fa330); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70007fa334); };
            c.w64(1, t68);
            blk = 27; continue;
        }
        40 => {
            c.w32(1, 0xfcu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70007fa342); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70007fa343); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70007fa344); };
            c.w64(3, t69);
            c.w32(0, 0x2u64);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70007fa350); };
            c.w64(6, t70);
            return c.call(0x700086ed74, 0x70007fa359, 0x70007fa354);
        }
        41 => {
            return RegfileExit::fault(0x70007fa359);
        }
        42 => {
            return RegfileExit::jump(0x70007fa35c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2u64) { return RegfileExit::fault(0x70007fa378); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fa37e); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fa380); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fa382); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fa384); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa386); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fa387); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fa388); }
            let r1 = c.sub(c.r64(4), 0xa0u64, 64);
            c.w64(4, r1);
            if !c.mov128_mr(c.r64(4), 6) { return RegfileExit::fault(0x70007fa390); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 7) { return RegfileExit::fault(0x70007fa396); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 8) { return RegfileExit::fault(0x70007fa39d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 9) { return RegfileExit::fault(0x70007fa3a4); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 10) { return RegfileExit::fault(0x70007fa3ab); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 11) { return RegfileExit::fault(0x70007fa3b2); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fa3b9); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 13) { return RegfileExit::fault(0x70007fa3c0); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 14) { return RegfileExit::fault(0x70007fa3c7); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 15) { return RegfileExit::fault(0x70007fa3d1); }
            c.w64(13, c.r64(1));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa3de); };
            c.w64(14, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x3048)) else { return RegfileExit::fault(0x70007fa3e2); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fa3e2); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x70007fa3e9); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fa3f4); };
            c.w64(10, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(10)) { return RegfileExit::fault(0x70007fa3fb); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x3070)) else { return RegfileExit::fault(0x70007fa3ff); };
            c.w64(0, t5);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(0x2u64) { return RegfileExit::fault(0x70007fa40f); }
            c.w64(0, c.r64(5));
            if !c.st64(c.r64(13).wrapping_add(0x3070), c.r64(0)) { return RegfileExit::fault(0x70007fa414); }
            blk = 3; continue;
        }
        2 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa420); }
            blk = 3; continue;
        }
        3 => {
            blk = 5; continue;
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x2ff8), c.r64(0)) { return RegfileExit::fault(0x70007fa427); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x70007fa42e); };
            c.w64(0, t6);
            blk = 7; continue;
        }
        5 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa43a); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x3050)) else { return RegfileExit::fault(0x70007fa43c); };
            if !c.push64(t7) { return RegfileExit::fault(0x70007fa43c); }
            if !c.st64(c.r64(13).wrapping_add(0x3050), c.r64(4)) { return RegfileExit::fault(0x70007fa443); }
            return c.call(0x70007fa7f4, 0x70007fa44f, 0x70007fa44a);
        }
        6 => {
            let sp8 = c.r64(4);
            let Some(pv9) = c.ld64(sp8) else { return RegfileExit::fault(0x70007fa44f); };
            c.set_rsp(sp8.wrapping_add(8));
            let pea10 = c.r64(13).wrapping_add(0x3050);
            if !c.st64(pea10, pv9) { c.set_rsp(sp8); return RegfileExit::fault(0x70007fa44f); }
            let r11 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            blk = 7; continue;
        }
        7 => {
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa45a); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(13).wrapping_add(0x3070));
            if !c.st64(c.r64(10), 0x0u64) { return RegfileExit::fault(0x70007fa46c); }
            blk = 9; continue;
        }
        9 => {
            let sp12 = c.r64(4);
            let Some(pv13) = c.ld64(sp12) else { return RegfileExit::fault(0x70007fa473); };
            c.set_rsp(sp12.wrapping_add(8));
            let pea14 = c.r64(13).wrapping_add(0x3048);
            if !c.st64(pea14, pv13) { c.set_rsp(sp12); return RegfileExit::fault(0x70007fa473); }
            if !c.mov128_rm(6, c.r64(4)) { return RegfileExit::fault(0x70007fa47a); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007fa480); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007fa487); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007fa48e); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007fa495); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007fa49c); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007fa4a3); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007fa4aa); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007fa4b1); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fa4bb); }
            let r15 = c.add(c.r64(4), 0xa0u64, 64);
            c.w64(4, r15);
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa4cc); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fa4cd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fa4ce); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fa4cf); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fa4d1); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fa4d3); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fa4d5); }
            let r16 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r16);
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa4db); }
            return c.ret(0x0, 0x70007fa4dc);
        }
        10 => {
            return RegfileExit::fault(0x70007fa4dd);
        }
        11 => {
            return RegfileExit::fault(0x70007fa4de);
        }
        12 => {
            return RegfileExit::fault(0x70007fa4df);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x4u64) { return RegfileExit::fault(0x70007fa4f8); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fa4fe); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fa500); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fa502); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fa504); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa506); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fa507); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fa508); }
            let r1 = c.sub(c.r64(4), 0xa0u64, 64);
            c.w64(4, r1);
            if !c.mov128_mr(c.r64(4), 6) { return RegfileExit::fault(0x70007fa510); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 7) { return RegfileExit::fault(0x70007fa516); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 8) { return RegfileExit::fault(0x70007fa51d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 9) { return RegfileExit::fault(0x70007fa524); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 10) { return RegfileExit::fault(0x70007fa52b); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 11) { return RegfileExit::fault(0x70007fa532); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fa539); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 13) { return RegfileExit::fault(0x70007fa540); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 14) { return RegfileExit::fault(0x70007fa547); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 15) { return RegfileExit::fault(0x70007fa551); }
            c.w64(13, c.r64(1));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa55e); };
            c.w64(14, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x3048)) else { return RegfileExit::fault(0x70007fa562); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fa562); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x70007fa569); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fa574); };
            c.w64(10, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(10)) { return RegfileExit::fault(0x70007fa57b); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x3070)) else { return RegfileExit::fault(0x70007fa57f); };
            c.w64(0, t5);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(0x2u64) { return RegfileExit::fault(0x70007fa58f); }
            c.w64(0, c.r64(5));
            if !c.st64(c.r64(13).wrapping_add(0x3070), c.r64(0)) { return RegfileExit::fault(0x70007fa594); }
            blk = 3; continue;
        }
        2 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa5a0); }
            blk = 3; continue;
        }
        3 => {
            blk = 5; continue;
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x2ff8), c.r64(0)) { return RegfileExit::fault(0x70007fa5a7); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x70007fa5ae); };
            c.w64(0, t6);
            blk = 7; continue;
        }
        5 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa5ba); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x3050)) else { return RegfileExit::fault(0x70007fa5bc); };
            if !c.push64(t7) { return RegfileExit::fault(0x70007fa5bc); }
            if !c.st64(c.r64(13).wrapping_add(0x3050), c.r64(4)) { return RegfileExit::fault(0x70007fa5c3); }
            return c.call(0x70007fa874, 0x70007fa5cf, 0x70007fa5ca);
        }
        6 => {
            let sp8 = c.r64(4);
            let Some(pv9) = c.ld64(sp8) else { return RegfileExit::fault(0x70007fa5cf); };
            c.set_rsp(sp8.wrapping_add(8));
            let pea10 = c.r64(13).wrapping_add(0x3050);
            if !c.st64(pea10, pv9) { c.set_rsp(sp8); return RegfileExit::fault(0x70007fa5cf); }
            let r11 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            blk = 7; continue;
        }
        7 => {
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa5da); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(13).wrapping_add(0x3070));
            if !c.st64(c.r64(10), 0x0u64) { return RegfileExit::fault(0x70007fa5ec); }
            blk = 9; continue;
        }
        9 => {
            let sp12 = c.r64(4);
            let Some(pv13) = c.ld64(sp12) else { return RegfileExit::fault(0x70007fa5f3); };
            c.set_rsp(sp12.wrapping_add(8));
            let pea14 = c.r64(13).wrapping_add(0x3048);
            if !c.st64(pea14, pv13) { c.set_rsp(sp12); return RegfileExit::fault(0x70007fa5f3); }
            if !c.mov128_rm(6, c.r64(4)) { return RegfileExit::fault(0x70007fa5fa); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007fa600); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007fa607); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007fa60e); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007fa615); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007fa61c); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007fa623); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007fa62a); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007fa631); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fa63b); }
            let r15 = c.add(c.r64(4), 0xa0u64, 64);
            c.w64(4, r15);
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa64c); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fa64d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fa64e); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fa64f); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fa651); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fa653); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fa655); }
            let r16 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r16);
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa65b); }
            return c.ret(0x0, 0x70007fa65c);
        }
        10 => {
            return RegfileExit::fault(0x70007fa65d);
        }
        11 => {
            return RegfileExit::fault(0x70007fa65e);
        }
        12 => {
            return RegfileExit::fault(0x70007fa65f);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x2u64) { return RegfileExit::fault(0x70007fa678); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70007fa67e); }
            if !c.push64(c.r64(13)) { return RegfileExit::fault(0x70007fa680); }
            if !c.push64(c.r64(14)) { return RegfileExit::fault(0x70007fa682); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70007fa684); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa686); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70007fa687); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70007fa688); }
            let r1 = c.sub(c.r64(4), 0xa0u64, 64);
            c.w64(4, r1);
            if !c.mov128_mr(c.r64(4), 6) { return RegfileExit::fault(0x70007fa690); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x10), 7) { return RegfileExit::fault(0x70007fa696); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x20), 8) { return RegfileExit::fault(0x70007fa69d); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x30), 9) { return RegfileExit::fault(0x70007fa6a4); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x40), 10) { return RegfileExit::fault(0x70007fa6ab); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x50), 11) { return RegfileExit::fault(0x70007fa6b2); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x60), 12) { return RegfileExit::fault(0x70007fa6b9); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x70), 13) { return RegfileExit::fault(0x70007fa6c0); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x80), 14) { return RegfileExit::fault(0x70007fa6c7); }
            if !c.mov128_mr(c.r64(4).wrapping_add(0x90), 15) { return RegfileExit::fault(0x70007fa6d1); }
            c.w64(13, c.r64(1));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa6de); };
            c.w64(14, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x3048)) else { return RegfileExit::fault(0x70007fa6e2); };
            if !c.push64(t3) { return RegfileExit::fault(0x70007fa6e2); }
            if !c.st64(c.r64(13).wrapping_add(0x3048), 0x0u64) { return RegfileExit::fault(0x70007fa6e9); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fa6f4); };
            c.w64(10, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(10)) { return RegfileExit::fault(0x70007fa6fb); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x3070)) else { return RegfileExit::fault(0x70007fa6ff); };
            c.w64(0, t5);
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(0x2u64) { return RegfileExit::fault(0x70007fa70f); }
            c.w64(0, c.r64(5));
            if !c.st64(c.r64(13).wrapping_add(0x3070), c.r64(0)) { return RegfileExit::fault(0x70007fa714); }
            blk = 3; continue;
        }
        2 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa720); }
            blk = 3; continue;
        }
        3 => {
            blk = 5; continue;
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x2ff8), c.r64(0)) { return RegfileExit::fault(0x70007fa727); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x180)) else { return RegfileExit::fault(0x70007fa72e); };
            c.w64(0, t6);
            blk = 7; continue;
        }
        5 => {
            if !c.push64(0x0u64) { return RegfileExit::fault(0x70007fa73a); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x3050)) else { return RegfileExit::fault(0x70007fa73c); };
            if !c.push64(t7) { return RegfileExit::fault(0x70007fa73c); }
            if !c.st64(c.r64(13).wrapping_add(0x3050), c.r64(4)) { return RegfileExit::fault(0x70007fa743); }
            return c.call(0x700081f434, 0x70007fa74f, 0x70007fa74a);
        }
        6 => {
            let sp8 = c.r64(4);
            let Some(pv9) = c.ld64(sp8) else { return RegfileExit::fault(0x70007fa74f); };
            c.set_rsp(sp8.wrapping_add(8));
            let pea10 = c.r64(13).wrapping_add(0x3050);
            if !c.st64(pea10, pv9) { c.set_rsp(sp8); return RegfileExit::fault(0x70007fa74f); }
            let r11 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            blk = 7; continue;
        }
        7 => {
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa75a); }
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(10, c.r64(13).wrapping_add(0x3070));
            if !c.st64(c.r64(10), 0x0u64) { return RegfileExit::fault(0x70007fa76c); }
            blk = 9; continue;
        }
        9 => {
            let sp12 = c.r64(4);
            let Some(pv13) = c.ld64(sp12) else { return RegfileExit::fault(0x70007fa773); };
            c.set_rsp(sp12.wrapping_add(8));
            let pea14 = c.r64(13).wrapping_add(0x3048);
            if !c.st64(pea14, pv13) { c.set_rsp(sp12); return RegfileExit::fault(0x70007fa773); }
            if !c.mov128_rm(6, c.r64(4)) { return RegfileExit::fault(0x70007fa77a); }
            if !c.mov128_rm(7, c.r64(4).wrapping_add(0x10)) { return RegfileExit::fault(0x70007fa780); }
            if !c.mov128_rm(8, c.r64(4).wrapping_add(0x20)) { return RegfileExit::fault(0x70007fa787); }
            if !c.mov128_rm(9, c.r64(4).wrapping_add(0x30)) { return RegfileExit::fault(0x70007fa78e); }
            if !c.mov128_rm(10, c.r64(4).wrapping_add(0x40)) { return RegfileExit::fault(0x70007fa795); }
            if !c.mov128_rm(11, c.r64(4).wrapping_add(0x50)) { return RegfileExit::fault(0x70007fa79c); }
            if !c.mov128_rm(12, c.r64(4).wrapping_add(0x60)) { return RegfileExit::fault(0x70007fa7a3); }
            if !c.mov128_rm(13, c.r64(4).wrapping_add(0x70)) { return RegfileExit::fault(0x70007fa7aa); }
            if !c.mov128_rm(14, c.r64(4).wrapping_add(0x80)) { return RegfileExit::fault(0x70007fa7b1); }
            if !c.mov128_rm(15, c.r64(4).wrapping_add(0x90)) { return RegfileExit::fault(0x70007fa7bb); }
            let r15 = c.add(c.r64(4), 0xa0u64, 64);
            c.w64(4, r15);
            if !c.pop64_reg(3) { return RegfileExit::fault(0x70007fa7cc); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x70007fa7cd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70007fa7ce); }
            if !c.pop64_reg(15) { return RegfileExit::fault(0x70007fa7cf); }
            if !c.pop64_reg(14) { return RegfileExit::fault(0x70007fa7d1); }
            if !c.pop64_reg(13) { return RegfileExit::fault(0x70007fa7d3); }
            if !c.pop64_reg(12) { return RegfileExit::fault(0x70007fa7d5); }
            let r16 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r16);
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa7db); }
            return c.ret(0x0, 0x70007fa7dc);
        }
        10 => {
            return RegfileExit::fault(0x70007fa7dd);
        }
        11 => {
            return RegfileExit::fault(0x70007fa7de);
        }
        12 => {
            return RegfileExit::fault(0x70007fa7df);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa7f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(8));
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa7f9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fa7fd); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fa7ff); };
            c.w64(6, t0);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa806); }
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fa807); };
            c.w64(10, t1);
            let Some(t2) = c.ld64(c.r64(10).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fa80b); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(10).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa80f); };
            c.w64(3, t3);
            c.w64(10, c.r64(4));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fa816); };
            let r5 = c.sub(c.r64(10), t4, 64);
            c.w64(10, r5);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            blk = 4; continue;
        }
        2 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fa827); };
            c.w64(3, t6);
            return c.call(0x700086ed74, 0x70007fa833, 0x70007fa82e);
        }
        3 => {
            return RegfileExit::fault(0x70007fa833);
        }
        4 => {
            c.w64(1, c.r64(0));
            blk = 6; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fa839); };
            c.w64(10, t7);
            let Some(t8) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70007fa83d); };
            if !c.push64(t8) { return RegfileExit::fault(0x70007fa83d); }
            blk = 6; continue;
        }
        6 => {
            let r9 = c.dec(c.r64(1), 64);
            c.w64(1, r9);
            if c.cc_ge() { blk = 5; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fa845); }
            return c.call(0x70007f2274, 0x70007fa84c, 0x70007fa847);
        }
        8 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa84f); }
            return c.ret(0x0, 0x70007fa850);
        }
        9 => {
            return RegfileExit::fault(0x70007fa851);
        }
        10 => {
            return RegfileExit::fault(0x70007fa852);
        }
        11 => {
            return RegfileExit::fault(0x70007fa853);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70007fa874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(8));
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70007fa879); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x70007fa87d); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x70007fa87f); };
            c.w64(6, t0);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70007fa886); }
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70007fa887); };
            c.w64(10, t1);
            let Some(t2) = c.ld64(c.r64(10).wrapping_add(0x30)) else { return RegfileExit::fault(0x70007fa88b); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(10).wrapping_add(0x38)) else { return RegfileExit::fault(0x70007fa88f); };
            c.w64(3, t3);
            c.w64(10, c.r64(4));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x50)) else { return RegfileExit::fault(0x70007fa896); };
            let r5 = c.sub(c.r64(10), t4, 64);
            c.w64(10, r5);
            if let Some(sr) = c.sar(c.r64(10), 0x3u64, 64) { c.w64(10, sr); }
            c.cmp(c.r64(10), c.r64(0), 64);
            if c.cc_le() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            blk = 4; continue;
        }
        2 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70007fa8a7); };
            c.w64(3, t6);
            return c.call(0x700086ed74, 0x70007fa8b3, 0x70007fa8ae);
        }
        3 => {
            return RegfileExit::fault(0x70007fa8b3);
        }
        4 => {
            c.w64(1, c.r64(0));
            blk = 6; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x70007fa8b9); };
            c.w64(10, t7);
            let Some(t8) = c.ld64(c.r64(10)) else { return RegfileExit::fault(0x70007fa8bd); };
            if !c.push64(t8) { return RegfileExit::fault(0x70007fa8bd); }
            blk = 6; continue;
        }
        6 => {
            let r9 = c.dec(c.r64(1), 64);
            c.w64(1, r9);
            if c.cc_ge() { blk = 5; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70007fa8c5); }
            return c.call(0x70007f7214, 0x70007fa8cc, 0x70007fa8c7);
        }
        8 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70007fa8cf); }
            return c.ret(0x0, 0x70007fa8d0);
        }
        9 => {
            return RegfileExit::fault(0x70007fa8d1);
        }
        10 => {
            return RegfileExit::fault(0x70007fa8d2);
        }
        11 => {
            return RegfileExit::fault(0x70007fa8d3);
        }
            _ => unreachable!(),
        }
    }
}
