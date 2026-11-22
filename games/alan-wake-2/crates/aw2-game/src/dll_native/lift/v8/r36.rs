#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000921bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921bf7); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000921bfb); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921c02); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921c09); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000921c0d); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000921c16); };
            c.w64(11, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000921c1d); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000921c24); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000921c31); };
            c.w64(3, (t10 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000921c3d); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let r13 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921c57); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000921c5d); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let r17 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921c6e); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921c73); };
            c.w64(1, t19);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000921c79); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921c7d); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000921c84); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921c8b); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000921c9e);
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x7000921c9f); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921cac); };
            c.w32(0, t28);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r29 = c.r64(3) & 0xfffu64;
            c.w64(3, r29);
            c.logic(r29, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 253; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921cd0); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921cd8); };
            c.w32(0, t32);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r33 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r33);
            c.logic(r33, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000921cf2);
        }
        12 => {
            let r34 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921cf7); };
            c.w32(0, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921cfc); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921d04); };
            c.w64(1, t38);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000921d0d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000921d11); }
            let r39 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000921d17); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000921d1b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000921d1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000921d23); }
            blk = 251; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000921d2c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000921d30); }
            let r40 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000921d36); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000921d3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000921d42); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000921d46); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000921d4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000921d4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000921d52); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000921d56); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t44) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000921d63); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921d67); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000921d6d); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921d73); };
            c.w32(6, t49);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000921d7c); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x7000921d86);
        }
        17 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000921d87); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000921d8e); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x7000921d98);
        }
        19 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000921d99); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000921da9); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000921dac); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921db5); };
            c.w32(6, t56);
            let r57 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r57);
            c.w32(3, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t58) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921dc1); };
            c.cmp(t58, c.r32(3), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921dca); };
            c.w32(3, t59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921dcd); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000921dd9); };
            c.cmp(t62, c.r32(6), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 26; continue;
        }
        26 => {
            let r63 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r63);
            c.logic(r63, 64);
            blk = 28; continue;
        }
        27 => {
            c.w64(6, c.r64(1));
            blk = 28; continue;
        }
        28 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 36; continue;
        }
        29 => {
            let r64 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r64);
            let Some(t65) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921e06); };
            c.w32(8, t65);
            let Some(t66) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921e0b); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921e0f); };
            c.w64(5, t67);
            c.w64(0, c.r64(6));
            let r68 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r68);
            return RegfileExit::jump(c.r64(1));
        }
        30 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r69 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r69);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921e27); };
            c.w32(6, t70);
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921e2c); };
            c.w32(9, t71);
            let r72 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921e34); };
            c.w32(6, t73);
            let r74 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r74);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921e3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000921e40); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000921e59); };
            c.w64(3, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000921e5d); };
            c.logic(t76 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let r77 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r77);
            blk = 33; continue;
        }
        32 => {
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000921e6c); };
            c.w32(7, t78);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000921e6f); };
            c.w64(7, t79);
            blk = 33; continue;
        }
        33 => {
            return c.call(c.r64(7), 0x7000921e79, 0x7000921e77);
        }
        34 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921e79); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921e7d); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000921e81); };
            c.w64(9, t82);
            let Some(t83) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921e85); };
            c.w32(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921e8a); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921e8e); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921e92); };
            c.w64(5, t86);
            c.w64(12, c.r64(8));
            let r87 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r87);
            return RegfileExit::jump(c.r64(1));
        }
        35 => {
            c.w64(3, c.r64(1));
            c.w64(6, c.r64(7));
            blk = 36; continue;
        }
        36 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(2, c.r64(6));
            let r88 = c.r64(2) & 0xfu64;
            c.w64(2, r88);
            c.logic(r88, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000921eb2); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_e() { blk = 181; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(2), 0x5u64, 64);
            if c.cc_e() { blk = 180; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_e() { blk = 179; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_e() { blk = 118; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(2), 0x6u64, 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(2), 0x8u64, 64);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(2), 0x9u64, 64);
            if c.cc_e() { blk = 100; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 114; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0xeu64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r89 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r89);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000921f41); };
            c.w32(2, t90);
            let Some(t91) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921f46); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921f4e); };
            c.w32(2, t93);
            let r94 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r94);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000921f55); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000921f59); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000921f5a); };
            if !c.push64(t95) { return RegfileExit::fault(0x7000921f5a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000921f5d); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000921f5e); };
            if !c.push64(t96) { return RegfileExit::fault(0x7000921f5e); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000921f61); };
            if !c.push64(t97) { return RegfileExit::fault(0x7000921f61); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000921f64); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000921f68); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000921f6f); };
            c.w64(6, t99);
            c.w32(0, 0x5u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000921f80, 0x7000921f7b);
        }
        49 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921f80); };
            c.w64(8, t100);
            let Some(t101) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000921f84); };
            c.w64(8, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000921f88); };
            c.w64(9, t102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000921f8c); };
            c.w32(11, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000921f91); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921f95); };
            c.w64(1, t105);
            let Some(t106) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921f99); };
            c.w64(5, t106);
            c.w64(12, c.r64(8));
            let r107 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r107);
            return RegfileExit::jump(c.r64(1));
        }
        50 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000921fa6); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921fad); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921fb4); };
            c.w32(0, t112);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r113 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r113);
            c.logic(r113, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x7000921fce);
        }
        52 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000921fcf); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000921fd7); };
            c.w32(0, t116);
            let r117 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r117);
            c.w64(11, c.r64(9).wrapping_add(0x9));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000921fe2); }
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000921fe6); };
            c.cmp(t118, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t119) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000921fef); };
            c.w32(8, t119);
            let Some(t120) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000921ff4); };
            c.w64(1, t120);
            let Some(t121) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000921ff8); };
            c.w64(5, t121);
            c.w64(9, c.r64(11));
            let r122 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r122);
            return RegfileExit::jump(c.r64(1));
        }
        54 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r123 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r123);
            let Some(t124) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092200b); };
            c.w32(3, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922010); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922018); };
            c.w32(3, t127);
            let r128 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r128);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092201f); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000922028); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000922029); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700092202a); };
            c.w64(3, t129);
            c.w32(0, 0x2u64);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922036); };
            c.w64(6, t130);
            return c.call(0x700086ed74, 0x700092203f, 0x700092203a);
        }
        55 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092203f); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922043); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000922047); };
            c.w64(9, t133);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092204b); };
            c.w32(11, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922050); };
            c.w64(15, t135);
            let Some(t136) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922054); };
            c.w64(1, t136);
            let Some(t137) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922058); };
            c.w64(5, t137);
            c.w64(12, c.r64(8));
            let r138 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r138);
            return RegfileExit::jump(c.r64(1));
        }
        56 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r139 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r139);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t140) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922070); };
            c.w32(2, t140);
            let Some(t141) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922075); };
            c.w32(1, t141);
            let r142 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r142);
            let Some(t143) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092207d); };
            c.w32(2, t143);
            let r144 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r144);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000922084); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000922088); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092208c); };
            c.w64(6, t145);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000922096); };
            c.w64(1, t146);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x70009220a7, 0x70009220a2);
        }
        57 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009220a7); };
            c.w64(8, t147);
            let Some(t148) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009220ab); };
            c.w64(8, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009220af); };
            c.w64(9, t149);
            let Some(t150) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009220b3); };
            c.w32(11, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009220b8); };
            c.w64(15, t151);
            let Some(t152) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009220bc); };
            c.w64(1, t152);
            let Some(t153) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009220c0); };
            c.w64(5, t153);
            c.w64(12, c.r64(8));
            let r154 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r154);
            return RegfileExit::jump(c.r64(1));
        }
        58 => {
            blk = 114; continue;
        }
        59 => {
            let Some(t155) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009220d2); };
            c.w32(11, t155);
            let r156 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r156);
            let Some(t157) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009220d9); };
            c.w32(2, t157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009220dc); };
            c.cmp(t158, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009220e9); };
            c.w32(1, t159);
            let Some(t160) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009220ee); };
            c.w32(6, t160);
            let r161 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r161);
            let Some(t162) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009220f6); };
            c.w32(1, t162);
            let r163 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r163);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t164) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922106); };
            c.w32(2, t164);
            let r165 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r165);
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700092210d); };
            c.cmp(t166, c.r32(2), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t167) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092211a); };
            c.w32(2, t167);
            let r168 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r168);
            let Some(t169) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922121); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000922127); };
            c.logic(t171 & 0x2u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000922131); };
            c.cmp(t172, c.r32(6), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(11, c.r64(0));
            blk = 98; continue;
        }
        65 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000922142); };
            c.w64(0, t173);
            let Some(t174) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x7000922149); };
            c.cmp(t174, 0x20u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t175) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000922152); };
            c.w32(0, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000922155); };
            c.cmp(t176, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t177) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922162); };
            c.w32(0, t177);
            let r178 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r178);
            let Some(t179) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922168); };
            c.w32(1, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092216b); };
            c.cmp(t180, c.r32(1), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t181) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000922174); };
            c.w32(0, t181);
            let r182 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r182);
            let Some(t183) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092217a); };
            c.w32(1, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092217d); };
            c.cmp(t184, c.r32(1), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092218e); };
            c.w32(1, t185);
            let r186 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r186);
            let Some(t187) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922194); };
            c.w32(6, t187);
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 75; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w32(0, c.r16(6));
            c.cmp(c.r32(0), 0x82au64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009221ad); };
            c.cmp(t188, c.r32(1), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 76; continue;
        }
        74 => {
            c.w64(3, c.r64(11));
            blk = 76; continue;
        }
        75 => {
            c.w64(3, c.r64(0));
            blk = 76; continue;
        }
        76 => {
            let Some(t189) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009221c4); };
            c.w32(11, t189);
            let r190 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r190);
            let Some(t191) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009221cb); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            let Some(t193) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009221d2); };
            c.w32(0, t193);
            let r194 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r194);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 251; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r195 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r195);
            let Some(t196) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009221eb); };
            c.w32(6, t196);
            let r197 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r197);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009221f2); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009221f6); };
            if !c.push64(t198) { return RegfileExit::fault(0x70009221f6); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000922200, 0x70009221fb);
        }
        78 => {
            c.w64(11, c.r64(0));
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922203); };
            c.w64(15, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922207); };
            c.w64(9, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092220b); };
            c.w64(8, t201);
            blk = 98; continue;
        }
        79 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r202 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r202);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092221b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092221f); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000922220); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000922221); };
            if !c.push64(t203) { return RegfileExit::fault(0x7000922221); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922224); };
            c.w64(6, t204);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000922228); };
            c.w64(3, t205);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700092223c, 0x7000922237);
        }
        80 => {
            c.w64(11, c.r64(0));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092223f); };
            c.w64(15, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922243); };
            c.w64(9, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922247); };
            c.w64(8, t208);
            blk = 98; continue;
        }
        81 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r209 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r209);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000922259); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092225d); };
            if !c.push64(t210) { return RegfileExit::fault(0x700092225d); }
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922260); };
            c.w64(6, t211);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x7000922271, 0x700092226c);
        }
        82 => {
            c.w64(11, c.r64(0));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922274); };
            c.w64(15, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922278); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092227c); };
            c.w64(8, t214);
            blk = 98; continue;
        }
        83 => {
            let Some(t215) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922285); };
            c.w32(0, t215);
            let r216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r216);
            let Some(t217) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092228b); };
            c.w32(1, t217);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 97; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 90; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 251; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009222b0); };
            c.w32(0, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009222b4); };
            c.cmp(t219, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t220) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009222cb); };
            c.cmp(t220, 0x40u64, 16);
            if c.cc_ae() { blk = 251; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t221) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009222d6); };
            c.w32(11, t221);
            let r222 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r222);
            blk = 98; continue;
        }
        90 => {
            let Some(t223) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009222e2); };
            c.w32(2, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70009222e6); };
            c.cmp(t224, c.r32(2), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t225) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009222f3); };
            c.w32(0, t225);
            c.w64(2, c.r64(0));
            let r226 = c.r32(2) & 0xc0u64;
            c.w32(2, r226);
            c.logic(r226, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t227) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000922308); };
            c.w32(11, t227);
            let r228 = c.r32(11) & 0x1fu64;
            c.w32(11, r228);
            c.logic(r228, 32);
            let r229 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r229);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 251; continue; }
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 251; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000922326); };
            c.w32(11, t230);
            let r231 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r231);
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700092232d); };
            c.cmp(t232, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t233) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092233a); };
            c.w32(0, t233);
            let Some(t234) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092233e); };
            c.cmp(t234, c.r32(0), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t235) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000922347); };
            c.w32(11, t235);
            let r236 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r236);
            blk = 98; continue;
        }
        97 => {
            let Some(t237) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000922350); };
            c.w32(0, t237);
            let Some(t238) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000922354); };
            c.cmp(t238, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 98; continue;
        }
        98 => {
            let r239 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r239);
            let Some(t240) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922365); };
            c.w64(8, t240);
            let Some(t241) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922369); };
            c.w32(0, t241);
            let Some(t242) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092236e); };
            c.w64(1, t242);
            let Some(t243) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922372); };
            c.w64(5, t243);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r244 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r244);
            return RegfileExit::jump(c.r64(1));
        }
        99 => {
            blk = 251; continue;
        }
        100 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000922387); };
            c.w64(0, t245);
            let Some(t246) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700092238e); };
            c.cmp(t246, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 101; continue;
        }
        101 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t247) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092239d); };
            c.w32(0, t247);
            let r248 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r248);
            blk = 104; continue;
        }
        103 => {
            let Some(t249) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009223a5); };
            c.w32(0, t249);
            let r250 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r250);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            return RegfileExit::fault(0x70009223b0);
        }
        106 => {
            let r251 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r251);
            c.logic(r251, 64);
            let Some(t252) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009223b5); };
            c.w32(2, t252);
            let r253 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r253);
            let Some(t254) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009223bb); };
            c.w32(3, t254);
            let r255 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r255);
            let Some(t256) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009223c1); };
            c.w64(2, t256);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t257) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009223cb); };
            c.w32(11, t257);
            let r258 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r258);
            let Some(t259) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009223d2); };
            c.w32(1, t259);
            let r260 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r260);
            blk = 108; continue;
        }
        108 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r261 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r261);
            let r262 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r262);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009223e4); }
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009223e8); };
            if !c.push64(t263) { return RegfileExit::fault(0x70009223e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009223eb); }
            c.w64(6, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            return c.call(0x70007fe8d4, 0x70009223ff, 0x70009223fa);
        }
        109 => {
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009223ff); };
            c.w64(8, t264);
            let Some(t265) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922403); };
            c.w64(8, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922407); };
            c.w64(9, t266);
            let Some(t267) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092240b); };
            c.w32(11, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922410); };
            c.w64(15, t268);
            let Some(t269) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922414); };
            c.w64(1, t269);
            let Some(t270) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922418); };
            c.w64(5, t270);
            c.w64(12, c.r64(8));
            let r271 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r271);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 114; continue;
        }
        111 => {
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000922427); };
            c.w64(0, t272);
            let Some(t273) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700092242e); };
            c.cmp(t273, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r274 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r274);
            let r275 = c.r64(6) & 0x3ffu64;
            c.w64(6, r275);
            c.logic(r275, 64);
            let r276 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r276);
            let r277 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r277);
            let Some(t278) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092244c); };
            c.w32(1, t278);
            let r279 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r279);
            let Some(t280) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000922452); };
            c.w32(1, t280);
            let r281 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r281);
            let Some(t282) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000922458); };
            c.w32(2, t282);
            let r283 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r283);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092245f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000922463); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(11));
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922470); };
            c.w64(6, t284);
            return c.call(0x70007fea14, 0x7000922479, 0x7000922474);
        }
        113 => {
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922479); };
            c.w64(8, t285);
            let Some(t286) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092247d); };
            c.w64(8, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922481); };
            c.w64(9, t287);
            let Some(t288) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922485); };
            c.w32(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092248a); };
            c.w64(15, t289);
            let Some(t290) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092248e); };
            c.w64(1, t290);
            let Some(t291) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922492); };
            c.w64(5, t291);
            c.w64(12, c.r64(8));
            let r292 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r292);
            return RegfileExit::jump(c.r64(1));
        }
        114 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r293 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r293);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t294) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009224aa); };
            c.w32(3, t294);
            let Some(t295) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009224af); };
            c.w32(2, t295);
            let r296 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r296);
            let Some(t297) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009224b7); };
            c.w32(3, t297);
            let r298 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r298);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009224be); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009224c2); }
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009224c3); };
            if !c.push64(t299) { return RegfileExit::fault(0x70009224c3); }
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009224c6); };
            if !c.push64(t300) { return RegfileExit::fault(0x70009224c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009224c9); }
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70009224cd); };
            c.w64(3, t301);
            c.w32(0, 0x3u64);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009224d9); };
            c.w64(6, t302);
            return c.call(0x700086ed74, 0x70009224e2, 0x70009224dd);
        }
        115 => {
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009224e2); };
            c.w64(8, t303);
            let Some(t304) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009224e6); };
            c.w64(8, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009224ea); };
            c.w64(9, t305);
            let Some(t306) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009224ee); };
            c.w32(11, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009224f3); };
            c.w64(15, t307);
            let Some(t308) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009224f7); };
            c.w64(1, t308);
            let Some(t309) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009224fb); };
            c.w64(5, t309);
            c.w64(12, c.r64(8));
            let r310 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r310);
            return RegfileExit::jump(c.r64(1));
        }
        116 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            let r311 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r311);
            let r312 = c.r64(6) & 0x3ffu64;
            c.w64(6, r312);
            c.logic(r312, 64);
            let r313 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r313);
            let r314 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r314);
            let Some(t315) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922521); };
            c.w32(3, t315);
            let r316 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r316);
            let Some(t317) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000922527); };
            c.w32(3, t317);
            let r318 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r318);
            let Some(t319) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700092252d); };
            c.w32(3, t319);
            let r320 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r320);
            let Some(t321) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000922534); };
            c.w32(7, t321);
            let r322 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r322);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092253a); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092253e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000922540); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922546); };
            c.w64(6, t323);
            return c.call(0x70007f2274, 0x700092254f, 0x700092254a);
        }
        117 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092254f); };
            c.w64(8, t324);
            let Some(t325) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922553); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922557); };
            c.w64(9, t326);
            let Some(t327) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092255b); };
            c.w32(11, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922560); };
            c.w64(15, t328);
            let Some(t329) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922564); };
            c.w64(1, t329);
            let Some(t330) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922568); };
            c.w64(5, t330);
            c.w64(12, c.r64(8));
            let r331 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r331);
            return RegfileExit::jump(c.r64(1));
        }
        118 => {
            let Some(t332) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000922575); };
            c.w32(2, t332);
            let r333 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r333);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000922580); };
            c.w64(2, t334);
            blk = 120; continue;
        }
        120 => {
            let Some(t335) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000922587); };
            c.w32(1, t335);
            let Some(t336) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092258a); };
            c.w32(6, t336);
            let Some(t337) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092258f); };
            c.w32(7, t337);
            let r338 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r338);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092259a); };
            c.w32(6, t339);
            let r340 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r340);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r341 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r341);
            let Some(t342) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009225a8); };
            c.w32(11, t342);
            let r343 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r343);
            let r344 = c.r64(11) & c.r64(1);
            c.w64(11, r344);
            c.logic(r344, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 121; continue;
        }
        121 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(8).wrapping_mul(4));
            let Some(t345) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009225c3); };
            c.w32(0, t345);
            let Some(t346) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009225c7); };
            c.cmp(t346, c.r32(0), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let r347 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r347);
            c.w64(8, c.r64(15).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(1));
            let r348 = c.r64(11) & c.r64(8);
            c.w64(11, r348);
            c.logic(r348, 64);
            blk = 121; continue;
        }
        124 => {
            let Some(t349) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009225e8); };
            c.w32(11, t349);
            let Some(t350) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009225ec); };
            c.w32(0, t350);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r351 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r351);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            return RegfileExit::fault(0x70009225ff);
        }
        126 => {
            let Some(t352) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922600); };
            c.w32(8, t352);
            let Some(t353) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000922605); };
            c.w32(11, t353);
            let r354 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r354);
            let Some(t355) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092260d); };
            c.w32(8, t355);
            let r356 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r356);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 177; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t357) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092261d); };
            c.w32(0, t357);
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000922621); };
            c.cmp(t358, c.r32(0), 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t359) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092262e); };
            c.w32(2, t359);
            let r360 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r360);
            let Some(t361) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922635); };
            c.w32(11, t361);
            let r362 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r362);
            let Some(t363) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092263c); };
            c.logic(t363 & 0x2u64, 8);
            if c.cc_ne() { blk = 150; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000922647); };
            c.cmp(t364, c.r32(11), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000922650); };
            c.w64(11, t365);
            blk = 177; continue;
        }
        131 => {
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700092265c); };
            c.w64(11, t366);
            let Some(t367) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000922663); };
            c.cmp(t367, 0x20u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t368) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700092266d); };
            c.w32(8, t368);
            let Some(t369) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000922671); };
            c.cmp(t369, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092267a); };
            c.w64(8, t370);
            blk = 251; continue;
        }
        134 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000922683); };
            c.w64(0, t371);
            let Some(t372) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922687); };
            c.w32(8, t372);
            let r373 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r373);
            let Some(t374) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092268e); };
            c.w32(11, t374);
            let Some(t375) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000922692); };
            c.cmp(t375, c.r32(11), 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t376) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092269b); };
            c.w32(8, t376);
            let r377 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r377);
            let Some(t378) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009226a2); };
            c.w32(11, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009226a6); };
            c.cmp(t379, c.r32(11), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 136; continue;
        }
        136 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t380) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009226b9); };
            c.w32(11, t380);
            let r381 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r381);
            let Some(t382) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009226c0); };
            c.w32(3, t382);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t383) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009226dd); };
            c.cmp(t383, c.r32(11), 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009226e6); };
            c.w64(8, t384);
            blk = 251; continue;
        }
        141 => {
            let Some(t385) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009226ef); };
            c.w32(6, t385);
            let r386 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r386);
            c.w64(0, c.r64(6));
            blk = 143; continue;
        }
        142 => {
            c.w64(0, c.r64(8));
            blk = 143; continue;
        }
        143 => {
            let Some(t387) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009226fd); };
            c.w32(8, t387);
            let r388 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r388);
            let Some(t389) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922704); };
            c.w32(8, t389);
            let r390 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r390);
            let Some(t391) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092270b); };
            c.w32(11, t391);
            let r392 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r392);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r393 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r393);
            let Some(t394) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000922724); };
            c.w32(6, t394);
            let r395 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092272b); };
            c.w64(8, t396);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092272f); }
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000922733); };
            if !c.push64(t397) { return RegfileExit::fault(0x7000922733); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700092273d, 0x7000922738);
        }
        145 => {
            c.w64(11, c.r64(0));
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922740); };
            c.w64(9, t398);
            blk = 177; continue;
        }
        146 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922749); };
            c.w64(8, t399);
            blk = 251; continue;
        }
        147 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922752); };
            c.w64(8, t400);
            blk = 251; continue;
        }
        148 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r401 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922764); };
            c.w64(3, t402);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000922768); }
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092276c); };
            if !c.push64(t403) { return RegfileExit::fault(0x700092276c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700092276f); }
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000922771); };
            if !c.push64(t404) { return RegfileExit::fault(0x7000922771); }
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000922774); };
            c.w64(6, t405);
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000922778); };
            c.w64(3, t406);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700092278c, 0x7000922787);
        }
        149 => {
            c.w64(11, c.r64(0));
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092278f); };
            c.w64(9, t407);
            blk = 177; continue;
        }
        150 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r408 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009227a2); };
            c.w64(0, t409);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009227a6); }
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009227aa); };
            if !c.push64(t410) { return RegfileExit::fault(0x70009227aa); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009227b2); };
            c.w64(6, t411);
            return c.call(0x70007f2274, 0x70009227bb, 0x70009227b6);
        }
        151 => {
            c.w64(11, c.r64(0));
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009227be); };
            c.w64(9, t412);
            blk = 177; continue;
        }
        152 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009227c7); };
            c.w64(0, t413);
            let Some(t414) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009227cb); };
            c.w32(8, t414);
            let r415 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r415);
            let Some(t416) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009227d2); };
            c.w32(3, t416);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 174; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 163; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009227f4); };
            c.w64(8, t417);
            blk = 251; continue;
        }
        156 => {
            let Some(t418) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009227fd); };
            c.w32(8, t418);
            let Some(t419) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000922801); };
            c.cmp(t419, c.r32(8), 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092280a); };
            c.w64(8, t420);
            blk = 251; continue;
        }
        158 => {
            let Some(t421) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000922813); };
            c.w32(8, t421);
            let r422 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r422);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922820); };
            c.w64(8, t423);
            blk = 251; continue;
        }
        160 => {
            let Some(t424) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922829); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922830); };
            c.cmp(t426, 0x40u64, 16);
            if c.cc_b() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922838); };
            c.w64(8, t427);
            blk = 251; continue;
        }
        162 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922841); };
            c.w32(8, t428);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 177; continue;
        }
        163 => {
            let Some(t429) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092284e); };
            c.w32(11, t429);
            let Some(t430) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000922852); };
            c.cmp(t430, c.r32(11), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092285b); };
            c.w64(8, t431);
            blk = 251; continue;
        }
        165 => {
            let Some(t432) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000922864); };
            c.w32(8, t432);
            c.w64(11, c.r64(8));
            let r433 = c.r32(11) & 0xc0u64;
            c.w32(11, r433);
            c.logic(r433, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t434) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092287c); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000922883); };
            c.w32(11, t436);
            let r437 = c.r32(11) & 0x1fu64;
            c.w32(11, r437);
            c.logic(r437, 32);
            let r438 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r438);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922895); };
            c.w64(8, t439);
            blk = 251; continue;
        }
        168 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 173; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t440) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70009228a4); };
            c.w32(11, t440);
            let r441 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r441);
            let Some(t442) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009228ab); };
            c.cmp(t442, c.r32(11), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t443) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009228b4); };
            c.w32(8, t443);
            let Some(t444) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009228b8); };
            c.cmp(t444, c.r32(8), 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t445) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009228c1); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            blk = 177; continue;
        }
        172 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009228ca); };
            c.w64(8, t447);
            blk = 251; continue;
        }
        173 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009228d3); };
            c.w64(8, t448);
            blk = 251; continue;
        }
        174 => {
            let Some(t449) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009228dc); };
            c.w32(8, t449);
            let Some(t450) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009228e0); };
            c.cmp(t450, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009228e9); };
            c.w64(8, t451);
            blk = 251; continue;
        }
        176 => {
            let Some(t452) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009228f2); };
            c.w32(11, t452);
            let r453 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r453);
            blk = 177; continue;
        }
        177 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922900); };
            c.w64(3, t454);
            let Some(t455) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922904); };
            c.w64(8, t455);
            let Some(t456) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922908); };
            c.w32(0, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092290d); };
            c.w64(15, t457);
            let Some(t458) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922911); };
            c.w64(1, t458);
            let Some(t459) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922915); };
            c.w64(5, t459);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r460 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r460);
            return RegfileExit::jump(c.r64(1));
        }
        178 => {
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922925); };
            c.w64(8, t461);
            blk = 251; continue;
        }
        179 => {
            let r462 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r462);
            let Some(t463) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922932); };
            c.w64(8, t463);
            let Some(t464) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922936); };
            c.w32(11, t464);
            let Some(t465) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092293b); };
            c.w64(1, t465);
            let Some(t466) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092293f); };
            c.w64(5, t466);
            c.w64(12, c.r64(8));
            let r467 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r467);
            return RegfileExit::jump(c.r64(1));
        }
        180 => {
            let r468 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r468);
            let Some(t469) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922950); };
            c.w32(8, t469);
            let Some(t470) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922955); };
            c.w64(1, t470);
            let Some(t471) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922959); };
            c.w64(5, t471);
            c.w64(0, c.r64(3));
            let r472 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r472);
            return RegfileExit::jump(c.r64(1));
        }
        181 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 195; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            let r473 = c.r64(11) & 0x7ffu64;
            c.w64(11, r473);
            c.logic(r473, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 190; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t474) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092298f); };
            c.w32(3, t474);
            let r475 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r475);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700092299a); };
            c.w64(3, t476);
            blk = 185; continue;
        }
        185 => {
            let Some(t477) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009229a1); };
            c.w32(11, t477);
            let r478 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r478);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let r479 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r479);
            let Some(t480) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009229b2); };
            c.w32(8, t480);
            let Some(t481) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009229b7); };
            c.w64(1, t481);
            let Some(t482) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009229bb); };
            c.w64(5, t482);
            c.w64(0, c.r64(11));
            let r483 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r483);
            return RegfileExit::jump(c.r64(1));
        }
        187 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t484) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009229d2); };
            c.w32(0, t484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009229d6); };
            c.cmp(t485, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 189; continue;
        }
        189 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009229e3); }
            blk = 243; continue;
        }
        190 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let r486 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r486);
            let Some(t487) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009229f7); };
            c.w32(8, t487);
            let Some(t488) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009229fc); };
            c.w32(0, t488);
            let r489 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r489);
            let Some(t490) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922a04); };
            c.w64(1, t490);
            let Some(t491) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922a08); };
            c.w64(5, t491);
            let r492 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r492);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let Some(t493) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922a12); };
            c.w32(11, t493);
            let r494 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r494);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t495) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922a24); };
            c.w32(0, t495);
            let Some(t496) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000922a28); };
            c.cmp(t496, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000922a35); }
            blk = 243; continue;
        }
        195 => {
            c.w64(11, c.r64(6));
            let r497 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r497);
            let r498 = c.r32(11) & 0xfu64;
            c.w32(11, r498);
            c.logic(r498, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 202; continue; }
            blk = 196; continue;
        }
        196 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 200; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 206; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            return RegfileExit::fault(0x7000922ab9);
        }
        209 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r499 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r499);
            let r500 = c.r64(6) & 0xfffffu64;
            c.w64(6, r500);
            c.logic(r500, 64);
            let Some(t501) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922ac9); };
            c.w32(8, (t501 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t502) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922ad3); };
            c.w32(8, t502);
            let Some(t503) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922ad8); };
            c.w64(1, t503);
            let Some(t504) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922adc); };
            c.w64(5, t504);
            let r505 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(c.r64(1));
        }
        210 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r506 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r506);
            let r507 = c.r64(6) & 0xfffffu64;
            c.w64(6, r507);
            c.logic(r507, 64);
            let Some(t508) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922af5); };
            c.w32(8, (t508 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t509) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922aff); };
            c.w32(8, t509);
            let Some(t510) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922b04); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922b08); };
            c.w64(5, t511);
            let r512 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r512);
            return RegfileExit::jump(c.r64(1));
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r513 = c.r64(6) & 0xfffffu64;
            c.w64(6, r513);
            c.logic(r513, 64);
            let Some(t514) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922b1d); };
            c.w32(8, t514);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000922b22); }
            c.w64(11, c.r64(8));
            let r515 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r515);
            if c.cc_o() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 218; continue;
        }
        213 => {
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000922b36); };
            c.w64(11, t516);
            let Some(t517) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000922b3d); };
            c.w64(0, t517);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t518) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000922b44); };
            c.w64(2, t518);
            let Some(t519) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000922b4b); };
            c.cmp(t519, c.r64(3), 64);
            if c.cc_a() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000922b5a, 0x7000922b55);
        }
        215 => {
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922b5a); };
            c.w64(8, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922b5e); };
            c.w64(15, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922b62); };
            c.w64(9, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922b66); };
            c.w64(12, t523);
            blk = 217; continue;
        }
        216 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000922b6c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 217; continue;
        }
        217 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000922b7f); };
            c.w64(8, t524);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000922b86); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000922b8a); }
            blk = 218; continue;
        }
        218 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t525) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922b96); };
            c.w32(8, t525);
            let Some(t526) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922b9b); };
            c.w64(1, t526);
            let Some(t527) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922b9f); };
            c.w64(5, t527);
            let r528 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r528);
            return RegfileExit::jump(c.r64(1));
        }
        219 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r529 = c.r64(6) & 0xfffffu64;
            c.w64(6, r529);
            c.logic(r529, 64);
            let Some(t530) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922bb4); };
            c.w32(8, t530);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000922bb9); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let r531 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r531);
            blk = 226; continue;
        }
        221 => {
            let Some(t532) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000922bcb); };
            c.w64(11, t532);
            let Some(t533) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000922bd2); };
            c.w64(0, t533);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000922bd9); };
            c.w64(2, t534);
            let Some(t535) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000922be0); };
            c.cmp(t535, c.r64(3), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000922bef, 0x7000922bea);
        }
        223 => {
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922bef); };
            c.w64(8, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922bf3); };
            c.w64(15, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922bf7); };
            c.w64(9, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922bfb); };
            c.w64(12, t539);
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000922c01); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 225; continue;
        }
        225 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t540) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000922c17); };
            c.w64(8, t540);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000922c1e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000922c22); }
            c.w64(8, c.r64(0));
            blk = 226; continue;
        }
        226 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t541) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922c31); };
            c.w32(11, t541);
            let Some(t542) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922c36); };
            c.w64(1, t542);
            let Some(t543) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922c3a); };
            c.w64(5, t543);
            c.w64(0, c.r64(8));
            let r544 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r544);
            return RegfileExit::jump(c.r64(1));
        }
        227 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r545 = c.r64(6) & 0xfffffu64;
            c.w64(6, r545);
            c.logic(r545, 64);
            let Some(t546) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922c52); };
            c.w64(8, t546);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000922c57); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000922c61); };
            c.w64(11, t547);
            let Some(t548) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000922c68); };
            c.w64(0, t548);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t549) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000922c6f); };
            c.w64(2, t549);
            let Some(t550) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000922c76); };
            c.cmp(t550, c.r64(3), 64);
            if c.cc_a() { blk = 231; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x7000922c85, 0x7000922c80);
        }
        230 => {
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922c85); };
            c.w64(8, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922c89); };
            c.w64(15, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922c8d); };
            c.w64(9, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922c91); };
            c.w64(12, t554);
            blk = 232; continue;
        }
        231 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000922c97); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 232; continue;
        }
        232 => {
            let Some(t555) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000922ca1); };
            c.w64(3, t555);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000922ca8); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000922cb1); }
            let r556 = c.neg(c.r64(8), 64);
            c.w64(8, r556);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000922cbb); }
            blk = 240; continue;
        }
        234 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000922cc1); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000922cc8); }
            blk = 240; continue;
        }
        235 => {
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000922cce); };
            c.w64(8, t557);
            let Some(t558) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000922cd5); };
            c.w64(11, t558);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000922cdc); };
            c.w64(0, t559);
            let Some(t560) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000922ce3); };
            c.cmp(t560, c.r64(3), 64);
            if c.cc_a() { blk = 238; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000922cf2, 0x7000922ced);
        }
        237 => {
            blk = 239; continue;
        }
        238 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x7000922cf4); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 239; continue;
        }
        239 => {
            let Some(t561) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000922cfe); };
            c.w64(8, t561);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000922d05); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000922d09); }
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922d10); };
            c.w64(12, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922d14); };
            c.w64(15, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922d18); };
            c.w64(9, t564);
            blk = 240; continue;
        }
        240 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t565) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922d23); };
            c.w32(8, t565);
            let Some(t566) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922d28); };
            c.w64(1, t566);
            let Some(t567) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922d2c); };
            c.w64(5, t567);
            let r568 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r568);
            return RegfileExit::jump(c.r64(1));
        }
        241 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r569 = c.r64(6) & 0xfffffu64;
            c.w64(6, r569);
            c.logic(r569, 64);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000922d41); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 243; continue;
        }
        242 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r570 = c.r64(6) & 0xfffffu64;
            c.w64(6, r570);
            c.logic(r570, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000922d58); }
            blk = 243; continue;
        }
        243 => {
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000922d5e); };
            c.w64(8, t571);
            let Some(t572) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000922d65); };
            c.w64(11, t572);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000922d6c); }
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000922d71); };
            c.w64(3, t573);
            let Some(t574) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000922d78); };
            c.cmp(t574, c.r64(0), 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000922d87, 0x7000922d82);
        }
        245 => {
            blk = 247; continue;
        }
        246 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000922d89); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 247; continue;
        }
        247 => {
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922d93); };
            c.w64(11, t575);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t576) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000922d9b); };
            c.w64(8, t576);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000922da2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000922da6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000922dab); }
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922db0); };
            c.w64(12, t577);
            let Some(t578) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922db4); };
            c.w32(8, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922db9); };
            c.w64(15, t579);
            let Some(t580) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922dbd); };
            c.w64(1, t580);
            let Some(t581) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922dc1); };
            c.w64(5, t581);
            let r582 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r582);
            return RegfileExit::jump(c.r64(1));
        }
        248 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r583 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r583);
            let r584 = c.r64(6) & 0xfffffu64;
            c.w64(6, r584);
            c.logic(r584, 64);
            let Some(t585) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000922dda); };
            c.w32(8, t585);
            let Some(t586) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922ddf); };
            c.w32(0, t586);
            let r587 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r587);
            let Some(t588) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922de6); };
            c.w64(1, t588);
            let Some(t589) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922dea); };
            c.w64(5, t589);
            let r590 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r590);
            return RegfileExit::jump(c.r64(1));
        }
        249 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r591 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r591);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000922dfb); }
            let Some(t592) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000922dff); };
            c.w64(8, t592);
            let Some(t593) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922e06); };
            c.w32(8, t593);
            let r594 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r594);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000922e0d); }
            let Some(t595) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000922e0f); };
            c.w64(3, t595);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000922e22, 0x7000922e1d);
        }
        250 => {
            return RegfileExit::fault(0x7000922e22);
        }
        251 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r596 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r596);
            c.w64(0, c.r64(9).wrapping_add(0x9));
            let Some(t597) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922e2e); };
            c.w32(3, t597);
            let Some(t598) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922e33); };
            c.w32(2, t598);
            let r599 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r599);
            let Some(t600) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000922e3b); };
            c.w64(1, t600);
            let Some(t601) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922e3f); };
            c.w32(3, t601);
            let r602 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r602);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000922e46); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000922e4a); }
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000922e4b); };
            if !c.push64(t603) { return RegfileExit::fault(0x7000922e4b); }
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000922e4e); };
            if !c.push64(t604) { return RegfileExit::fault(0x7000922e4e); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000922e54); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000922e56); }
            let Some(t605) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x7000922e5a); };
            c.w64(3, t605);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x7000922e71, 0x7000922e6c);
        }
        252 => {
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922e71); };
            c.w64(8, t606);
            let Some(t607) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922e75); };
            c.w64(8, t607);
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000922e79); };
            c.w64(9, t608);
            let Some(t609) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922e7d); };
            c.w32(11, t609);
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922e82); };
            c.w64(15, t610);
            let Some(t611) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922e86); };
            c.w64(1, t611);
            let Some(t612) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922e8a); };
            c.w64(5, t612);
            c.w64(12, c.r64(8));
            let r613 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r613);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            return RegfileExit::fault(0x7000922e97);
        }
        254 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000922e98); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000922e9c); }
            let r614 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r614);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r615 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r615);
            let Some(t616) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922ea9); };
            c.w32(0, t616);
            let Some(t617) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922eae); };
            c.w32(3, t617);
            let r618 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r618);
            let Some(t619) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000922eb6); };
            c.w64(2, t619);
            let Some(t620) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922eba); };
            c.w32(1, t620);
            let r621 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r621);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000922ec1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000922ec5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000922ec9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000922ecd); }
            c.w32(0, 0xeu64);
            c.w64(6, c.r64(2));
            return c.call(0x7000804df4, 0x7000922ede, 0x7000922ed9);
        }
        255 => {
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922ede); };
            c.w64(8, t622);
            let Some(t623) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922ee2); };
            c.w64(8, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922ee6); };
            c.w64(11, t624);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t625) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000922eee); };
            c.w32(11, t625);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922ef3); };
            c.w64(15, t626);
            let Some(t627) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922ef7); };
            c.w64(1, t627);
            let Some(t628) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922efb); };
            c.w64(5, t628);
            c.w64(12, c.r64(8));
            let r629 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r629);
            return RegfileExit::jump(c.r64(1));
        }
        256 => {
            return RegfileExit::jump(0x7000922f0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000922f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000922f14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000922f18); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922f1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000922f22); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000922f26); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922f2d); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922f32); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000922f3a); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000922f3e); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000922f45); };
            c.w32(6, t11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000922f4a); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000922f52); };
            c.w32(3, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000922f56); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000922f5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000922f5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000922f62); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000922f66); };
            c.cmp(t15, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000922f6f); };
            c.w64(8, t16);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r17 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r17);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(6).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000922f83); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(3, c.r64(8));
            return c.call(0x700083bef4, 0x7000922f95, 0x7000922f90);
        }
        3 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000922f95); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922f99); };
            c.w64(2, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000922f9d); };
            c.w64(8, t20);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t21) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000922fa5); };
            c.w32(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000922faa); };
            c.w64(15, t22);
            let Some(t23) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922fae); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000922fb2); };
            c.w64(5, t24);
            c.w64(12, c.r64(2));
            let Some(t25) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000922fb9); };
            c.w64(0, t25);
            let r26 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r26);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000922fc8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000922fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000922fd4); };
            c.w64(3, (t0 as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000922fdc); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000922fe0); };
            c.w32(2, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000922fe5); };
            c.w32(6, t3);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000922fef); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000922ffe); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let r7 = c.add(c.r32(6), 0xffffffffffffffffu64, 32);
            c.w32(6, r7);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x700092300b);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092300c); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x700092301a);
        }
        7 => {
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x7000923023); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r9 = c.r64(6) & c.r64(3);
            c.w64(6, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000923034); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r11 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092303e); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923043); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.w32(0, c.r32(0));
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            c.w64(6, 0xfffffffffffc0000u64);
            let r15 = c.r64(6) & c.r64(0);
            c.w64(6, r15);
            c.logic(r15, 64);
            let Some(t16) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000923058); };
            c.logic(t16 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000923062); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000923063); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x7000923070, 0x700092306b);
        }
        12 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000923070); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000923072); }
            blk = 9; continue;
        }
        13 => {
            return RegfileExit::jump(0x7000923078);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000923094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000923097); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092309b); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009230a1); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70009230ae);
        }
        2 => {
            c.w64(2, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x70009230b7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r3 = c.r64(6) & c.r64(3);
            c.w64(6, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009230c8); };
            c.logic(t4 & 0x4u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let r5 = c.add(c.r64(9), 0x5u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009230d2); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009230d7); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            c.w32(0, c.r32(0));
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            c.w64(6, 0xfffffffffffc0000u64);
            let r9 = c.r64(6) & c.r64(0);
            c.w64(6, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009230ec); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70009230f6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009230f7); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x7000923104, 0x70009230ff);
        }
        7 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000923104); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000923106); }
            blk = 4; continue;
        }
        8 => {
            return RegfileExit::jump(0x700092310c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000923114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000923114); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000923118); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092311e); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923123); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092312b); };
            c.w64(8, t4);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923133); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092313a); };
            c.w64(7, t7);
            let r8 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r8);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923141); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000923145); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000923146); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092314a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092314e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x7000923152); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000923166, 0x7000923161);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923166); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092316a); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092316e); };
            c.w64(7, t12);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000923176); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092317b); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700092317f); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923183); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000923194);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009231b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009231b7); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009231bb); };
            c.w32(8, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009231c0); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009231c8); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009231d7); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009231dd); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009231f5); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009231f8); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923207); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r32(11), 0xffffffffffffffffu64, 32);
            c.w32(11, r11);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000923215);
        }
        9 => {
            return RegfileExit::fault(0x7000923216);
        }
        10 => {
            return RegfileExit::fault(0x7000923217);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923218); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x7000923226);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092322b); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923230); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923238); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092323e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000923242); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923248); };
            c.w32(8, t19);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923251); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            let r22 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092325c); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923264); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000923268); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092326a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092326e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000923272); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x7000923276); };
            c.w64(3, t25);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700092328a, 0x7000923285);
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092328a); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092328e); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923292); };
            c.w64(9, t28);
            let r29 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r29);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700092329a); };
            c.w32(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092329f); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009232a3); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009232a7); };
            c.w64(5, t33);
            c.w64(12, c.r64(2));
            let r34 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x70009232b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009232d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009232d7); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009232db); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009232e3); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009232f2); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009232f8); };
            c.w32(1, t5);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923310); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000923313); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923326); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let r10 = c.add(c.r32(0), 0xffffffffffffffffu64, 32);
            c.w32(0, r10);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000923333);
        }
        9 => {
            return RegfileExit::fault(0x7000923334);
        }
        10 => {
            return RegfileExit::fault(0x7000923335);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923336); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092333a); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923341); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923348); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092334c); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000923355); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092335c); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000923369); };
            c.w64(0, (t20 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000923375); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let r23 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092338f); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923395); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009233a6); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009233ab); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009233b4); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009233b8); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009233bf); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009233c6); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70009233d9);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70009233da); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009233e7); };
            c.w32(0, t38);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r39 = c.r64(3) & 0xfffu64;
            c.w64(3, r39);
            c.logic(r39, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 265; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092340b); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923413); };
            c.w32(0, t42);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r43 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r43);
            c.logic(r43, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x700092342d);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000923432); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923437); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092343f); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000923448); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092344c); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000923452); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000923456); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700092345a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092345e); }
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000923467); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092346b); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000923471); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000923479); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092347d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000923481); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000923485); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000923489); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700092348d); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000923491); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092349e); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009234a2); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009234a8); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009234ae); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70009234b7); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x70009234c1);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009234c2); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70009234c8); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70009234d2);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009234d3); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009234e3); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70009234e6); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009234ef); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009234fd); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923507); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092350a); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000923514); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 37; continue;
        }
        37 => {
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            blk = 46; continue;
        }
        38 => {
            c.w64(0, c.r64(2));
            blk = 46; continue;
        }
        39 => {
            let r74 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092353b); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923540); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923544); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r79 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r79);
            c.w64(6, c.r64(9).wrapping_add(0xd));
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923558); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092355d); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923565); };
            c.w32(9, t83);
            let r84 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r84);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092356d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000923571); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092357e); };
            c.logic(t85 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r86 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r86);
            blk = 43; continue;
        }
        42 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092358d); };
            c.w32(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000923590); };
            c.w64(7, t88);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x700092359a, 0x7000923598);
        }
        44 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092359a); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092359e); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009235a2); };
            c.w64(9, t91);
            let Some(t92) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009235a6); };
            c.w32(11, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009235ab); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009235af); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009235b3); };
            c.w64(5, t95);
            c.w64(12, c.r64(8));
            let r96 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r96);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(7));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(3, c.r64(1));
            let r97 = c.r64(3) & 0xfu64;
            c.w64(3, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009235d6); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(3), 0xdu64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_e() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(3), 0x6u64, 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 122; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(3), 0x8u64, 64);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(3), 0x9u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(3), 0xcu64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(3), 0xeu64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r98 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r98);
            c.w64(3, c.r64(9).wrapping_add(0xd));
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923664); };
            c.w32(2, t99);
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923669); };
            c.w32(1, t100);
            let r101 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r101);
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923671); };
            c.w32(2, t102);
            let r103 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r103);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923678); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700092367c); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092367d); };
            if !c.push64(t104) { return RegfileExit::fault(0x700092367d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000923680); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000923681); };
            if !c.push64(t105) { return RegfileExit::fault(0x7000923681); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000923684); };
            if !c.push64(t106) { return RegfileExit::fault(0x7000923684); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x7000923687); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092368b); };
            c.w64(6, t107);
            c.w32(0, 0x5u64);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000923694); };
            c.w64(3, t108);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70009236a3, 0x700092369e);
        }
        59 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009236a3); };
            c.w64(8, t109);
            let Some(t110) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009236a7); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70009236ab); };
            c.w64(9, t111);
            let Some(t112) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009236af); };
            c.w32(11, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009236b4); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009236b8); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009236bc); };
            c.w64(5, t115);
            c.w64(12, c.r64(8));
            let r116 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009236c9); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009236d0); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009236d7); };
            c.w32(0, t121);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r122 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r122);
            c.logic(r122, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x70009236f1);
        }
        62 => {
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009236f2); };
            c.w32(11, t123);
            let r124 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r124);
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009236fa); };
            c.w32(0, t125);
            let r126 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r126);
            c.w64(11, c.r64(9).wrapping_add(0xd));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000923705); }
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923709); };
            c.cmp(t127, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000923712); };
            c.w32(8, t128);
            let Some(t129) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923717); };
            c.w64(1, t129);
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092371b); };
            c.w64(5, t130);
            c.w64(9, c.r64(11));
            let r131 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r131);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r132 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r132);
            let Some(t133) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092372e); };
            c.w32(3, t133);
            let Some(t134) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923733); };
            c.w32(2, t134);
            let r135 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r135);
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092373b); };
            c.w32(3, t136);
            let r137 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r137);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000923742); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092374b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092374c); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700092374d); };
            c.w64(3, t138);
            c.w32(0, 0x2u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923759); };
            c.w64(6, t139);
            return c.call(0x700086ed74, 0x7000923762, 0x700092375d);
        }
        65 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923762); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923766); };
            c.w64(8, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092376a); };
            c.w64(9, t142);
            let Some(t143) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092376e); };
            c.w32(11, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923773); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923777); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092377b); };
            c.w64(5, t146);
            c.w64(12, c.r64(8));
            let r147 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r147);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r148 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r148);
            c.w64(2, c.r64(9).wrapping_add(0xd));
            let Some(t149) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923792); };
            c.w32(1, t149);
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923797); };
            c.w32(6, t150);
            let r151 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r151);
            let Some(t152) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092379f); };
            c.w32(1, t152);
            let r153 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r153);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70009237a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70009237aa); }
            c.w64(3, c.r64(1));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009237b1); };
            c.w64(1, t154);
            c.w64(2, c.r64(11));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009237b8); };
            c.w64(6, t155);
            return c.call(0x70008bc174, 0x70009237c1, 0x70009237bc);
        }
        67 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009237c1); };
            c.w64(8, t156);
            let Some(t157) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009237c5); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009237c9); };
            c.w64(9, t158);
            let Some(t159) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009237cd); };
            c.w32(11, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009237d2); };
            c.w64(15, t160);
            let Some(t161) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009237d6); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009237da); };
            c.w64(5, t162);
            c.w64(12, c.r64(8));
            let r163 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r163);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009237ec); };
            c.w32(3, t164);
            let r165 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r165);
            let Some(t166) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009237f2); };
            c.w32(2, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009237f5); };
            c.cmp(t167, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t168) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923802); };
            c.w32(1, t168);
            let Some(t169) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923807); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092380f); };
            c.w32(1, t171);
            let r172 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r172);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t173) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092381f); };
            c.w32(2, t173);
            let r174 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000923825); };
            c.cmp(t175, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t176) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923832); };
            c.w32(2, t176);
            let r177 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r177);
            let Some(t178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923838); };
            c.w32(6, t178);
            let r179 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r179);
            let Some(t180) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092383e); };
            c.logic(t180 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000923848); };
            c.cmp(t181, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000923851); };
            c.w64(3, t182);
            blk = 109; continue;
        }
        75 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700092385d); };
            c.w64(6, t183);
            let Some(t184) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x7000923864); };
            c.cmp(t184, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t185) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700092386d); };
            c.w32(1, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923870); };
            c.cmp(t186, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092387d); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923883); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923886); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092388f); };
            c.w32(1, t191);
            let r192 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r192);
            let Some(t193) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923895); };
            c.w32(6, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923898); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t195) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009238aa); };
            c.w32(6, t195);
            let r196 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r196);
            let Some(t197) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009238b0); };
            c.w32(7, t197);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(1), 0x82au64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009238cc); };
            c.cmp(t198, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009238d5); };
            c.w64(12, t199);
            blk = 263; continue;
        }
        84 => {
            c.w64(0, c.r64(3));
            blk = 86; continue;
        }
        85 => {
            c.w64(0, c.r64(1));
            blk = 86; continue;
        }
        86 => {
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009238e6); };
            c.w32(0, t200);
            let r201 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r201);
            let Some(t202) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009238ec); };
            c.w32(0, t202);
            let r203 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r203);
            let Some(t204) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009238f2); };
            c.w32(3, t204);
            let r205 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r205);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r206 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r206);
            let Some(t207) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000923907); };
            c.w32(6, t207);
            let r208 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r208);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700092390d); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923911); };
            if !c.push64(t209) { return RegfileExit::fault(0x7000923911); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x700092391c, 0x7000923917);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092391f); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923923); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923927); };
            c.w64(8, t212);
            blk = 109; continue;
        }
        89 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000923930); };
            c.w64(12, t213);
            blk = 263; continue;
        }
        90 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r214 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r214);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923940); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000923944); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000923945); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923946); };
            if !c.push64(t215) { return RegfileExit::fault(0x7000923946); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923949); };
            c.w64(6, t216);
            c.w32(0, 0x3u64);
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000923952); };
            c.w64(3, t217);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000923961, 0x700092395c);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923964); };
            c.w64(15, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923968); };
            c.w64(9, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092396c); };
            c.w64(8, t220);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r221 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092397e); }
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923982); };
            if !c.push64(t222) { return RegfileExit::fault(0x7000923982); }
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923985); };
            c.w64(6, t223);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000923993, 0x700092398e);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923996); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092399a); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092399e); };
            c.w64(8, t226);
            blk = 109; continue;
        }
        94 => {
            let Some(t227) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009239a7); };
            c.w32(11, t227);
            let r228 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r228);
            let Some(t229) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009239ae); };
            c.w32(1, t229);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 263; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009239d4); };
            c.w32(11, t230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009239d8); };
            c.cmp(t231, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t232) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009239ee); };
            c.cmp(t232, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t233) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009239f9); };
            c.w32(11, t233);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t234) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923a06); };
            c.w32(2, t234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000923a09); };
            c.cmp(t235, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t236) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000923a16); };
            c.w32(11, t236);
            c.w64(2, c.r64(11));
            let r237 = c.r32(2) & 0xc0u64;
            c.w32(2, r237);
            c.logic(r237, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t238) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000923a2c); };
            c.w32(3, t238);
            let r239 = c.r32(3) & 0x1fu64;
            c.w32(3, r239);
            c.logic(r239, 32);
            let r240 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r240);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_a() { blk = 263; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t241) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000923a48); };
            c.w32(3, t241);
            let r242 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r242);
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923a4e); };
            c.cmp(t243, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t244) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923a5b); };
            c.w32(11, t244);
            let Some(t245) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923a5f); };
            c.cmp(t245, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t246) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923a68); };
            c.w32(3, t246);
            let r247 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r247);
            blk = 109; continue;
        }
        108 => {
            let Some(t248) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923a70); };
            c.w32(11, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000923a74); };
            c.cmp(t249, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r250 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r250);
            let Some(t251) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923a85); };
            c.w64(8, t251);
            let Some(t252) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000923a89); };
            c.w32(11, t252);
            let Some(t253) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923a8e); };
            c.w64(1, t253);
            let Some(t254) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923a92); };
            c.w64(5, t254);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r255 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r255);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000923aa7); };
            c.w64(3, t256);
            let Some(t257) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x7000923aae); };
            c.cmp(t257, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t258) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923abc); };
            c.w32(3, t258);
            let r259 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r259);
            blk = 115; continue;
        }
        114 => {
            let Some(t260) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000923ac4); };
            c.w32(3, t260);
            let r261 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r261);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x7000923acf);
        }
        117 => {
            let r262 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r262);
            c.logic(r262, 64);
            let Some(t263) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923ad4); };
            c.w32(1, t263);
            let r264 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r264);
            let Some(t265) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923ada); };
            c.w32(0, t265);
            let r266 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r266);
            let Some(t267) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923ae0); };
            c.w64(1, t267);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923ae9); };
            c.w32(2, t268);
            let r269 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r269);
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923aef); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            blk = 119; continue;
        }
        119 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r272 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r272);
            let r273 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r273);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000923aff); }
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923b03); };
            if !c.push64(t274) { return RegfileExit::fault(0x7000923b03); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000923b06); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x7000923b1e, 0x7000923b19);
        }
        120 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923b1e); };
            c.w64(8, t275);
            let Some(t276) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923b22); };
            c.w64(8, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923b26); };
            c.w64(9, t277);
            let Some(t278) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000923b2a); };
            c.w32(11, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923b2f); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923b33); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923b37); };
            c.w64(5, t281);
            c.w64(12, c.r64(8));
            let r282 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r282);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000923b46); };
            c.w64(11, t283);
            let Some(t284) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000923b4d); };
            c.cmp(t284, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r285 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r285);
            let r286 = c.r64(1) & 0x3ffu64;
            c.w64(1, r286);
            c.logic(r286, 64);
            let r287 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r287);
            let r288 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r288);
            let Some(t289) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923b6d); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r290);
            let Some(t291) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000923b73); };
            c.w32(2, t291);
            let r292 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r292);
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000923b79); };
            c.w32(3, t293);
            let r294 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r294);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923b80); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000923b84); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923b8e); };
            c.w64(6, t295);
            return c.call(0x70007fea14, 0x7000923b97, 0x7000923b92);
        }
        124 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923b97); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923b9b); };
            c.w64(8, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923b9f); };
            c.w64(9, t298);
            let Some(t299) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000923ba3); };
            c.w32(11, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923ba8); };
            c.w64(15, t300);
            let Some(t301) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923bac); };
            c.w64(1, t301);
            let Some(t302) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923bb0); };
            c.w64(5, t302);
            c.w64(12, c.r64(8));
            let r303 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r303);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r304 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r304);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t305) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923bc8); };
            c.w32(3, t305);
            let Some(t306) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923bcd); };
            c.w32(2, t306);
            let r307 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r307);
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923bd5); };
            c.w32(3, t308);
            let r309 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r309);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923bdc); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000923be0); }
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000923be1); };
            if !c.push64(t310) { return RegfileExit::fault(0x7000923be1); }
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000923be4); };
            if !c.push64(t311) { return RegfileExit::fault(0x7000923be4); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000923be7); }
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x7000923beb); };
            c.w64(3, t312);
            c.w32(0, 0x3u64);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923bf7); };
            c.w64(6, t313);
            return c.call(0x700086ed74, 0x7000923c00, 0x7000923bfb);
        }
        126 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923c00); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923c04); };
            c.w64(8, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000923c08); };
            c.w64(9, t316);
            let Some(t317) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000923c0c); };
            c.w32(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923c11); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923c15); };
            c.w64(1, t319);
            let Some(t320) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923c19); };
            c.w64(5, t320);
            c.w64(12, c.r64(8));
            let r321 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r321);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r322 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r322);
            let r323 = c.r64(1) & 0x3ffu64;
            c.w64(1, r323);
            c.logic(r323, 64);
            let r324 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r324);
            let r325 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r325);
            let Some(t326) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923c40); };
            c.w32(0, t326);
            let r327 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r327);
            let Some(t328) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000923c46); };
            c.w32(0, t328);
            let r329 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r329);
            let Some(t330) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000923c4c); };
            c.w32(0, t330);
            let r331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r331);
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923c53); };
            c.w32(7, t332);
            let r333 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r333);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923c59); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000923c5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000923c5e); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923c64); };
            c.w64(6, t334);
            return c.call(0x70007f2274, 0x7000923c6d, 0x7000923c68);
        }
        128 => {
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923c6d); };
            c.w64(8, t335);
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923c71); };
            c.w64(8, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923c75); };
            c.w64(9, t337);
            let Some(t338) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000923c79); };
            c.w32(11, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000923c7e); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000923c82); };
            c.w64(1, t340);
            let Some(t341) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000923c86); };
            c.w64(5, t341);
            c.w64(12, c.r64(8));
            let r342 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r342);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t343) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923c93); };
            c.w32(3, t343);
            let r344 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r344);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000923c9e); };
            c.w64(3, t345);
            blk = 131; continue;
        }
        131 => {
            let Some(t346) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923ca5); };
            c.w32(2, t346);
            let Some(t347) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923ca8); };
            c.w32(1, t347);
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923cad); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t350) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923cb8); };
            c.w32(1, t350);
            let r351 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r351);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r352 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r352);
            let Some(t353) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923cc6); };
            c.w32(6, t353);
            let r354 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r354);
            let r355 = c.r64(6) & c.r64(2);
            c.w64(6, r355);
            c.logic(r355, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t356) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923cdc); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000923ce1); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r358 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r358);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r359 = c.r64(0) & c.r64(8);
            c.w64(0, r359);
            c.logic(r359, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t360) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923d05); };
            c.w32(11, t360);
            let Some(t361) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923d09); };
            c.w32(0, t361);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x7000923d1d);
        }
        137 => {
            let Some(t362) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000923d1e); };
            c.w32(11, t362);
            let Some(t363) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923d23); };
            c.w32(8, t363);
            let r364 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r364);
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923d2b); };
            c.w32(11, t365);
            let r366 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r366);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923d3b); };
            c.w32(0, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000923d3f); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923d4c); };
            c.w32(2, t369);
            let r370 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r370);
            let Some(t371) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923d53); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000923d5a); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000923d65); };
            c.cmp(t374, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000923d6e); };
            c.w64(8, t375);
            blk = 188; continue;
        }
        142 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000923d7a); };
            c.w64(8, t376);
            let Some(t377) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000923d81); };
            c.cmp(t377, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t378) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000923d8b); };
            c.w32(8, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923d8f); };
            c.cmp(t379, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923d98); };
            c.w64(8, t380);
            blk = 263; continue;
        }
        145 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000923da1); };
            c.w64(0, t381);
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923da5); };
            c.w32(8, t382);
            let r383 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r383);
            let Some(t384) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923dac); };
            c.w32(11, t384);
            let Some(t385) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923db0); };
            c.cmp(t385, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000923db9); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923dc0); };
            c.w32(11, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923dc4); };
            c.cmp(t389, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t390) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923dd7); };
            c.w32(11, t390);
            let r391 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r391);
            let Some(t392) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923dde); };
            c.w32(3, t392);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000923dfb); };
            c.cmp(t393, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923e04); };
            c.w64(8, t394);
            blk = 263; continue;
        }
        152 => {
            let Some(t395) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923e0d); };
            c.w32(0, t395);
            let r396 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r396);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t397) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923e18); };
            c.w32(8, t397);
            let r398 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r398);
            let Some(t399) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923e1f); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923e26); };
            c.w32(11, t401);
            let r402 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r402);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r403 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r403);
            let Some(t404) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000923e3f); };
            c.w32(6, t404);
            let r405 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923e46); };
            c.w64(8, t406);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923e4a); }
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923e4e); };
            if !c.push64(t407) { return RegfileExit::fault(0x7000923e4e); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000923e58, 0x7000923e53);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923e5b); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        157 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923e64); };
            c.w64(8, t409);
            blk = 263; continue;
        }
        158 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923e6d); };
            c.w64(8, t410);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w64(0, c.r64(8).wrapping_sub(0x1));
            let r411 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923e7f); };
            c.w64(3, t412);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000923e83); }
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000923e87); };
            if !c.push64(t413) { return RegfileExit::fault(0x7000923e87); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000923e8a); }
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923e8c); };
            if !c.push64(t414) { return RegfileExit::fault(0x7000923e8c); }
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923e8f); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000923e93); };
            c.w64(3, t416);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000923ea7, 0x7000923ea2);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923eaa); };
            c.w64(9, t417);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r418 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923ebd); };
            c.w64(0, t419);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000923ec1); }
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000923ec5); };
            if !c.push64(t420) { return RegfileExit::fault(0x7000923ec5); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000923ecd); };
            c.w64(6, t421);
            return c.call(0x70007f2274, 0x7000923ed6, 0x7000923ed1);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000923ed9); };
            c.w64(9, t422);
            blk = 188; continue;
        }
        163 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000923ee2); };
            c.w64(0, t423);
            let Some(t424) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923ee6); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923eed); };
            c.w32(3, t426);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 185; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923f0f); };
            c.w64(8, t427);
            blk = 263; continue;
        }
        167 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923f18); };
            c.w32(8, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000923f1c); };
            c.cmp(t429, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923f25); };
            c.w64(8, t430);
            blk = 263; continue;
        }
        169 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923f2e); };
            c.w32(8, t431);
            let r432 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r432);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923f3b); };
            c.w64(8, t433);
            blk = 263; continue;
        }
        171 => {
            let Some(t434) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923f44); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923f4b); };
            c.cmp(t436, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923f53); };
            c.w64(8, t437);
            blk = 263; continue;
        }
        173 => {
            let Some(t438) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000923f5c); };
            c.w32(8, t438);
            let r439 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r439);
            blk = 188; continue;
        }
        174 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923f68); };
            c.w32(8, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000923f6c); };
            c.cmp(t441, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923f75); };
            c.w64(8, t442);
            blk = 263; continue;
        }
        176 => {
            let Some(t443) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000923f7e); };
            c.w32(8, t443);
            c.w64(11, c.r64(8));
            let r444 = c.r32(11) & 0xc0u64;
            c.w32(11, r444);
            c.logic(r444, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000923f96); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            let Some(t447) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000923f9d); };
            c.w32(11, t447);
            let r448 = c.r32(11) & 0x1fu64;
            c.w32(11, r448);
            c.logic(r448, 32);
            let r449 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r449);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923faf); };
            c.w64(8, t450);
            blk = 263; continue;
        }
        179 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000923fbe); };
            c.w32(8, t451);
            let r452 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r452);
            let Some(t453) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000923fc5); };
            c.cmp(t453, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000923fce); };
            c.w32(11, t454);
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000923fd2); };
            c.cmp(t455, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000923fdb); };
            c.w32(8, t456);
            let r457 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r457);
            blk = 188; continue;
        }
        183 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923fe4); };
            c.w64(8, t458);
            blk = 263; continue;
        }
        184 => {
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000923fed); };
            c.w64(8, t459);
            blk = 263; continue;
        }
        185 => {
            let Some(t460) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000923ff6); };
            c.w32(8, t460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000923ffa); };
            c.cmp(t461, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924003); };
            c.w64(8, t462);
            blk = 263; continue;
        }
        187 => {
            let Some(t463) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092400c); };
            c.w32(8, t463);
            let r464 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r464);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092401a); };
            c.w64(0, t465);
            let Some(t466) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092401e); };
            c.w64(11, t466);
            let Some(t467) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000924022); };
            c.w32(0, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924027); };
            c.w64(15, t468);
            let Some(t469) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x700092402b); };
            c.w64(1, t469);
            let Some(t470) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092402f); };
            c.w64(5, t470);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r471 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r471);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092403f); };
            c.w64(8, t472);
            blk = 263; continue;
        }
        190 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r473 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r473);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t474) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924053); };
            c.w64(3, t474);
            let Some(t475) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924057); };
            c.w32(2, t475);
            let Some(t476) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092405c); };
            c.w32(3, t476);
            let r477 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r477);
            let Some(t478) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924062); };
            c.w32(3, t478);
            let r479 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r479);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924069); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092406d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092406e); }
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000924072); };
            c.w64(3, t480);
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924079); };
            c.w64(6, t481);
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700092408a, 0x7000924085);
        }
        191 => {
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092408a); };
            c.w64(8, t482);
            let Some(t483) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092408e); };
            c.w64(8, t483);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000924092); };
            c.w64(9, t484);
            let Some(t485) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000924096); };
            c.w32(11, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092409b); };
            c.w64(15, t486);
            let Some(t487) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092409f); };
            c.w64(1, t487);
            let Some(t488) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009240a3); };
            c.w64(5, t488);
            c.w64(12, c.r64(8));
            let r489 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r489);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let r490 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r490);
            let Some(t491) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009240b4); };
            c.w32(8, t491);
            let Some(t492) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009240b9); };
            c.w64(1, t492);
            let Some(t493) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009240bd); };
            c.w64(5, t493);
            let r494 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r494);
            return RegfileExit::jump(c.r64(1));
        }
        193 => {
            c.logic(c.r8(1) & 0x40u64, 8);
            if c.cc_ne() { blk = 207; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w64(11, c.r64(1));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(1) & 0x100u64, 16);
            c.w8(3, (c.cc_ne() as u64));
            c.w32(3, c.r8(3));
            let r495 = c.r64(11) & 0x7ffu64;
            c.w64(11, r495);
            c.logic(r495, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t496) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009240ee); };
            c.w32(0, t496);
            let r497 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r497);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t498) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009240f8); };
            c.w64(0, t498);
            blk = 197; continue;
        }
        197 => {
            let Some(t499) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009240ff); };
            c.w32(0, t499);
            let r500 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r500);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let r501 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r501);
            let Some(t502) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924110); };
            c.w32(8, t502);
            let Some(t503) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924115); };
            c.w64(1, t503);
            let Some(t504) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924119); };
            c.w64(5, t504);
            let r505 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(c.r64(1));
        }
        199 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t506) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092412b); };
            c.w32(11, t506);
            let Some(t507) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092412f); };
            c.cmp(t507, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 201; continue;
        }
        201 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092413c); }
            blk = 255; continue;
        }
        202 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let r508 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r508);
            let Some(t509) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092414f); };
            c.w32(8, t509);
            let Some(t510) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924154); };
            c.w32(0, t510);
            let r511 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r511);
            let Some(t512) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092415c); };
            c.w64(1, t512);
            let Some(t513) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924160); };
            c.w64(5, t513);
            let r514 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r514);
            return RegfileExit::jump(c.r64(1));
        }
        204 => {
            let Some(t515) = c.ld32(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092416a); };
            c.w32(11, t515);
            let r516 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r516);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t517) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092417c); };
            c.w32(0, t517);
            let Some(t518) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000924180); };
            c.cmp(t518, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700092418d); }
            blk = 255; continue;
        }
        207 => {
            c.w64(3, c.r64(1));
            let r519 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r519);
            let r520 = c.r32(3) & 0xfu64;
            c.w32(3, r520);
            c.logic(r520, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_l() { blk = 214; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 218; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 231; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            return RegfileExit::fault(0x70009241fe);
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r521 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r521);
            let r522 = c.r64(1) & 0xfffffu64;
            c.w64(1, r522);
            c.logic(r522, 64);
            let Some(t523) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092420e); };
            c.w32(8, (t523 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t524) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924218); };
            c.w32(8, t524);
            let Some(t525) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092421d); };
            c.w64(1, t525);
            let Some(t526) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924221); };
            c.w64(5, t526);
            let r527 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r527);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r528 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r528);
            let r529 = c.r64(1) & 0xfffffu64;
            c.w64(1, r529);
            c.logic(r529, 64);
            let Some(t530) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092423a); };
            c.w32(8, (t530 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t531) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924244); };
            c.w32(8, t531);
            let Some(t532) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924249); };
            c.w64(1, t532);
            let Some(t533) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092424d); };
            c.w64(5, t533);
            let r534 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r534);
            return RegfileExit::jump(c.r64(1));
        }
        223 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r535 = c.r64(1) & 0xfffffu64;
            c.w64(1, r535);
            c.logic(r535, 64);
            let Some(t536) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924262); };
            c.w32(8, t536);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000924267); }
            c.w64(11, c.r64(8));
            let r537 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r537);
            if c.cc_o() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 230; continue;
        }
        225 => {
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092427b); };
            c.w64(11, t538);
            let Some(t539) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000924282); };
            c.w64(0, t539);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t540) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000924289); };
            c.w64(2, t540);
            let Some(t541) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000924290); };
            c.cmp(t541, c.r64(3), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092429f, 0x700092429a);
        }
        227 => {
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092429f); };
            c.w64(8, t542);
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009242a3); };
            c.w64(15, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009242a7); };
            c.w64(9, t544);
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009242ab); };
            c.w64(12, t545);
            blk = 229; continue;
        }
        228 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70009242b1); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 229; continue;
        }
        229 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009242c4); };
            c.w64(8, t546);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009242cb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009242cf); }
            blk = 230; continue;
        }
        230 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0xd));
            let Some(t547) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009242db); };
            c.w32(8, t547);
            let Some(t548) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009242e0); };
            c.w64(1, t548);
            let Some(t549) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009242e4); };
            c.w64(5, t549);
            let r550 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r550);
            return RegfileExit::jump(c.r64(1));
        }
        231 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r551 = c.r64(1) & 0xfffffu64;
            c.w64(1, r551);
            c.logic(r551, 64);
            let Some(t552) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009242f9); };
            c.w32(8, t552);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009242fe); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let r553 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r553);
            blk = 238; continue;
        }
        233 => {
            let Some(t554) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000924310); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000924317); };
            c.w64(0, t555);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t556) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092431e); };
            c.w64(2, t556);
            let Some(t557) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000924325); };
            c.cmp(t557, c.r64(3), 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000924334, 0x700092432f);
        }
        235 => {
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924334); };
            c.w64(8, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924338); };
            c.w64(15, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092433c); };
            c.w64(9, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000924340); };
            c.w64(12, t561);
            blk = 237; continue;
        }
        236 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000924346); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 237; continue;
        }
        237 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t562) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092435c); };
            c.w64(8, t562);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000924363); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000924367); }
            c.w64(8, c.r64(0));
            blk = 238; continue;
        }
        238 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t563) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924376); };
            c.w32(11, t563);
            let Some(t564) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092437b); };
            c.w64(1, t564);
            let Some(t565) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092437f); };
            c.w64(5, t565);
            c.w64(0, c.r64(8));
            let r566 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r566);
            return RegfileExit::jump(c.r64(1));
        }
        239 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r567 = c.r64(1) & 0xfffffu64;
            c.w64(1, r567);
            c.logic(r567, 64);
            let Some(t568) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924397); };
            c.w64(8, t568);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092439c); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 247; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009243a6); };
            c.w64(11, t569);
            let Some(t570) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009243ad); };
            c.w64(0, t570);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009243b4); };
            c.w64(2, t571);
            let Some(t572) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009243bb); };
            c.cmp(t572, c.r64(3), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70009243ca, 0x70009243c5);
        }
        242 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009243ca); };
            c.w64(8, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009243ce); };
            c.w64(15, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009243d2); };
            c.w64(9, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009243d6); };
            c.w64(12, t576);
            blk = 244; continue;
        }
        243 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70009243dc); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 244; continue;
        }
        244 => {
            let Some(t577) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70009243e6); };
            c.w64(3, t577);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009243ed); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70009243f6); }
            let r578 = c.neg(c.r64(8), 64);
            c.w64(8, r578);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000924400); }
            blk = 252; continue;
        }
        246 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000924406); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700092440d); }
            blk = 252; continue;
        }
        247 => {
            let Some(t579) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000924413); };
            c.w64(8, t579);
            let Some(t580) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092441a); };
            c.w64(11, t580);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t581) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000924421); };
            c.w64(0, t581);
            let Some(t582) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000924428); };
            c.cmp(t582, c.r64(3), 64);
            if c.cc_a() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000924437, 0x7000924432);
        }
        249 => {
            blk = 251; continue;
        }
        250 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x7000924439); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 251; continue;
        }
        251 => {
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000924443); };
            c.w64(8, t583);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700092444a); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700092444e); }
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000924455); };
            c.w64(12, t584);
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924459); };
            c.w64(15, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092445d); };
            c.w64(9, t586);
            blk = 252; continue;
        }
        252 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0xd));
            let Some(t587) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924468); };
            c.w32(8, t587);
            let Some(t588) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092446d); };
            c.w64(1, t588);
            let Some(t589) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924471); };
            c.w64(5, t589);
            let r590 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r590);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r591 = c.r64(1) & 0xfffffu64;
            c.w64(1, r591);
            c.logic(r591, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000924486); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 255; continue;
        }
        254 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r592 = c.r64(1) & 0xfffffu64;
            c.w64(1, r592);
            c.logic(r592, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x700092449d); }
            blk = 255; continue;
        }
        255 => {
            let Some(t593) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009244a3); };
            c.w64(8, t593);
            let Some(t594) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009244aa); };
            c.w64(11, t594);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70009244b1); }
            let Some(t595) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009244b6); };
            c.w64(3, t595);
            let Some(t596) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009244bd); };
            c.cmp(t596, c.r64(0), 64);
            if c.cc_a() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009244cc, 0x70009244c7);
        }
        257 => {
            blk = 259; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70009244ce); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 259; continue;
        }
        259 => {
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009244d8); };
            c.w64(3, t597);
            c.w64(9, c.r64(3).wrapping_add(0xd));
            let Some(t598) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009244e0); };
            c.w64(8, t598);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009244e7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70009244eb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009244f0); }
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009244f5); };
            c.w64(12, t599);
            let Some(t600) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009244f9); };
            c.w32(8, t600);
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009244fe); };
            c.w64(15, t601);
            let Some(t602) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924502); };
            c.w64(1, t602);
            let Some(t603) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924506); };
            c.w64(5, t603);
            let r604 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r604);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r605 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r605);
            let r606 = c.r64(1) & 0xfffffu64;
            c.w64(1, r606);
            c.logic(r606, 64);
            let Some(t607) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092451f); };
            c.w32(8, t607);
            let Some(t608) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924524); };
            c.w32(0, t608);
            let r609 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r609);
            let Some(t610) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092452b); };
            c.w64(1, t610);
            let Some(t611) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092452f); };
            c.w64(5, t611);
            let r612 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r612);
            return RegfileExit::jump(c.r64(1));
        }
        261 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r613 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r613);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092453f); }
            let Some(t614) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000924543); };
            c.w64(8, t614);
            let Some(t615) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092454a); };
            c.w32(8, t615);
            let r616 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r616);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000924551); }
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000924553); };
            c.w64(3, t617);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000924567, 0x7000924562);
        }
        262 => {
            return RegfileExit::fault(0x7000924567);
        }
        263 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r618 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r618);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t619) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924573); };
            c.w32(3, t619);
            let Some(t620) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924578); };
            c.w32(2, t620);
            let r621 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r621);
            let Some(t622) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924580); };
            c.w64(1, t622);
            let Some(t623) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924584); };
            c.w32(3, t623);
            let r624 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r624);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092458b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092458f); }
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000924590); };
            if !c.push64(t625) { return RegfileExit::fault(0x7000924590); }
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000924593); };
            if !c.push64(t626) { return RegfileExit::fault(0x7000924593); }
            c.w32(11, 0x2u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092459c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700092459e); }
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70009245a2); };
            c.w64(3, t627);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70009245b9, 0x70009245b4);
        }
        264 => {
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009245b9); };
            c.w64(8, t628);
            let Some(t629) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009245bd); };
            c.w64(8, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009245c1); };
            c.w64(9, t630);
            let Some(t631) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009245c5); };
            c.w32(11, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009245ca); };
            c.w64(15, t632);
            let Some(t633) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009245ce); };
            c.w64(1, t633);
            let Some(t634) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009245d2); };
            c.w64(5, t634);
            c.w64(12, c.r64(8));
            let r635 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r635);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x70009245df);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009245e0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009245e4); }
            let r636 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r636);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r637 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r637);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t638) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009245f5); };
            c.w32(3, t638);
            let Some(t639) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009245fa); };
            c.w32(2, t639);
            let r640 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r640);
            let Some(t641) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924602); };
            c.w64(1, t641);
            let Some(t642) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924606); };
            c.w32(3, t642);
            let r643 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r643);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092460d); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000924611); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000924615); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000924619); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xcu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x7000924630, 0x700092462b);
        }
        267 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924630); };
            c.w64(8, t644);
            let Some(t645) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924634); };
            c.w64(8, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000924638); };
            c.w64(9, t646);
            let Some(t647) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092463c); };
            c.w32(11, t647);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924641); };
            c.w64(15, t648);
            let Some(t649) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924645); };
            c.w64(1, t649);
            let Some(t650) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924649); };
            c.w64(5, t650);
            c.w64(12, c.r64(8));
            let r651 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r651);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000924656); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092465a); }
            let r652 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r652);
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r653 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r653);
            let Some(t654) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924666); };
            c.w32(3, t654);
            let Some(t655) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092466b); };
            c.w32(2, t655);
            let r656 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r656);
            let Some(t657) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924673); };
            c.w32(3, t657);
            let r658 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r658);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092467a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092467e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092467f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000924683); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000924687); }
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x700092468b); };
            c.w64(3, t659);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x700092469f, 0x700092469a);
        }
        269 => {
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092469f); };
            c.w64(8, t660);
            let Some(t661) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009246a3); };
            c.w64(8, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009246a7); };
            c.w64(11, t662);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t663) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009246af); };
            c.w32(11, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009246b4); };
            c.w64(15, t664);
            let Some(t665) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009246b8); };
            c.w64(1, t665);
            let Some(t666) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009246bc); };
            c.w64(5, t666);
            c.w64(12, c.r64(8));
            let r667 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r667);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x70009246cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009246d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009246d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009246d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009246de); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009246e3); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009246eb); };
            c.w64(8, t4);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009246f3); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009246fa); };
            c.w64(7, t7);
            let r8 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r8);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924701); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000924705); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000924706); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092470a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092470e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x7000924712); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x7000924726, 0x7000924721);
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924726); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092472a); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092472e); };
            c.w64(7, t12);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000924736); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092473b); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x700092473f); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924743); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x7000924754);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000924774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924777); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092477b); };
            c.w32(8, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000924780); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924788); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924797); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092479d); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009247b5); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009247b8); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009247c7); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r32(11), 0xffffffffffffffffu64, 32);
            c.w32(11, r11);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70009247d5);
        }
        9 => {
            return RegfileExit::fault(0x70009247d6);
        }
        10 => {
            return RegfileExit::fault(0x70009247d7);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009247d8); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70009247e6);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009247eb); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009247f0); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009247f8); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009247fe); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000924802); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924808); };
            c.w32(8, t19);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924811); };
            c.w32(0, t20);
            let r21 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r21);
            let r22 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            let Some(t23) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092481c); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924824); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000924828); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092482a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092482e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x7000924832); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x7000924836); };
            c.w64(3, t25);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700092484a, 0x7000924845);
        }
        15 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092484a); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092484e); };
            c.w64(2, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000924852); };
            c.w64(9, t28);
            let r29 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r29);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700092485a); };
            c.w32(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092485f); };
            c.w64(15, t31);
            let Some(t32) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924863); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924867); };
            c.w64(5, t33);
            c.w64(12, c.r64(2));
            let r34 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r34);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x7000924878);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000924894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924897); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092489b); };
            c.w32(0, t1);
            c.w64(3, c.r64(11));
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009248a3); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009248b2); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009248b8); };
            c.w32(1, t5);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009248d0); };
            c.w32(1, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009248d3); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_ne() { blk = 268; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009248e6); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let r10 = c.add(c.r32(0), 0xffffffffffffffffu64, 32);
            c.w32(0, r10);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70009248f3);
        }
        9 => {
            return RegfileExit::fault(0x70009248f4);
        }
        10 => {
            return RegfileExit::fault(0x70009248f5);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009248f6); };
            c.w64(11, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009248fa); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924901); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            let Some(t16) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924908); };
            c.w32(0, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092490c); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000924915); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092491c); };
            c.cmp(t19, c.r32(11), 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000924929); };
            c.w64(0, (t20 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t21) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000924935); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let r23 = c.r64(2) & 0xfffffffffffffffdu64;
            c.w64(2, r23);
            c.logic(r23, 64);
            let Some(t24) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092494f); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000924955); };
            c.cmp(t26, c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r27);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924966); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092496b); };
            c.w64(1, t29);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924974); };
            c.w64(8, t30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924978); };
            c.w32(8, t31);
            let r32 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092497f); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924986); };
            c.w32(11, t35);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x7000924999);
        }
        20 => {
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700092499a); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009249a7); };
            c.w32(0, t38);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r39 = c.r64(3) & 0xfffu64;
            c.w64(3, r39);
            c.logic(r39, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 265; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009249cb); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009249d3); };
            c.w32(0, t42);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r43 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r43);
            c.logic(r43, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70009249ed);
        }
        23 => {
            let r44 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r44);
            let Some(t45) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009249f2); };
            c.w32(0, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009249f7); };
            c.w32(8, t46);
            let r47 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r47);
            let Some(t48) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70009249ff); };
            c.w64(1, t48);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        24 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000924a08); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000924a0c); }
            let r49 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r49);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000924a12); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000924a16); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000924a1a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000924a1e); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000924a2a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000924a2e); }
            let r50 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(3)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000924a34); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000924a3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000924a40); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000924a44); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000924a48); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000924a4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000924a50); }
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000924a54); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 262; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t54) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000924a61); };
            c.w64(3, t54);
            let Some(t55) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924a65); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000924a6b); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924a71); };
            c.w32(1, t59);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000924a7a); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x7000924a84);
        }
        28 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000924a85); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000924a8b); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x7000924a95);
        }
        30 => {
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000924a96); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924aa6); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000924aa9); };
            c.cmp(t65, c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924ab2); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924ac0); };
            c.cmp(t68, c.r32(11), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924aca); };
            c.w32(1, t69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000924acd); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000924ad7); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 37; continue;
        }
        37 => {
            let r73 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r73);
            c.logic(r73, 64);
            blk = 46; continue;
        }
        38 => {
            c.w64(0, c.r64(2));
            blk = 46; continue;
        }
        39 => {
            let r74 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r74);
            let Some(t75) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924afe); };
            c.w32(8, t75);
            let Some(t76) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924b03); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924b07); };
            c.w64(5, t77);
            let r78 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r78);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r79 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r79);
            c.w64(6, c.r64(9).wrapping_add(0xd));
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924b1b); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924b20); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924b28); };
            c.w32(9, t83);
            let r84 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r84);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000924b30); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000924b34); }
            c.w64(1, c.r64(9));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(11));
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000924b41); };
            c.logic(t85 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r86 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r86);
            blk = 43; continue;
        }
        42 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000924b50); };
            c.w32(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x7000924b53); };
            c.w64(7, t88);
            blk = 43; continue;
        }
        43 => {
            return c.call(c.r64(7), 0x7000924b5d, 0x7000924b5b);
        }
        44 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924b5d); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924b61); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000924b65); };
            c.w64(9, t91);
            let Some(t92) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000924b69); };
            c.w32(11, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924b6e); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924b72); };
            c.w64(1, t94);
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924b76); };
            c.w64(5, t95);
            c.w64(12, c.r64(8));
            let r96 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r96);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(7));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(3, c.r64(1));
            let r97 = c.r64(3) & 0xfu64;
            c.w64(3, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000924b99); }
            c.cmp(c.r64(3), 0x4u64, 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_e() { blk = 191; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(3), 0xdu64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_e() { blk = 129; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(3), 0x6u64, 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 122; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(3), 0x8u64, 64);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(3), 0x9u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(3), 0x3u64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(3), 0xcu64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(3), 0xeu64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r98 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r98);
            c.w64(3, c.r64(9).wrapping_add(0xd));
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924c27); };
            c.w32(2, t99);
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924c2c); };
            c.w32(1, t100);
            let r101 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r101);
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924c34); };
            c.w32(2, t102);
            let r103 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r103);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924c3b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000924c3f); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924c40); };
            if !c.push64(t104) { return RegfileExit::fault(0x7000924c40); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000924c43); }
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000924c44); };
            if !c.push64(t105) { return RegfileExit::fault(0x7000924c44); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000924c47); };
            if !c.push64(t106) { return RegfileExit::fault(0x7000924c47); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x7000924c4a); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000924c4e); };
            c.w64(6, t107);
            c.w32(0, 0x5u64);
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000924c57); };
            c.w64(3, t108);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000924c66, 0x7000924c61);
        }
        59 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924c66); };
            c.w64(8, t109);
            let Some(t110) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924c6a); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000924c6e); };
            c.w64(9, t111);
            let Some(t112) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000924c72); };
            c.w32(11, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924c77); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924c7b); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924c7f); };
            c.w64(5, t115);
            c.w64(12, c.r64(8));
            let r116 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let Some(t117) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000924c8c); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924c93); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924c9a); };
            c.w32(0, t121);
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r122 = c.r64(1) & 0x1ffffffu64;
            c.w64(1, r122);
            c.logic(r122, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return RegfileExit::fault(0x7000924cb4);
        }
        62 => {
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924cb5); };
            c.w32(11, t123);
            let r124 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r124);
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924cbd); };
            c.w32(0, t125);
            let r126 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r126);
            c.w64(11, c.r64(9).wrapping_add(0xd));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000924cc8); }
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000924ccc); };
            c.cmp(t127, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t128) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000924cd5); };
            c.w32(8, t128);
            let Some(t129) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924cda); };
            c.w64(1, t129);
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924cde); };
            c.w64(5, t130);
            c.w64(9, c.r64(11));
            let r131 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r131);
            return RegfileExit::jump(c.r64(1));
        }
        64 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r132 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r132);
            let Some(t133) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924cf1); };
            c.w32(3, t133);
            let Some(t134) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924cf6); };
            c.w32(2, t134);
            let r135 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r135);
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924cfe); };
            c.w32(3, t136);
            let r137 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r137);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000924d05); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000924d0e); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000924d0f); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x7000924d10); };
            c.w64(3, t138);
            c.w32(0, 0x2u64);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000924d1c); };
            c.w64(6, t139);
            return c.call(0x700086ed74, 0x7000924d25, 0x7000924d20);
        }
        65 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924d25); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924d29); };
            c.w64(8, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000924d2d); };
            c.w64(9, t142);
            let Some(t143) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000924d31); };
            c.w32(11, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924d36); };
            c.w64(15, t144);
            let Some(t145) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924d3a); };
            c.w64(1, t145);
            let Some(t146) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924d3e); };
            c.w64(5, t146);
            c.w64(12, c.r64(8));
            let r147 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r147);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r148 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r148);
            c.w64(3, c.r64(9).wrapping_add(0xd));
            let Some(t149) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924d56); };
            c.w32(2, t149);
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924d5b); };
            c.w32(1, t150);
            let r151 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r151);
            let Some(t152) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924d63); };
            c.w32(2, t152);
            let r153 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r153);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924d6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000924d6e); }
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000924d72); };
            c.w64(6, t154);
            c.w64(3, c.r64(2));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924d79); };
            c.w64(1, t155);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x7000924d8a, 0x7000924d85);
        }
        67 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924d8a); };
            c.w64(8, t156);
            let Some(t157) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924d8e); };
            c.w64(8, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000924d92); };
            c.w64(9, t158);
            let Some(t159) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000924d96); };
            c.w32(11, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924d9b); };
            c.w64(15, t160);
            let Some(t161) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000924d9f); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000924da3); };
            c.w64(5, t162);
            c.w64(12, c.r64(8));
            let r163 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r163);
            return RegfileExit::jump(c.r64(1));
        }
        68 => {
            blk = 125; continue;
        }
        69 => {
            let Some(t164) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000924db5); };
            c.w32(3, t164);
            let r165 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r165);
            let Some(t166) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924dbb); };
            c.w32(2, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000924dbe); };
            c.cmp(t167, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t168) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000924dcb); };
            c.w32(1, t168);
            let Some(t169) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924dd0); };
            c.w32(6, t169);
            let r170 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r170);
            let Some(t171) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924dd8); };
            c.w32(1, t171);
            let r172 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r172);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t173) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924de8); };
            c.w32(2, t173);
            let r174 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000924dee); };
            c.cmp(t175, c.r32(2), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t176) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924dfb); };
            c.w32(2, t176);
            let r177 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r177);
            let Some(t178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924e01); };
            c.w32(6, t178);
            let r179 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r179);
            let Some(t180) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000924e07); };
            c.logic(t180 & 0x2u64, 8);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000924e11); };
            c.cmp(t181, c.r32(6), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000924e1a); };
            c.w64(3, t182);
            blk = 109; continue;
        }
        75 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000924e26); };
            c.w64(6, t183);
            let Some(t184) = c.ld8(c.r64(6)) else { return RegfileExit::fault(0x7000924e2d); };
            c.cmp(t184, 0x20u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t185) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000924e36); };
            c.w32(1, t185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000924e39); };
            c.cmp(t186, c.r32(1), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924e46); };
            c.w32(1, t187);
            let r188 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r188);
            let Some(t189) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924e4c); };
            c.w32(6, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000924e4f); };
            c.cmp(t190, c.r32(6), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000924e58); };
            c.w32(1, t191);
            let r192 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r192);
            let Some(t193) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924e5e); };
            c.w32(6, t193);
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000924e61); };
            c.cmp(t194, c.r32(6), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t195) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924e73); };
            c.w32(6, t195);
            let r196 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r196);
            let Some(t197) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924e79); };
            c.w32(7, t197);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(1), 0x82au64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000924e95); };
            c.cmp(t198, c.r32(6), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000924e9e); };
            c.w64(12, t199);
            blk = 263; continue;
        }
        84 => {
            c.w64(0, c.r64(3));
            blk = 86; continue;
        }
        85 => {
            c.w64(0, c.r64(1));
            blk = 86; continue;
        }
        86 => {
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000924eaf); };
            c.w32(0, t200);
            let r201 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r201);
            let Some(t202) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924eb5); };
            c.w32(0, t202);
            let r203 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r203);
            let Some(t204) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924ebb); };
            c.w32(3, t204);
            let r205 = c.sub(c.r32(3), 0x89u64, 32);
            c.w32(3, r205);
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(3, c.r64(9).wrapping_sub(0x1));
            let r206 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r206);
            let Some(t207) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000924ed0); };
            c.w32(6, t207);
            let r208 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r208);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000924ed6); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924eda); };
            if !c.push64(t209) { return RegfileExit::fault(0x7000924eda); }
            c.w64(1, c.r64(11));
            return c.call(0x70007f6fb4, 0x7000924ee5, 0x7000924ee0);
        }
        88 => {
            c.w64(3, c.r64(0));
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924ee8); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000924eec); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924ef0); };
            c.w64(8, t212);
            blk = 109; continue;
        }
        89 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000924ef9); };
            c.w64(12, t213);
            blk = 263; continue;
        }
        90 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r214 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r214);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000924f09); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000924f0d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000924f0e); }
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924f0f); };
            if !c.push64(t215) { return RegfileExit::fault(0x7000924f0f); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000924f12); };
            c.w64(6, t216);
            c.w32(0, 0x3u64);
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000924f1b); };
            c.w64(3, t217);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000924f2a, 0x7000924f25);
        }
        91 => {
            c.w64(3, c.r64(0));
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924f2d); };
            c.w64(15, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000924f31); };
            c.w64(9, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924f35); };
            c.w64(8, t220);
            blk = 109; continue;
        }
        92 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r221 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000924f47); }
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000924f4b); };
            if !c.push64(t222) { return RegfileExit::fault(0x7000924f4b); }
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000924f4e); };
            c.w64(6, t223);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000924f5c, 0x7000924f57);
        }
        93 => {
            c.w64(3, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000924f5f); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000924f63); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000924f67); };
            c.w64(8, t226);
            blk = 109; continue;
        }
        94 => {
            let Some(t227) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000924f70); };
            c.w32(1, t227);
            let r228 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r228);
            let Some(t229) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924f76); };
            c.w32(6, t229);
            c.cmp(c.r16(6), 0x834u64, 16);
            if c.cc_e() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 101; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r16(6), 0x411u64, 16);
            if c.cc_ne() { blk = 263; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t230) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924f9b); };
            c.w32(0, t230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000924f9e); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t232) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924fb4); };
            c.cmp(t232, 0x40u64, 16);
            if c.cc_ae() { blk = 263; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t233) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000924fbf); };
            c.w32(11, t233);
            c.w32(3, c.r64(11).wrapping_add(c.r64(11)));
            blk = 109; continue;
        }
        101 => {
            let Some(t234) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000924fcc); };
            c.w32(2, t234);
            let Some(t235) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000924fcf); };
            c.cmp(t235, c.r32(2), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t236) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000924fdc); };
            c.w32(2, t236);
            c.w64(1, c.r64(2));
            let r237 = c.r32(1) & 0xc0u64;
            c.w32(1, r237);
            c.logic(r237, 32);
            c.cmp(c.r32(1), 0xc0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t238) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000924ff1); };
            c.w32(3, t238);
            let r239 = c.r32(3) & 0x1fu64;
            c.w32(3, r239);
            c.logic(r239, 32);
            let r240 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r240);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_a() { blk = 263; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t241) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092500c); };
            c.w32(3, t241);
            let r242 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r242);
            let Some(t243) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000925012); };
            c.cmp(t243, c.r32(3), 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t244) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092501f); };
            c.w32(11, t244);
            let Some(t245) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000925023); };
            c.cmp(t245, c.r32(11), 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t246) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092502c); };
            c.w32(3, t246);
            let r247 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r247);
            blk = 109; continue;
        }
        108 => {
            let Some(t248) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925034); };
            c.w32(0, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000925037); };
            c.cmp(t249, c.r32(0), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 109; continue;
        }
        109 => {
            let r250 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r250);
            let Some(t251) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925048); };
            c.w64(8, t251);
            let Some(t252) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092504c); };
            c.w32(11, t252);
            let Some(t253) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925051); };
            c.w64(1, t253);
            let Some(t254) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925055); };
            c.w64(5, t254);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(3));
            let r255 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r255);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 263; continue;
        }
        111 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700092506a); };
            c.w64(3, t256);
            let Some(t257) = c.ld8(c.r64(3)) else { return RegfileExit::fault(0x7000925071); };
            c.cmp(t257, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t258) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092507f); };
            c.w32(3, t258);
            let r259 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r259);
            blk = 115; continue;
        }
        114 => {
            let Some(t260) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000925087); };
            c.w32(3, t260);
            let r261 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r261);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return RegfileExit::fault(0x7000925092);
        }
        117 => {
            let r262 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r262);
            c.logic(r262, 64);
            let Some(t263) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925097); };
            c.w32(1, t263);
            let r264 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r264);
            let Some(t265) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092509d); };
            c.w32(0, t265);
            let r266 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r266);
            let Some(t267) = c.ld64(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009250a3); };
            c.w64(1, t267);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t268) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009250ac); };
            c.w32(2, t268);
            let r269 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r269);
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009250b2); };
            c.w32(2, t270);
            let r271 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r271);
            blk = 119; continue;
        }
        119 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r272 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r272);
            let r273 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r273);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009250c2); }
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009250c6); };
            if !c.push64(t274) { return RegfileExit::fault(0x70009250c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009250c9); }
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            return c.call(0x70007fe8d4, 0x70009250e1, 0x70009250dc);
        }
        120 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009250e1); };
            c.w64(8, t275);
            let Some(t276) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009250e5); };
            c.w64(8, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009250e9); };
            c.w64(9, t277);
            let Some(t278) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009250ed); };
            c.w32(11, t278);
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009250f2); };
            c.w64(15, t279);
            let Some(t280) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009250f6); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009250fa); };
            c.w64(5, t281);
            c.w64(12, c.r64(8));
            let r282 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r282);
            return RegfileExit::jump(c.r64(1));
        }
        121 => {
            blk = 125; continue;
        }
        122 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000925109); };
            c.w64(11, t283);
            let Some(t284) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000925110); };
            c.cmp(t284, 0x20u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r285 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r285);
            let r286 = c.r64(1) & 0x3ffu64;
            c.w64(1, r286);
            c.logic(r286, 64);
            let r287 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r287);
            let r288 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r288);
            let Some(t289) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925130); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r290);
            let Some(t291) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000925136); };
            c.w32(2, t291);
            let r292 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r292);
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700092513c); };
            c.w32(3, t293);
            let r294 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r294);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925143); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000925147); }
            c.w64(2, c.r64(6));
            c.w64(1, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000925151); };
            c.w64(6, t295);
            return c.call(0x70007fea14, 0x700092515a, 0x7000925155);
        }
        124 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092515a); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092515e); };
            c.w64(8, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925162); };
            c.w64(9, t298);
            let Some(t299) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000925166); };
            c.w32(11, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092516b); };
            c.w64(15, t300);
            let Some(t301) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092516f); };
            c.w64(1, t301);
            let Some(t302) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925173); };
            c.w64(5, t302);
            c.w64(12, c.r64(8));
            let r303 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r303);
            return RegfileExit::jump(c.r64(1));
        }
        125 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r304 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r304);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t305) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092518b); };
            c.w32(3, t305);
            let Some(t306) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925190); };
            c.w32(2, t306);
            let r307 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r307);
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925198); };
            c.w32(3, t308);
            let r309 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r309);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092519f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009251a3); }
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009251a4); };
            if !c.push64(t310) { return RegfileExit::fault(0x70009251a4); }
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009251a7); };
            if !c.push64(t311) { return RegfileExit::fault(0x70009251a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70009251aa); }
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70009251ae); };
            c.w64(3, t312);
            c.w32(0, 0x3u64);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009251ba); };
            c.w64(6, t313);
            return c.call(0x700086ed74, 0x70009251c3, 0x70009251be);
        }
        126 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009251c3); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009251c7); };
            c.w64(8, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009251cb); };
            c.w64(9, t316);
            let Some(t317) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009251cf); };
            c.w32(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009251d4); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009251d8); };
            c.w64(1, t319);
            let Some(t320) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009251dc); };
            c.w64(5, t320);
            c.w64(12, c.r64(8));
            let r321 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r321);
            return RegfileExit::jump(c.r64(1));
        }
        127 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if let Some(sr) = c.shr(c.r64(1), 0x6u64, 64) { c.w64(1, sr); }
            let r322 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r322);
            let r323 = c.r64(1) & 0x3ffu64;
            c.w64(1, r323);
            c.logic(r323, 64);
            let r324 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r324);
            let r325 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(3, r325);
            let Some(t326) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925203); };
            c.w32(0, t326);
            let r327 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r327);
            let Some(t328) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000925209); };
            c.w32(0, t328);
            let r329 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r329);
            let Some(t330) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700092520f); };
            c.w32(0, t330);
            let r331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r331);
            let Some(t332) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925216); };
            c.w32(7, t332);
            let r333 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r333);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092521c); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000925220); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000925221); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000925227); };
            c.w64(6, t334);
            return c.call(0x70007f2274, 0x7000925230, 0x700092522b);
        }
        128 => {
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925230); };
            c.w64(8, t335);
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925234); };
            c.w64(8, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925238); };
            c.w64(9, t337);
            let Some(t338) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092523c); };
            c.w32(11, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925241); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925245); };
            c.w64(1, t340);
            let Some(t341) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925249); };
            c.w64(5, t341);
            c.w64(12, c.r64(8));
            let r342 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r342);
            return RegfileExit::jump(c.r64(1));
        }
        129 => {
            let Some(t343) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925256); };
            c.w32(3, t343);
            let r344 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r344);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000925261); };
            c.w64(3, t345);
            blk = 131; continue;
        }
        131 => {
            let Some(t346) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000925268); };
            c.w32(2, t346);
            let Some(t347) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092526b); };
            c.w32(1, t347);
            let Some(t348) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925270); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t350) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092527b); };
            c.w32(1, t350);
            let r351 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r351);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r352 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r352);
            let Some(t353) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925289); };
            c.w32(6, t353);
            let r354 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r354);
            let r355 = c.r64(6) & c.r64(2);
            c.w64(6, r355);
            c.logic(r355, 64);
            blk = 132; continue;
        }
        132 => {
            c.w64(15, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(15).wrapping_mul(4));
            let Some(t356) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092529f); };
            c.w32(0, t356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009252a4); };
            c.cmp(t357, c.r32(0), 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r358 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r358);
            c.w64(8, c.r64(11).wrapping_add(c.r64(6)));
            c.w64(0, c.r64(2));
            let r359 = c.r64(0) & c.r64(8);
            c.w64(0, r359);
            c.logic(r359, 64);
            c.w64(6, c.r64(0));
            blk = 132; continue;
        }
        135 => {
            let Some(t360) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009252c8); };
            c.w32(11, t360);
            let Some(t361) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009252cc); };
            c.w32(0, t361);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            return RegfileExit::fault(0x70009252e0);
        }
        137 => {
            let Some(t362) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009252e1); };
            c.w32(11, t362);
            let Some(t363) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009252e6); };
            c.w32(8, t363);
            let r364 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r364);
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009252ee); };
            c.w32(11, t365);
            let r366 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r366);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009252fe); };
            c.w32(0, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000925302); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092530f); };
            c.w32(2, t369);
            let r370 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r370);
            let Some(t371) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925316); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            let Some(t373) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092531d); };
            c.logic(t373 & 0x2u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t374) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000925328); };
            c.cmp(t374, c.r32(8), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000925331); };
            c.w64(8, t375);
            blk = 188; continue;
        }
        142 => {
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700092533d); };
            c.w64(8, t376);
            let Some(t377) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000925344); };
            c.cmp(t377, 0x20u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t378) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700092534e); };
            c.w32(8, t378);
            let Some(t379) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000925352); };
            c.cmp(t379, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092535b); };
            c.w64(8, t380);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        145 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000925367); };
            c.w64(0, t381);
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092536b); };
            c.w32(8, t382);
            let r383 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r383);
            let Some(t384) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925372); };
            c.w32(11, t384);
            let Some(t385) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000925376); };
            c.cmp(t385, c.r32(11), 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092537f); };
            c.w32(8, t386);
            let r387 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925386); };
            c.w32(11, t388);
            let Some(t389) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092538a); };
            c.cmp(t389, c.r32(11), 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t390) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092539d); };
            c.w32(11, t390);
            let r391 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r391);
            let Some(t392) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009253a4); };
            c.w32(3, t392);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 153; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t393) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009253c1); };
            c.cmp(t393, c.r32(11), 32);
            if c.cc_ne() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009253ca); };
            c.w64(8, t394);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        152 => {
            let Some(t395) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009253d6); };
            c.w32(0, t395);
            let r396 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r396);
            blk = 154; continue;
        }
        153 => {
            c.w64(0, c.r64(8));
            blk = 154; continue;
        }
        154 => {
            let Some(t397) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009253e1); };
            c.w32(8, t397);
            let r398 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r398);
            let Some(t399) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009253e8); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009253ef); };
            c.w32(11, t401);
            let r402 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r402);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r403 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r403);
            let Some(t404) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000925408); };
            c.w32(6, t404);
            let r405 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092540f); };
            c.w64(8, t406);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925413); }
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000925417); };
            if !c.push64(t407) { return RegfileExit::fault(0x7000925417); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000925421, 0x700092541c);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925424); };
            c.w64(9, t408);
            blk = 188; continue;
        }
        157 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092542d); };
            c.w64(8, t409);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        158 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925439); };
            c.w64(8, t410);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        159 => {
            c.w64(8, c.r64(9));
            c.w64(0, c.r64(8).wrapping_sub(0x1));
            let r411 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092544e); };
            c.w64(3, t412);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000925452); }
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000925456); };
            if !c.push64(t413) { return RegfileExit::fault(0x7000925456); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000925459); }
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092545b); };
            if !c.push64(t414) { return RegfileExit::fault(0x700092545b); }
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092545e); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000925462); };
            c.w64(3, t416);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000925476, 0x7000925471);
        }
        160 => {
            c.w64(8, c.r64(0));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925479); };
            c.w64(9, t417);
            blk = 188; continue;
        }
        161 => {
            c.w64(8, c.r64(9));
            c.w64(11, c.r64(8).wrapping_sub(0x1));
            let r418 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092548c); };
            c.w64(0, t419);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925490); }
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000925494); };
            if !c.push64(t420) { return RegfileExit::fault(0x7000925494); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092549c); };
            c.w64(6, t421);
            return c.call(0x70007f2274, 0x70009254a5, 0x70009254a0);
        }
        162 => {
            c.w64(8, c.r64(0));
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009254a8); };
            c.w64(9, t422);
            blk = 188; continue;
        }
        163 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009254b1); };
            c.w64(0, t423);
            let Some(t424) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009254b5); };
            c.w32(11, t424);
            let r425 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r425);
            let Some(t426) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009254bc); };
            c.w32(3, t426);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 185; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009254de); };
            c.w64(8, t427);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        167 => {
            let Some(t428) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009254ea); };
            c.w32(8, t428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009254ee); };
            c.cmp(t429, c.r32(8), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009254f7); };
            c.w64(8, t430);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        169 => {
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000925503); };
            c.w32(8, t431);
            let r432 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r432);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925510); };
            c.w64(8, t433);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        171 => {
            let Some(t434) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092551c); };
            c.w32(11, t434);
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            let Some(t436) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925523); };
            c.cmp(t436, 0x40u64, 16);
            if c.cc_b() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092552b); };
            c.w64(8, t437);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        173 => {
            let Some(t438) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925537); };
            c.w32(8, t438);
            let r439 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r439);
            blk = 188; continue;
        }
        174 => {
            let Some(t440) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925543); };
            c.w32(8, t440);
            let Some(t441) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000925547); };
            c.cmp(t441, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925550); };
            c.w64(8, t442);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        176 => {
            let Some(t443) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092555c); };
            c.w32(8, t443);
            c.w64(11, c.r64(8));
            let r444 = c.r32(11) & 0xc0u64;
            c.w32(11, r444);
            c.logic(r444, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t445) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000925574); };
            c.w32(11, t445);
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            let Some(t447) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092557b); };
            c.w32(11, t447);
            let r448 = c.r32(11) & 0x1fu64;
            c.w32(11, r448);
            c.logic(r448, 32);
            let r449 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r449);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092558d); };
            c.w64(8, t450);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        179 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t451) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092559f); };
            c.w32(8, t451);
            let r452 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r452);
            let Some(t453) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009255a6); };
            c.cmp(t453, c.r32(8), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t454) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009255af); };
            c.w32(11, t454);
            let Some(t455) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009255b3); };
            c.cmp(t455, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t456) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009255bc); };
            c.w32(8, t456);
            let r457 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r457);
            blk = 188; continue;
        }
        183 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009255c5); };
            c.w64(8, t458);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        184 => {
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009255d1); };
            c.w64(8, t459);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        185 => {
            let Some(t460) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009255dd); };
            c.w32(8, t460);
            let Some(t461) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70009255e1); };
            c.cmp(t461, c.r32(8), 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009255ea); };
            c.w64(8, t462);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        187 => {
            let Some(t463) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009255f6); };
            c.w32(8, t463);
            let r464 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r464);
            blk = 188; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925604); };
            c.w64(3, t465);
            let Some(t466) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925608); };
            c.w64(11, t466);
            let Some(t467) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700092560c); };
            c.w32(0, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925611); };
            c.w64(15, t468);
            let Some(t469) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925615); };
            c.w64(1, t469);
            let Some(t470) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925619); };
            c.w64(5, t470);
            c.w64(12, c.r64(11));
            c.w64(0, c.r64(8));
            let r471 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r471);
            return RegfileExit::jump(c.r64(1));
        }
        189 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925629); };
            c.w64(8, t472);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        190 => {
            let r473 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r473);
            let Some(t474) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925639); };
            c.w64(8, t474);
            let Some(t475) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092563d); };
            c.w32(11, t475);
            let Some(t476) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925642); };
            c.w64(1, t476);
            let Some(t477) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925646); };
            c.w64(5, t477);
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092564a); };
            c.w64(0, t478);
            c.w64(12, c.r64(8));
            let r479 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r479);
            return RegfileExit::jump(c.r64(1));
        }
        191 => {
            let r480 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r480);
            let Some(t481) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092565e); };
            c.w32(8, t481);
            let Some(t482) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925663); };
            c.w64(1, t482);
            let Some(t483) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925667); };
            c.w64(5, t483);
            let r484 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r484);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            c.logic(c.r8(1) & 0x40u64, 8);
            if c.cc_ne() { blk = 206; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w64(3, c.r64(1));
            if let Some(sr) = c.shr(c.r64(3), 0x9u64, 64) { c.w64(3, sr); }
            c.logic(c.r16(1) & 0x100u64, 16);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            let r485 = c.r64(3) & 0x7ffu64;
            c.w64(3, r485);
            c.logic(r485, 64);
            c.logic(c.r8(1) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 201; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t486) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925698); };
            c.w32(0, t486);
            let r487 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r487);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70009256a2); };
            c.w64(0, t488);
            blk = 196; continue;
        }
        196 => {
            let Some(t489) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009256a9); };
            c.w32(0, t489);
            let r490 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r490);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r491 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r491);
            let Some(t492) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009256b9); };
            c.w32(8, t492);
            let Some(t493) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009256be); };
            c.w64(1, t493);
            let Some(t494) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009256c2); };
            c.w64(5, t494);
            let r495 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r495);
            return RegfileExit::jump(c.r64(1));
        }
        198 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t496) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009256d4); };
            c.w32(3, t496);
            let Some(t497) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009256d7); };
            c.cmp(t497, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009256e4); }
            blk = 254; continue;
        }
        201 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let r498 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r498);
            let Some(t499) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009256f7); };
            c.w32(8, t499);
            let Some(t500) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009256fc); };
            c.w32(0, t500);
            let r501 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r501);
            let Some(t502) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925703); };
            c.w64(1, t502);
            let Some(t503) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925707); };
            c.w64(5, t503);
            let r504 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r504);
            return RegfileExit::jump(c.r64(1));
        }
        203 => {
            let Some(t505) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925711); };
            c.w32(0, t505);
            let r506 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r506);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 263; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t507) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925720); };
            c.w32(3, t507);
            let Some(t508) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925723); };
            c.cmp(t508, c.r32(3), 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000925730); }
            blk = 254; continue;
        }
        206 => {
            c.w64(3, c.r64(1));
            let r509 = c.shr(c.r32(3), 0x7u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r509);
            let r510 = c.r32(3) & 0xfu64;
            c.w32(3, r510);
            c.logic(r510, 32);
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_l() { blk = 213; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_l() { blk = 211; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 217; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            return RegfileExit::fault(0x70009257a0);
        }
        220 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r511 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r511);
            let r512 = c.r64(1) & 0xfffffu64;
            c.w64(1, r512);
            c.logic(r512, 64);
            let Some(t513) = c.ld8(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009257b0); };
            c.w32(8, (t513 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t514) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009257ba); };
            c.w32(8, t514);
            let Some(t515) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009257bf); };
            c.w64(1, t515);
            let Some(t516) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009257c3); };
            c.w64(5, t516);
            let r517 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r517);
            return RegfileExit::jump(c.r64(1));
        }
        221 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r518 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r518);
            let r519 = c.r64(1) & 0xfffffu64;
            c.w64(1, r519);
            c.logic(r519, 64);
            let Some(t520) = c.ld16(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009257dc); };
            c.w32(8, (t520 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t521) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009257e6); };
            c.w32(8, t521);
            let Some(t522) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009257eb); };
            c.w64(1, t522);
            let Some(t523) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009257ef); };
            c.w64(5, t523);
            let r524 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r524);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r525 = c.r64(1) & 0xfffffu64;
            c.w64(1, r525);
            c.logic(r525, 64);
            let Some(t526) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925804); };
            c.w32(8, t526);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000925809); }
            c.w64(11, c.r64(8));
            let r527 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r527);
            if c.cc_o() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 229; continue;
        }
        224 => {
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092581d); };
            c.w64(11, t528);
            let Some(t529) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000925824); };
            c.w64(0, t529);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092582b); };
            c.w64(2, t530);
            let Some(t531) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000925832); };
            c.cmp(t531, c.r64(3), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000925841, 0x700092583c);
        }
        226 => {
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925841); };
            c.w64(8, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925845); };
            c.w64(15, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925849); };
            c.w64(9, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092584d); };
            c.w64(12, t535);
            blk = 228; continue;
        }
        227 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000925853); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 228; continue;
        }
        228 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925866); };
            c.w64(8, t536);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700092586d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000925871); }
            blk = 229; continue;
        }
        229 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0xd));
            let Some(t537) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x700092587d); };
            c.w32(8, t537);
            let Some(t538) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925882); };
            c.w64(1, t538);
            let Some(t539) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925886); };
            c.w64(5, t539);
            let r540 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r540);
            return RegfileExit::jump(c.r64(1));
        }
        230 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r541 = c.r64(1) & 0xfffffu64;
            c.w64(1, r541);
            c.logic(r541, 64);
            let Some(t542) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092589b); };
            c.w32(8, t542);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009258a0); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            let r543 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r543);
            blk = 237; continue;
        }
        232 => {
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009258b2); };
            c.w64(11, t544);
            let Some(t545) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009258b9); };
            c.w64(0, t545);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009258c0); };
            c.w64(2, t546);
            let Some(t547) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70009258c7); };
            c.cmp(t547, c.r64(3), 64);
            if c.cc_a() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009258d6, 0x70009258d1);
        }
        234 => {
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009258d6); };
            c.w64(8, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009258da); };
            c.w64(15, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009258de); };
            c.w64(9, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009258e2); };
            c.w64(12, t551);
            blk = 236; continue;
        }
        235 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70009258e8); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 236; continue;
        }
        236 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t552) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009258fe); };
            c.w64(8, t552);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000925905); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000925909); }
            c.w64(8, c.r64(0));
            blk = 237; continue;
        }
        237 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t553) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000925918); };
            c.w32(11, t553);
            let Some(t554) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092591d); };
            c.w64(1, t554);
            let Some(t555) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925921); };
            c.w64(5, t555);
            c.w64(0, c.r64(8));
            let r556 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r556);
            return RegfileExit::jump(c.r64(1));
        }
        238 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r557 = c.r64(1) & 0xfffffu64;
            c.w64(1, r557);
            c.logic(r557, 64);
            let Some(t558) = c.ld64(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925939); };
            c.w64(8, t558);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700092593e); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 246; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000925948); };
            c.w64(11, t559);
            let Some(t560) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092594f); };
            c.w64(0, t560);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t561) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000925956); };
            c.w64(2, t561);
            let Some(t562) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700092595d); };
            c.cmp(t562, c.r64(3), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x700092596c, 0x7000925967);
        }
        241 => {
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092596c); };
            c.w64(8, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925970); };
            c.w64(15, t564);
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925974); };
            c.w64(9, t565);
            let Some(t566) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000925978); };
            c.w64(12, t566);
            blk = 243; continue;
        }
        242 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092597e); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 243; continue;
        }
        243 => {
            let Some(t567) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000925988); };
            c.w64(3, t567);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700092598f); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x7000925998); }
            let r568 = c.neg(c.r64(8), 64);
            c.w64(8, r568);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70009259a2); }
            blk = 251; continue;
        }
        245 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70009259a8); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70009259af); }
            blk = 251; continue;
        }
        246 => {
            let Some(t569) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009259b5); };
            c.w64(8, t569);
            let Some(t570) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009259bc); };
            c.w64(11, t570);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009259c3); };
            c.w64(0, t571);
            let Some(t572) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009259ca); };
            c.cmp(t572, c.r64(3), 64);
            if c.cc_a() { blk = 249; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70009259d9, 0x70009259d4);
        }
        248 => {
            blk = 250; continue;
        }
        249 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70009259db); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 250; continue;
        }
        250 => {
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70009259e5); };
            c.w64(8, t573);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70009259ec); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70009259f0); }
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009259f7); };
            c.w64(12, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009259fb); };
            c.w64(15, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009259ff); };
            c.w64(9, t576);
            blk = 251; continue;
        }
        251 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0xd));
            let Some(t577) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000925a0a); };
            c.w32(8, t577);
            let Some(t578) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925a0f); };
            c.w64(1, t578);
            let Some(t579) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925a13); };
            c.w64(5, t579);
            let r580 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r580);
            return RegfileExit::jump(c.r64(1));
        }
        252 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r581 = c.r64(1) & 0xfffffu64;
            c.w64(1, r581);
            c.logic(r581, 64);
            if !c.movss_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000925a28); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 254; continue;
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r582 = c.r64(1) & 0xfffffu64;
            c.w64(1, r582);
            c.logic(r582, 64);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000925a3f); }
            blk = 254; continue;
        }
        254 => {
            let Some(t583) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000925a45); };
            c.w64(8, t583);
            let Some(t584) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000925a4c); };
            c.w64(11, t584);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000925a53); }
            let Some(t585) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000925a58); };
            c.w64(3, t585);
            let Some(t586) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000925a5f); };
            c.cmp(t586, c.r64(0), 64);
            if c.cc_a() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000925a6e, 0x7000925a69);
        }
        256 => {
            blk = 258; continue;
        }
        257 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000925a70); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 258; continue;
        }
        258 => {
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925a7a); };
            c.w64(3, t587);
            c.w64(9, c.r64(3).wrapping_add(0xd));
            let Some(t588) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925a82); };
            c.w64(8, t588);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000925a89); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x7000925a8d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000925a92); }
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000925a97); };
            c.w64(12, t589);
            let Some(t590) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000925a9b); };
            c.w32(8, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925aa0); };
            c.w64(15, t591);
            let Some(t592) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925aa4); };
            c.w64(1, t592);
            let Some(t593) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925aa8); };
            c.w64(5, t593);
            let r594 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r594);
            return RegfileExit::jump(c.r64(1));
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(1), 0xbu64, 64) { c.w64(1, sr); }
            let r595 = c.add(c.r64(9), 0xdu64, 64);
            c.w64(9, r595);
            let r596 = c.r64(1) & 0xfffffu64;
            c.w64(1, r596);
            c.logic(r596, 64);
            let Some(t597) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000925ac1); };
            c.w32(8, t597);
            let Some(t598) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925ac6); };
            c.w32(0, t598);
            let r599 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r599);
            let Some(t600) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925acd); };
            c.w64(1, t600);
            let Some(t601) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925ad1); };
            c.w64(5, t601);
            let r602 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r602);
            return RegfileExit::jump(c.r64(1));
        }
        260 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r603 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r603);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000925ae1); }
            let Some(t604) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000925ae5); };
            c.w64(8, t604);
            let Some(t605) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925aec); };
            c.w32(8, t605);
            let r606 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r606);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000925af3); }
            let Some(t607) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000925af5); };
            c.w64(3, t607);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000925b09, 0x7000925b04);
        }
        261 => {
            return RegfileExit::fault(0x7000925b09);
        }
        262 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 263; continue;
        }
        263 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r608 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r608);
            c.w64(3, c.r64(9).wrapping_add(0xd));
            let Some(t609) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000925b17); };
            c.w32(2, t609);
            let Some(t610) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925b1c); };
            c.w32(1, t610);
            let r611 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r611);
            let Some(t612) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000925b24); };
            c.w64(6, t612);
            let Some(t613) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925b28); };
            c.w32(2, t613);
            let r614 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r614);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000925b2f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000925b33); }
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000925b34); };
            if !c.push64(t615) { return RegfileExit::fault(0x7000925b34); }
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000925b37); };
            if !c.push64(t616) { return RegfileExit::fault(0x7000925b37); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000925b3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000925b3c); }
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x7000925b40); };
            c.w64(3, t617);
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000925b54, 0x7000925b4f);
        }
        264 => {
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925b54); };
            c.w64(8, t618);
            let Some(t619) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925b58); };
            c.w64(8, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000925b5c); };
            c.w64(9, t620);
            let Some(t621) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000925b60); };
            c.w32(11, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925b65); };
            c.w64(15, t622);
            let Some(t623) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925b69); };
            c.w64(1, t623);
            let Some(t624) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925b6d); };
            c.w64(5, t624);
            c.w64(12, c.r64(8));
            let r625 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r625);
            return RegfileExit::jump(c.r64(1));
        }
        265 => {
            return RegfileExit::fault(0x7000925b7a);
        }
        266 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000925b7b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000925b7f); }
            let r626 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r626);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r627 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r627);
            c.w64(0, c.r64(9).wrapping_add(0xd));
            let Some(t628) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000925b90); };
            c.w32(3, t628);
            let Some(t629) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925b95); };
            c.w32(2, t629);
            let r630 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r630);
            let Some(t631) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000925b9d); };
            c.w64(1, t631);
            let Some(t632) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925ba1); };
            c.w32(3, t632);
            let r633 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r633);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925ba8); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000925bac); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000925bb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000925bb4); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            c.w32(0, 0xeu64);
            c.w64(11, c.r64(0));
            return c.call(0x7000804df4, 0x7000925bcb, 0x7000925bc6);
        }
        267 => {
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925bcb); };
            c.w64(8, t634);
            let Some(t635) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925bcf); };
            c.w64(8, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000925bd3); };
            c.w64(9, t636);
            let Some(t637) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000925bd7); };
            c.w32(11, t637);
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925bdc); };
            c.w64(15, t638);
            let Some(t639) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925be0); };
            c.w64(1, t639);
            let Some(t640) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925be4); };
            c.w64(5, t640);
            c.w64(12, c.r64(8));
            let r641 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r641);
            return RegfileExit::jump(c.r64(1));
        }
        268 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000925bf1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000925bf5); }
            let r642 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r642);
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r643 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r643);
            let Some(t644) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000925c01); };
            c.w32(3, t644);
            let Some(t645) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925c06); };
            c.w32(2, t645);
            let r646 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r646);
            let Some(t647) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925c0e); };
            c.w32(3, t647);
            let r648 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r648);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000925c15); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000925c19); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000925c1a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000925c1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000925c22); }
            let Some(t649) = c.ld64(c.r64(13).wrapping_add(0x26d0)) else { return RegfileExit::fault(0x7000925c26); };
            c.w64(3, t649);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x7000925c3a, 0x7000925c35);
        }
        269 => {
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925c3a); };
            c.w64(8, t650);
            let Some(t651) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925c3e); };
            c.w64(8, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925c42); };
            c.w64(11, t652);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t653) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000925c4a); };
            c.w32(11, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925c4f); };
            c.w64(15, t654);
            let Some(t655) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925c53); };
            c.w64(1, t655);
            let Some(t656) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925c57); };
            c.w64(5, t656);
            c.w64(12, c.r64(8));
            let r657 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r657);
            return RegfileExit::jump(c.r64(1));
        }
        270 => {
            return RegfileExit::jump(0x7000925c68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000925c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000925c74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000925c78); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000925c7e); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925c83); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000925c8b); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925c91); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925c98); };
            c.w64(7, t7);
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t8) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000925ca0); };
            c.w64(3, t8);
            let r9 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000925ca7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000925cab); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000925caf); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925cbf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000925cc3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000925cc4); }
            c.w64(6, c.r64(3));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2728)) else { return RegfileExit::fault(0x7000925cc8); };
            c.w64(2, t10);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x7000925cdf, 0x7000925cda);
        }
        3 => {
            blk = 7; continue;
        }
        4 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925ce1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000925ce5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000925ce6); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2730)) else { return RegfileExit::fault(0x7000925ce7); };
            c.w64(2, t11);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x7000925cfe, 0x7000925cf9);
        }
        5 => {
            blk = 7; continue;
        }
        6 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000925d00); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000925d04); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000925d05); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2738)) else { return RegfileExit::fault(0x7000925d06); };
            c.w64(2, t12);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x7000925d1d, 0x7000925d18);
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925d1d); };
            c.w64(2, t13);
            let Some(t14) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000925d21); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000925d25); };
            c.w64(7, t15);
            c.w64(9, c.r64(7).wrapping_add(0x6));
            let Some(t16) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000925d2d); };
            c.w32(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925d32); };
            c.w64(15, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925d36); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925d3a); };
            c.w64(5, t19);
            c.w64(12, c.r64(2));
            let r20 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r20);
            return RegfileExit::jump(c.r64(1));
        }
        8 => {
            return RegfileExit::jump(0x7000925d48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000925d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000925d54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000925d58); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000925d5e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925d62); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000925d66); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925d6d); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925d74); };
            c.w32(0, t7);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000925d78); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000925d7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000925d80); }
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000925d84); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000925d92); };
            c.w64(0, t9);
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000925d99); };
            c.w64(3, (t10 as u32 as i32 as i64 as u64));
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000925d9e); };
            c.w64(3, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000925da2); };
            c.w64(11, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000925da9); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925dbb); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            blk = 7; continue;
        }
        6 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925dc3); };
            c.w64(2, t16);
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000925dca); };
            c.logic(t17 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000925dd7); };
            c.w64(1, (t18 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(6, c.r32(2));
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000925de4); };
            c.w32(7, t19);
            let r20 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r20);
            let r21 = c.r32(6) | 0x2u64;
            c.w32(6, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, c.r32(7));
            let r22 = c.r32(1) & 0x3u64;
            c.w32(1, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925e00); };
            c.w32(1, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x7000925e03); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t25) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925e10); };
            c.w32(1, t25);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r26 = c.sub(c.r64(1), 0x2u64, 64);
            c.w64(1, r26);
            blk = 12; continue;
        }
        12 => {
            c.w64(6, c.r64(1).wrapping_mul(4));
            c.w32(15, c.r32(2));
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000925e28); };
            c.w32(11, t27);
            let r28 = c.r32(15) | 0x2u64;
            c.w32(15, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let r29 = c.sub(c.r64(1), 0x2u64, 64);
            c.w64(1, r29);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ge() { blk = 12; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 78; continue;
        }
        15 => {
            let Some(t30) = c.ld32(c.r64(7).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000925e45); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            c.w64(2, c.r64(11));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000925e50); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000925e54); };
            c.w64(11, t33);
            blk = 17; continue;
        }
        16 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(c.r64(1)).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000925e5d); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, c.r64(3));
            c.w64(6, c.r64(2));
            blk = 19; continue;
        }
        19 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(7, c.r64(6));
            let r36 = c.r64(7) & 0xfu64;
            c.w64(7, r36);
            c.logic(r36, 64);
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(7), 0x5u64, 64);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r64(7), 0xdu64, 64);
            if c.cc_e() { blk = 80; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 171; continue;
        }
        23 => {
            c.w64(11, c.r64(1));
            blk = 80; continue;
        }
        24 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            let r37 = c.r64(11) & 0x7ffu64;
            c.w64(11, r37);
            c.logic(r37, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000925ecc); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000925ed7); };
            c.w64(1, t40);
            blk = 28; continue;
        }
        28 => {
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925ede); };
            c.w32(11, t41);
            let r42 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r42);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925ef9); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925efd); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000925f0a); }
            blk = 74; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925f1a); };
            c.w32(0, t45);
            let r46 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r46);
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        34 => {
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925f2a); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t49) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925f3c); };
            c.w32(2, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000925f40); };
            c.cmp(t50, c.r32(2), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000925f4d); }
            blk = 74; continue;
        }
        37 => {
            c.w64(11, c.r64(6));
            let r51 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r51);
            let r52 = c.r32(11) & 0xfu64;
            c.w32(11, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 46; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 323; continue;
        }
        43 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 45; continue;
        }
        45 => {
            blk = 323; continue;
        }
        46 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 323; continue;
        }
        51 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 323; continue;
        }
        54 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r53 = c.r64(6) & 0xfffffu64;
            c.w64(6, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000925fec); };
            c.w32(11, (t54 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        55 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r55 = c.r64(6) & 0xfffffu64;
            c.w64(6, r55);
            c.logic(r55, 64);
            let Some(t56) = c.ld16(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926009); };
            c.w32(11, (t56 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        56 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r57 = c.r64(6) & 0xfffffu64;
            c.w64(6, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926026); };
            c.w32(11, t58);
            c.w64(0, c.r64(11));
            let r59 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r59);
            if c.cc_o() { blk = 324; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 58; continue;
        }
        58 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        59 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r60 = c.r64(6) & 0xfffffu64;
            c.w64(6, r60);
            c.logic(r60, 64);
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092604d); };
            c.w32(11, t61);
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 329; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 61; continue;
        }
        61 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        62 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r62 = c.r64(6) & 0xfffffu64;
            c.w64(6, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld64(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926076); };
            c.w64(11, t63);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000926081); };
            c.w64(0, t64);
            let Some(t65) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000926088); };
            c.w64(3, t65);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092608f); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000926096); };
            c.cmp(t67, c.r64(2), 64);
            if c.cc_be() { blk = 334; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700092609f); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70009260a6); };
            c.w64(3, t68);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009260ad); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_g() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70009260b6); }
            let r69 = c.neg(c.r64(11), 64);
            c.w64(11, r69);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70009260c0); }
            blk = 71; continue;
        }
        67 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70009260c6); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70009260cd); }
            blk = 71; continue;
        }
        68 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009260d3); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009260da); };
            c.w64(0, t71);
            c.w64(2, c.r64(0).wrapping_add(0x8));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009260e1); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009260e8); };
            c.cmp(t73, c.r64(2), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70009260f1); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70009260f8); };
            c.w64(0, t74);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70009260ff); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000926103); }
            c.w64(0, c.r64(11));
            blk = 71; continue;
        }
        71 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r75 = c.r64(6) & 0xfffffu64;
            c.w64(6, r75);
            c.logic(r75, 64);
            if !c.movss_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000926121); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 74; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r76 = c.r64(6) & 0xfffffu64;
            c.w64(6, r76);
            c.logic(r76, 64);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000926138); }
            blk = 74; continue;
        }
        74 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092613e); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000926145); };
            c.w64(0, t78);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092614c); };
            c.w64(2, t79);
            let Some(t80) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000926153); };
            c.cmp(t80, c.r64(3), 64);
            if c.cc_be() { blk = 338; continue; }
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092615c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000926163); };
            c.w64(0, t81);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700092616a); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092616e); }
            blk = 80; continue;
        }
        77 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r82 = c.r64(6) & 0xfffffu64;
            c.w64(6, r82);
            c.logic(r82, 64);
            let Some(t83) = c.ld32(c.r64(1).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926181); };
            c.w32(0, t83);
            let r84 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r84);
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        78 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r85 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r85);
            let Some(t86) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926194); };
            c.w64(2, t86);
            let Some(t87) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926198); };
            c.w32(1, t87);
            let Some(t88) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092619d); };
            c.w32(6, t88);
            let r89 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r89);
            let Some(t90) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009261a3); };
            c.w64(7, t90);
            let Some(t91) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009261a7); };
            c.w64(2, (t91 as u32 as i32 as i64 as u64));
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009261ac); };
            c.w32(1, t92);
            let r93 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r93);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009261b6); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009261ba); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70009261bb); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70009261bc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009261bd); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2050)) else { return RegfileExit::fault(0x70009261be); };
            c.w64(3, t94);
            c.w32(0, 0x4u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70009261d2, 0x70009261cd);
        }
        79 => {
            c.w64(11, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009261d5); };
            c.w64(15, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009261d9); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009261dd); };
            c.w64(8, t97);
            blk = 80; continue;
        }
        80 => {
            let Some(t98) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009261e1); };
            c.w64(8, t98);
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0xd));
            let Some(t99) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009261ec); };
            c.w32(0, t99);
            let Some(t100) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70009261f1); };
            c.w64(1, t100);
            let Some(t101) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009261f5); };
            c.w64(5, t101);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r102 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r102);
            return RegfileExit::jump(c.r64(1));
        }
        81 => {
            blk = 78; continue;
        }
        82 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let r103 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r103);
            let Some(t104) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092620e); };
            c.w64(2, t104);
            let Some(t105) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926212); };
            c.w32(1, t105);
            let Some(t106) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926217); };
            c.w32(6, t106);
            let r107 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r107);
            let Some(t108) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092621d); };
            c.w64(7, t108);
            let Some(t109) = c.ld32(c.r64(2).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926221); };
            c.w64(2, (t109 as u32 as i32 as i64 as u64));
            let Some(t110) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926226); };
            c.w32(1, t110);
            let r111 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r111);
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000926230); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x7000832fd4, 0x700092624e, 0x7000926249);
        }
        83 => {
            c.w64(11, c.r64(0));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926251); };
            c.w64(15, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926255); };
            c.w64(9, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926259); };
            c.w64(8, t114);
            blk = 80; continue;
        }
        84 => {
            let Some(t115) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092625f); };
            c.w32(1, t115);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000926262); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x7000926266); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092626a); }
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700092626e); };
            c.cmp(t116, c.r32(1), 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092627b); };
            c.w32(1, t117);
            let r118 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r118);
            c.w32(6, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t119) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926287); };
            c.cmp(t119, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t120) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926290); };
            c.w32(1, t120);
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000926299); }
            c.logic(c.r8(7) & 0x30u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 88; continue;
        }
        88 => {
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009262b1); };
            c.w32(7, t121);
            let r122 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r122);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70009262bd); };
            c.w64(7, t123);
            blk = 91; continue;
        }
        91 => {
            let Some(t124) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009262c4); };
            c.w32(0, t124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009262c7); };
            c.w32(1, t125);
            let Some(t126) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009262cc); };
            c.w32(2, t126);
            let r127 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r127);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t128) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009262d7); };
            c.w32(1, t128);
            let r129 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r129);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r130 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r130);
            let Some(t131) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009262e5); };
            c.w32(15, t131);
            let r132 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r132);
            let r133 = c.r64(15) & c.r64(0);
            c.w64(15, r133);
            c.logic(r133, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70009262f0); }
            blk = 92; continue;
        }
        92 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(3).wrapping_mul(4));
            let Some(t134) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926301); };
            c.w32(2, t134);
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000926306); };
            c.cmp(t135, c.r32(2), 32);
            if c.cc_e() { blk = 150; continue; }
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let r136 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r136);
            c.w64(11, c.r64(6).wrapping_add(c.r64(15)));
            c.w64(3, c.r64(0));
            let r137 = c.r64(3) & c.r64(11);
            c.w64(3, r137);
            c.logic(r137, 64);
            c.w64(15, c.r64(3));
            blk = 92; continue;
        }
        95 => {
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092632a); };
            c.w32(0, t138);
            let Some(t139) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092632d); };
            c.w32(2, t139);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r140 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r140);
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_b() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            return RegfileExit::fault(0x7000926341);
        }
        97 => {
            let Some(t141) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926342); };
            c.w32(0, t141);
            let Some(t142) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926347); };
            c.w64(3, t142);
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092634b); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000926353); };
            c.w64(1, t145);
            let Some(t146) = c.ld32(c.r64(1).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926357); };
            c.w32(0, t146);
            let r147 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r147);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 149; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t148) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926367); };
            c.w32(2, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700092636b); };
            c.cmp(t149, c.r32(2), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t150) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926378); };
            c.w32(2, t150);
            let r151 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r151);
            let Some(t152) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092637f); };
            c.w32(11, t152);
            let r153 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r153);
            let Some(t154) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926386); };
            c.logic(t154 & 0x2u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000926391); };
            c.cmp(t155, c.r32(11), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092639a); };
            c.w64(11, t156);
            blk = 149; continue;
        }
        102 => {
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70009263a6); };
            c.w64(11, t157);
            let Some(t158) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70009263ad); };
            c.cmp(t158, 0x20u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t159) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70009263b7); };
            c.w32(11, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009263bb); };
            c.cmp(t160, c.r32(11), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009263c4); };
            c.w64(3, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009263c8); };
            c.w64(0, t162);
            blk = 78; continue;
        }
        105 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009263d1); };
            c.w64(1, t163);
            let Some(t164) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009263d5); };
            c.w32(11, t164);
            let r165 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r165);
            let Some(t166) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009263dc); };
            c.w32(0, t166);
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009263e0); };
            c.cmp(t167, c.r32(0), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t168) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009263e9); };
            c.w32(11, t168);
            let r169 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r169);
            let Some(t170) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009263f0); };
            c.w32(0, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70009263f4); };
            c.cmp(t171, c.r32(0), 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 107; continue;
        }
        107 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t172) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926407); };
            c.w32(0, t172);
            let r173 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r173);
            let Some(t174) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092640e); };
            c.w32(3, t174);
            c.w32(6, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 114; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w32(11, c.r16(3));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700092642a); };
            c.cmp(t175, c.r32(0), 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(11, c.r64(1));
            blk = 114; continue;
        }
        112 => {
            c.w64(3, c.r64(1));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092643b); };
            c.w64(0, t176);
            blk = 78; continue;
        }
        113 => {
            let Some(t177) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926444); };
            c.w32(11, t177);
            let r178 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r178);
            blk = 114; continue;
        }
        114 => {
            let Some(t179) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092644b); };
            c.w32(11, t179);
            let r180 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r180);
            let Some(t181) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926452); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926459); };
            c.w32(0, t183);
            let r184 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r184);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r185 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r185);
            let Some(t186) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092646e); };
            c.w32(6, t186);
            let r187 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r187);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926475); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000926479); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000926481, 0x700092647c);
        }
        116 => {
            c.w64(11, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926484); };
            c.w64(9, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926488); };
            c.w64(8, t189);
            blk = 149; continue;
        }
        117 => {
            c.w64(3, c.r64(1));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926494); };
            c.w64(0, t190);
            blk = 78; continue;
        }
        118 => {
            c.w64(3, c.r64(1));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009264a0); };
            c.w64(0, t191);
            blk = 78; continue;
        }
        119 => {
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r192 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r192);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009264b2); }
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009264b6); };
            if !c.push64(t193) { return RegfileExit::fault(0x70009264b6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009264b9); }
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009264ba); };
            if !c.push64(t194) { return RegfileExit::fault(0x70009264ba); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70009264c5); };
            c.w64(3, t195);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70009264d4, 0x70009264cf);
        }
        120 => {
            c.w64(11, c.r64(0));
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009264d7); };
            c.w64(9, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009264db); };
            c.w64(8, t197);
            blk = 149; continue;
        }
        121 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r198 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r198);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009264ed); }
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009264f1); };
            if !c.push64(t199) { return RegfileExit::fault(0x70009264f1); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(6, c.r64(3));
            return c.call(0x70007f2274, 0x7000926501, 0x70009264fc);
        }
        122 => {
            c.w64(11, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926504); };
            c.w64(9, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926508); };
            c.w64(8, t201);
            blk = 149; continue;
        }
        123 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926511); };
            c.w64(2, t202);
            let Some(t203) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926515); };
            c.w32(0, t203);
            let r204 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r204);
            let Some(t205) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092651b); };
            c.w32(3, t205);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 146; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 134; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(3, c.r64(2));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092653f); };
            c.w64(0, t206);
            blk = 78; continue;
        }
        127 => {
            let Some(t207) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926548); };
            c.w32(11, t207);
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700092654c); };
            c.cmp(t208, c.r32(11), 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(3, c.r64(2));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926558); };
            c.w64(0, t209);
            blk = 78; continue;
        }
        129 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926561); };
            c.w32(11, t210);
            let r211 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r211);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(3, c.r64(2));
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926571); };
            c.w64(0, t212);
            blk = 78; continue;
        }
        131 => {
            let Some(t213) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092657a); };
            c.w32(0, t213);
            let r214 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r214);
            let Some(t215) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926581); };
            c.cmp(t215, 0x40u64, 16);
            if c.cc_b() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(3, c.r64(2));
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092658b); };
            c.w64(0, t216);
            blk = 78; continue;
        }
        133 => {
            let Some(t217) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926594); };
            c.w32(11, t217);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 149; continue;
        }
        134 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009265a4); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70009265a8); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(3, c.r64(2));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009265b4); };
            c.w64(0, t220);
            blk = 78; continue;
        }
        136 => {
            let Some(t221) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009265bd); };
            c.w32(11, t221);
            c.w64(0, c.r64(11));
            let r222 = c.r32(0) & 0xc0u64;
            c.w32(0, r222);
            c.logic(r222, 32);
            c.cmp(c.r32(0), 0xc0u64, 32);
            if c.cc_e() { blk = 139; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t223) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009265d1); };
            c.w32(0, t223);
            let r224 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r224);
            let Some(t225) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70009265d7); };
            c.w32(0, t225);
            let r226 = c.r32(0) & 0x1fu64;
            c.w32(0, r226);
            c.logic(r226, 32);
            let r227 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r227);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_be() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w64(3, c.r64(2));
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009265e8); };
            c.w64(0, t228);
            blk = 78; continue;
        }
        139 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 145; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70009265f7); };
            c.w32(0, t229);
            let r230 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009265fd); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t232) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926606); };
            c.w32(11, t232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700092660a); };
            c.cmp(t233, c.r32(11), 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t234) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926613); };
            c.w32(0, t234);
            let r235 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r235);
            blk = 143; continue;
        }
        143 => {
            c.w64(11, c.r64(0));
            blk = 149; continue;
        }
        144 => {
            c.w64(3, c.r64(2));
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926621); };
            c.w64(0, t236);
            blk = 78; continue;
        }
        145 => {
            c.w64(3, c.r64(2));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092662d); };
            c.w64(0, t237);
            blk = 78; continue;
        }
        146 => {
            let Some(t238) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926636); };
            c.w32(11, t238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700092663a); };
            c.cmp(t239, c.r32(11), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(3, c.r64(2));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926646); };
            c.w64(0, t240);
            blk = 78; continue;
        }
        148 => {
            let Some(t241) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092664f); };
            c.w32(0, t241);
            let r242 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r242);
            c.w64(11, c.r64(0));
            blk = 149; continue;
        }
        149 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926658); };
            c.w64(15, t243);
            blk = 80; continue;
        }
        150 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926661); };
            c.w64(15, t244);
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000926665); };
            c.w64(11, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092666c); };
            c.w64(3, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926670); };
            c.w64(0, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000926674); };
            c.w64(2, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000926678); };
            c.w64(1, t249);
            blk = 159; continue;
        }
        151 => {
            let Some(t250) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926681); };
            c.w32(6, t250);
            let r251 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r251);
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t252) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926690); };
            c.w64(7, t252);
            let r253 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r253);
            c.logic(r253, 64);
            let Some(t254) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926698); };
            c.w32(7, t254);
            let r255 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r255);
            let Some(t256) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092669e); };
            c.w32(7, t256);
            let r257 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r257);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t258) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009266a8); };
            c.w32(12, t258);
            let r259 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r259);
            let Some(t260) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009266af); };
            c.cmp(t260, 0xbeu64, 16);
            if c.cc_ne() { blk = 78; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t261) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009266bd); };
            c.w32(12, t261);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            return RegfileExit::fault(0x70009266d0);
        }
        156 => {
            let Some(t262) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009266d1); };
            c.w32(12, t262);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t263) = c.ld32(c.r64(6).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70009266d8); };
            c.w32(6, t263);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xa9u64, 64);
            if c.cc_a() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            return RegfileExit::fault(0x70009266ea);
        }
        158 => {
            let Some(t264) = c.ld32(c.r64(7).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x70009266eb); };
            c.cmp(t264, c.r32(6), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t265) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009266f7); };
            c.w32(6, t265);
            let r266 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r266);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000926703); };
            c.cmp(t267, c.r32(6), 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 162; continue;
        }
        162 => {
            let r268 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r268);
            c.logic(r268, 64);
            blk = 164; continue;
        }
        163 => {
            c.w64(6, c.r64(3));
            blk = 164; continue;
        }
        164 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 19; continue;
        }
        165 => {
            c.w64(11, c.r64(6));
            blk = 80; continue;
        }
        166 => {
            c.w64(11, c.r64(9));
            c.w64(1, c.r64(11).wrapping_sub(0x1));
            let r269 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r269);
            let Some(t270) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092673d); };
            c.w64(6, t270);
            let Some(t271) = c.ld32(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926741); };
            c.w32(7, t271);
            let Some(t272) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926746); };
            c.w32(9, t272);
            let r273 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r273);
            let Some(t274) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092674d); };
            c.w64(12, t274);
            let Some(t275) = c.ld32(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926751); };
            c.w64(6, (t275 as u32 as i32 as i64 as u64));
            let Some(t276) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926756); };
            c.w32(7, t276);
            let r277 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r277);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000926761); }
            c.w64(8, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(7));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(6));
            c.w64(6, c.r64(12));
            let Some(t278) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000926777); };
            c.logic(t278 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let r279 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r279);
            blk = 169; continue;
        }
        168 => {
            let Some(t280) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000926787); };
            c.w32(8, t280);
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700092678b); };
            c.w64(8, t281);
            blk = 169; continue;
        }
        169 => {
            return c.call(c.r64(8), 0x7000926796, 0x7000926793);
        }
        170 => {
            c.w64(11, c.r64(0));
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926799); };
            c.w64(15, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092679d); };
            c.w64(9, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009267a1); };
            c.w64(8, t284);
            blk = 80; continue;
        }
        171 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009267aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70009267ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70009267b2); }
            c.cmp(c.r64(7), 0x2u64, 64);
            if c.cc_e() { blk = 261; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r64(7), 0x6u64, 64);
            if c.cc_e() { blk = 259; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 254; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r64(7), 0x8u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r64(7), 0x9u64, 64);
            if c.cc_e() { blk = 243; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r64(7), 0x3u64, 64);
            if c.cc_e() { blk = 190; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 257; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r64(7), 0xcu64, 64);
            if c.cc_e() { blk = 188; continue; }
            blk = 179; continue;
        }
        179 => {
            c.cmp(c.r64(7), 0xeu64, 64);
            if c.cc_e() { blk = 182; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r285 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r285);
            let Some(t286) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926821); };
            c.w64(6, t286);
            let Some(t287) = c.ld32(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926825); };
            c.w32(7, t287);
            let Some(t288) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092682a); };
            c.w32(9, t288);
            let r289 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r289);
            let Some(t290) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926831); };
            c.w64(12, t290);
            let Some(t291) = c.ld32(c.r64(6).wrapping_add(c.r64(11)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926835); };
            c.w64(6, (t291 as u32 as i32 as i64 as u64));
            let Some(t292) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092683a); };
            c.w32(7, t292);
            let r293 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r293);
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000926845); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000926849); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700092684a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700092684b); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700092684c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092684d); }
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700092684e); };
            c.w64(3, t294);
            c.w32(0, 0x5u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x7000926862, 0x700092685d);
        }
        181 => {
            c.w64(11, c.r64(0));
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926865); };
            c.w64(15, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926869); };
            c.w64(9, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092686d); };
            c.w64(8, t297);
            blk = 80; continue;
        }
        182 => {
            let Some(t298) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926876); };
            c.w32(11, t298);
            let r299 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r299);
            let Some(t300) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092687d); };
            c.w32(11, t300);
            let r301 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r301);
            let Some(t302) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926884); };
            c.w32(0, t302);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r303 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r303);
            c.logic(r303, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            return RegfileExit::fault(0x700092689e);
        }
        184 => {
            let Some(t304) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092689f); };
            c.w32(11, t304);
            let r305 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r305);
            let Some(t306) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009268a7); };
            c.w32(0, t306);
            let r307 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r307);
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009268ae); };
            c.cmp(t308, c.r32(0), 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(11, c.r64(0));
            blk = 80; continue;
        }
        186 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r309 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r309);
            let Some(t310) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009268c8); };
            c.w64(3, t310);
            let Some(t311) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009268cc); };
            c.w32(2, t311);
            let Some(t312) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009268d1); };
            c.w32(3, t312);
            let r313 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r313);
            let Some(t314) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009268d7); };
            c.w64(1, t314);
            let Some(t315) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009268db); };
            c.w32(3, t315);
            let r316 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r316);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009268e2); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009268eb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70009268ec); }
            let Some(t317) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70009268ed); };
            c.w64(3, t317);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000926901, 0x70009268fc);
        }
        187 => {
            c.w64(11, c.r64(0));
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926904); };
            c.w64(15, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926908); };
            c.w64(9, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092690c); };
            c.w64(8, t320);
            blk = 80; continue;
        }
        188 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r321 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r321);
            let Some(t322) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092691e); };
            c.w64(2, t322);
            let Some(t323) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926922); };
            c.w32(6, t323);
            let Some(t324) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926927); };
            c.w32(2, t324);
            let r325 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r325);
            let Some(t326) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092692d); };
            c.w64(7, t326);
            let Some(t327) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926931); };
            c.w32(2, t327);
            let r328 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r328);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926938); }
            c.w64(6, c.r64(7));
            c.w64(0, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w32(2, 0x2u64);
            return c.call(0x70008bc174, 0x7000926952, 0x700092694d);
        }
        189 => {
            c.w64(11, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926955); };
            c.w64(15, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926959); };
            c.w64(9, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092695d); };
            c.w64(8, t331);
            blk = 80; continue;
        }
        190 => {
            let Some(t332) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926966); };
            c.w32(2, t332);
            let r333 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r333);
            let Some(t334) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092696c); };
            c.w32(6, t334);
            let Some(t335) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700092696f); };
            c.cmp(t335, c.r32(2), 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t336) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092697c); };
            c.w64(7, t336);
            let Some(t337) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926980); };
            c.w64(12, t337);
            let Some(t338) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926984); };
            c.w32(0, t338);
            let Some(t339) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926989); };
            c.w32(7, t339);
            let r340 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r340);
            let Some(t341) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092698f); };
            c.w32(0, t341);
            let r342 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r342);
            c.logic(c.r8(6) & 0x2u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t343) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009269a0); };
            c.w32(6, t343);
            let r344 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70009269a6); };
            c.cmp(t345, c.r32(6), 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t346) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009269b3); };
            c.w32(6, t346);
            let r347 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r347);
            let Some(t348) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009269b9); };
            c.w32(7, t348);
            let r349 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r349);
            let Some(t350) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009269bf); };
            c.logic(t350 & 0x2u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t351) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70009269c9); };
            c.cmp(t351, c.r32(7), 32);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(2, c.r64(11));
            blk = 242; continue;
        }
        196 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70009269da); };
            c.w64(11, t352);
            let Some(t353) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70009269e1); };
            c.cmp(t353, 0x20u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t354) = c.ld32(c.r64(6).wrapping_add(0x33)) else { return RegfileExit::fault(0x70009269eb); };
            c.w32(11, t354);
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70009269ef); };
            c.cmp(t355, c.r32(11), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009269f8); };
            c.w64(0, t356);
            blk = 78; continue;
        }
        199 => {
            let Some(t357) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926a01); };
            c.w32(11, t357);
            let r358 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r358);
            let Some(t359) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926a08); };
            c.w32(0, t359);
            let Some(t360) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000926a0c); };
            c.cmp(t360, c.r32(0), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t361) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000926a15); };
            c.w32(11, t361);
            let r362 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r362);
            let Some(t363) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926a1c); };
            c.w32(0, t363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000926a20); };
            c.cmp(t364, c.r32(0), 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 201; continue;
        }
        201 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t365) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926a33); };
            c.w32(0, t365);
            let r366 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r366);
            let Some(t367) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926a3a); };
            c.w32(7, t367);
            c.w32(12, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 207; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w32(11, c.r16(7));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000926a58); };
            c.cmp(t368, c.r32(0), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926a61); };
            c.w64(0, t369);
            blk = 78; continue;
        }
        206 => {
            c.w64(1, c.r64(2));
            blk = 208; continue;
        }
        207 => {
            c.w64(1, c.r64(11));
            blk = 208; continue;
        }
        208 => {
            let Some(t370) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926a72); };
            c.w32(11, t370);
            let r371 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r371);
            let Some(t372) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926a79); };
            c.w32(11, t372);
            let r373 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r373);
            let Some(t374) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926a80); };
            c.w32(0, t374);
            let r375 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r375);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r376 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r376);
            let Some(t377) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000926a95); };
            c.w32(11, t377);
            let r378 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r378);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926a9c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926aa0); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(11));
            return c.call(0x70007f6fb4, 0x7000926aae, 0x7000926aa9);
        }
        210 => {
            c.w64(2, c.r64(0));
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926ab1); };
            c.w64(15, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926ab5); };
            c.w64(9, t380);
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926ab9); };
            c.w64(8, t381);
            blk = 242; continue;
        }
        211 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926ac2); };
            c.w64(0, t382);
            blk = 78; continue;
        }
        212 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926acb); };
            c.w64(0, t383);
            blk = 78; continue;
        }
        213 => {
            c.w64(11, c.r64(9));
            c.w64(2, c.r64(11).wrapping_sub(0x1));
            let r384 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r384);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000926add); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000926ae1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000926ae2); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926ae3); }
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000926ae4); };
            c.w64(3, t385);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x7000926af8, 0x7000926af3);
        }
        214 => {
            c.w64(2, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926afb); };
            c.w64(15, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926aff); };
            c.w64(9, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926b03); };
            c.w64(8, t388);
            blk = 242; continue;
        }
        215 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r389 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r389);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926b15); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926b19); }
            c.w64(7, c.r64(6));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            return c.call(0x70007f2274, 0x7000926b27, 0x7000926b22);
        }
        216 => {
            c.w64(2, c.r64(0));
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926b2a); };
            c.w64(15, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926b2e); };
            c.w64(9, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926b32); };
            c.w64(8, t392);
            blk = 242; continue;
        }
        217 => {
            let Some(t393) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926b3b); };
            c.w32(11, t393);
            let r394 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r394);
            let Some(t395) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926b42); };
            c.w32(0, t395);
            c.cmp(c.r16(0), 0x834u64, 16);
            if c.cc_e() { blk = 240; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w32(7, c.r64(0).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 228; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r16(0), 0x411u64, 16);
            if c.cc_e() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926b62); };
            c.w64(0, t396);
            blk = 78; continue;
        }
        221 => {
            let Some(t397) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926b6b); };
            c.w32(11, t397);
            let Some(t398) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000926b6f); };
            c.cmp(t398, c.r32(11), 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926b78); };
            c.w64(0, t399);
            blk = 78; continue;
        }
        223 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926b86); };
            c.w64(0, t400);
            blk = 78; continue;
        }
        225 => {
            let Some(t401) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926b8f); };
            c.cmp(t401, 0x40u64, 16);
            if c.cc_b() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926b96); };
            c.w64(0, t402);
            blk = 78; continue;
        }
        227 => {
            let Some(t403) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926b9f); };
            c.w32(11, t403);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(2, c.r64(0));
            blk = 242; continue;
        }
        228 => {
            let Some(t404) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926baf); };
            c.w32(0, t404);
            let Some(t405) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000926bb2); };
            c.cmp(t405, c.r32(0), 32);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926bbb); };
            c.w64(0, t406);
            blk = 78; continue;
        }
        230 => {
            let Some(t407) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926bc4); };
            c.w32(11, t407);
            c.w64(0, c.r64(11));
            let r408 = c.r32(0) & 0xc0u64;
            c.w32(0, r408);
            c.logic(r408, 32);
            c.cmp(c.r32(0), 0xc0u64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t409) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000926bd8); };
            c.w32(0, t409);
            let r410 = c.r32(0) & 0x1fu64;
            c.w32(0, r410);
            c.logic(r410, 32);
            let r411 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r411);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_be() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926be6); };
            c.w64(0, t412);
            blk = 78; continue;
        }
        233 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t413) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000926bf5); };
            c.w32(0, t413);
            let r414 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r414);
            let Some(t415) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000926bfb); };
            c.cmp(t415, c.r32(0), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t416) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926c04); };
            c.w32(11, t416);
            let Some(t417) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000926c08); };
            c.cmp(t417, c.r32(11), 32);
            if c.cc_ne() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t418) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926c11); };
            c.w32(0, t418);
            let r419 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r419);
            blk = 237; continue;
        }
        237 => {
            c.w64(2, c.r64(0));
            blk = 242; continue;
        }
        238 => {
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926c1c); };
            c.w64(0, t420);
            blk = 78; continue;
        }
        239 => {
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926c25); };
            c.w64(0, t421);
            blk = 78; continue;
        }
        240 => {
            let Some(t422) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926c2e); };
            c.w32(11, t422);
            let Some(t423) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000926c32); };
            c.cmp(t423, c.r32(11), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926c3b); };
            c.w64(0, t424);
            blk = 78; continue;
        }
        242 => {
            c.w64(11, c.r64(2));
            blk = 80; continue;
        }
        243 => {
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000926c4c); };
            c.w64(11, t425);
            let Some(t426) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000926c53); };
            c.cmp(t426, 0x20u64, 8);
            if c.cc_e() { blk = 257; continue; }
            blk = 244; continue;
        }
        244 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t427) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926c63); };
            c.w32(11, t427);
            let r428 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r428);
            blk = 247; continue;
        }
        246 => {
            let Some(t429) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000926c6c); };
            c.w32(11, t429);
            let r430 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r430);
            blk = 247; continue;
        }
        247 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            return RegfileExit::fault(0x7000926c79);
        }
        249 => {
            let r431 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r431);
            c.logic(r431, 64);
            let Some(t432) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926c7e); };
            c.w32(0, t432);
            let r433 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r433);
            let Some(t434) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926c84); };
            c.w32(2, t434);
            let r435 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r435);
            let Some(t436) = c.ld64(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926c8a); };
            c.w64(0, t436);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t437) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926c94); };
            c.w32(1, t437);
            let r438 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r438);
            let Some(t439) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926c9a); };
            c.w32(1, t439);
            let r440 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r440);
            blk = 252; continue;
        }
        251 => {
            c.w64(1, c.r64(3));
            blk = 252; continue;
        }
        252 => {
            c.w64(6, c.r64(9));
            c.w64(7, c.r64(6).wrapping_sub(0x1));
            let r441 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r441);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000926cae); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926cb2); }
            c.w64(6, c.r64(11));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007fe8d4, 0x7000926cc6, 0x7000926cc1);
        }
        253 => {
            c.w64(11, c.r64(0));
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926cc9); };
            c.w64(15, t442);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926ccd); };
            c.w64(9, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926cd1); };
            c.w64(8, t444);
            blk = 80; continue;
        }
        254 => {
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000926cda); };
            c.w64(11, t445);
            let Some(t446) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000926ce1); };
            c.cmp(t446, 0x20u64, 8);
            if c.cc_e() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r447 = c.r64(6) & 0x3ffu64;
            c.w64(6, r447);
            c.logic(r447, 64);
            let r448 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r448);
            let r449 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r449);
            let Some(t450) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926cff); };
            c.w32(6, t450);
            let r451 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r451);
            let Some(t452) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000926d05); };
            c.w32(6, t452);
            let r453 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r453);
            let Some(t454) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926d0b); };
            c.w64(7, t454);
            let Some(t455) = c.ld32(c.r64(6).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000926d0f); };
            c.w32(2, t455);
            let r456 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r456);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926d16); }
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(10));
            c.w64(6, c.r64(7));
            return c.call(0x70007fea14, 0x7000926d2b, 0x7000926d26);
        }
        256 => {
            c.w64(11, c.r64(0));
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926d2e); };
            c.w64(15, t457);
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926d32); };
            c.w64(9, t458);
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926d36); };
            c.w64(8, t459);
            blk = 80; continue;
        }
        257 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r460 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r460);
            let Some(t461) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926d48); };
            c.w64(2, t461);
            let Some(t462) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926d4c); };
            c.w32(1, t462);
            let Some(t463) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926d51); };
            c.w32(2, t463);
            let r464 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r464);
            let Some(t465) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926d57); };
            c.w64(6, t465);
            let Some(t466) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926d5b); };
            c.w32(2, t466);
            let r467 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r467);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926d62); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926d66); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000926d67); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926d68); }
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000926d69); };
            c.w64(3, t468);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000926d7a, 0x7000926d75);
        }
        258 => {
            c.w64(11, c.r64(0));
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926d7d); };
            c.w64(15, t469);
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926d81); };
            c.w64(9, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926d85); };
            c.w64(8, t471);
            blk = 80; continue;
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r472 = c.r64(6) & 0x3ffu64;
            c.w64(6, r472);
            c.logic(r472, 64);
            let r473 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r473);
            let r474 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r474);
            let Some(t475) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926da6); };
            c.w32(1, t475);
            let r476 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r476);
            let Some(t477) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000926dac); };
            c.w32(1, t477);
            let r478 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r478);
            let Some(t479) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000926db2); };
            c.w32(2, t479);
            let r480 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r480);
            let Some(t481) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926db9); };
            c.w64(1, t481);
            let Some(t482) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926dbd); };
            c.w32(7, t482);
            let r483 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r483);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926dc3); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000926dc7); }
            c.w64(6, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000926dd2, 0x7000926dcd);
        }
        260 => {
            c.w64(11, c.r64(0));
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000926dd5); };
            c.w64(15, t484);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926dd9); };
            c.w64(9, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926ddd); };
            c.w64(8, t486);
            blk = 80; continue;
        }
        261 => {
            let Some(t487) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926de6); };
            c.w32(2, t487);
            let r488 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r488);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 263; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000926df1); };
            c.w64(2, t489);
            blk = 263; continue;
        }
        263 => {
            let Some(t490) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926df8); };
            c.w64(6, t490);
            let Some(t491) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926dfc); };
            c.w32(7, t491);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t492) = c.ld32(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926e02); };
            c.w32(12, t492);
            let Some(t493) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926e07); };
            c.w32(3, t493);
            let r494 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r494);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t495) = c.ld32(c.r64(3).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926e10); };
            c.w32(12, t495);
            let r496 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r496);
            let r497 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r497);
            let Some(t498) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926e1c); };
            c.w32(0, t498);
            let r499 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r499);
            let r500 = c.r64(0) & c.r64(7);
            c.w64(0, r500);
            c.logic(r500, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000926e27); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 264; continue;
        }
        264 => {
            c.w64(1, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(11, c.r64(1).wrapping_mul(4));
            let Some(t501) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926e3b); };
            c.w32(3, t501);
            let Some(t502) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000926e40); };
            c.cmp(t502, c.r32(3), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let r503 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r503);
            c.w64(11, c.r64(15).wrapping_add(c.r64(0)));
            c.w64(0, c.r64(7));
            let r504 = c.r64(0) & c.r64(11);
            c.w64(0, r504);
            c.logic(r504, 64);
            blk = 264; continue;
        }
        267 => {
            let Some(t505) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926e62); };
            c.w32(0, t505);
            let Some(t506) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926e65); };
            c.w32(3, t506);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r507 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r507);
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_b() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            return RegfileExit::fault(0x7000926e79);
        }
        269 => {
            let Some(t508) = c.ld32(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000926e7a); };
            c.w32(0, t508);
            let Some(t509) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000926e7f); };
            c.w64(1, t509);
            let Some(t510) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926e83); };
            c.w32(11, t510);
            let r511 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000926e8b); };
            c.w64(2, t512);
            let Some(t513) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926e8f); };
            c.w32(0, t513);
            let r514 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r514);
            c.logic(c.r8(3) & 0x2u64, 8);
            if c.cc_e() { blk = 321; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t515) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926e9f); };
            c.w32(3, t515);
            let Some(t516) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000926ea3); };
            c.cmp(t516, c.r32(3), 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t517) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000926eb0); };
            c.w32(2, t517);
            let r518 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r518);
            let Some(t519) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926eb7); };
            c.w32(11, t519);
            let r520 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r520);
            let Some(t521) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000926ebe); };
            c.logic(t521 & 0x2u64, 8);
            if c.cc_ne() { blk = 293; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t522) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000926ec9); };
            c.cmp(t522, c.r32(11), 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000926ed2); };
            c.w64(11, t523);
            blk = 321; continue;
        }
        274 => {
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000926ede); };
            c.w64(11, t524);
            let Some(t525) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000926ee5); };
            c.cmp(t525, 0x20u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t526) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000926eef); };
            c.w32(11, t526);
            let Some(t527) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000926ef3); };
            c.cmp(t527, c.r32(11), 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926efc); };
            c.w64(3, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926f00); };
            c.w64(0, t529);
            blk = 78; continue;
        }
        277 => {
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000926f09); };
            c.w64(3, t530);
            let Some(t531) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926f0d); };
            c.w32(11, t531);
            let r532 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r532);
            let Some(t533) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926f14); };
            c.w32(0, t533);
            let Some(t534) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000926f18); };
            c.cmp(t534, c.r32(0), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t535) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000926f21); };
            c.w32(11, t535);
            let r536 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r536);
            let Some(t537) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926f28); };
            c.w32(0, t537);
            let Some(t538) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000926f2c); };
            c.cmp(t538, c.r32(0), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 279; continue;
        }
        279 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t539) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926f3f); };
            c.w32(0, t539);
            let r540 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r540);
            let Some(t541) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926f46); };
            c.w32(1, t541);
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 285; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w32(11, c.r16(1));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t542) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000926f62); };
            c.cmp(t542, c.r32(0), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926f6b); };
            c.w64(3, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926f6f); };
            c.w64(0, t544);
            blk = 78; continue;
        }
        284 => {
            let Some(t545) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000926f78); };
            c.w32(0, t545);
            let r546 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r546);
            c.w64(3, c.r64(0));
            blk = 286; continue;
        }
        285 => {
            c.w64(3, c.r64(11));
            blk = 286; continue;
        }
        286 => {
            let Some(t547) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000926f86); };
            c.w32(11, t547);
            let r548 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r548);
            let Some(t549) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000926f8d); };
            c.w32(11, t549);
            let r550 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r550);
            let Some(t551) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000926f94); };
            c.w32(0, t551);
            let r552 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r552);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let r553 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r553);
            let Some(t554) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000926fa9); };
            c.w32(6, t554);
            let r555 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r555);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000926fb0); }
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926fb4); };
            if !c.push64(t556) { return RegfileExit::fault(0x7000926fb4); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000926fbe, 0x7000926fb9);
        }
        288 => {
            c.w64(11, c.r64(0));
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000926fc1); };
            c.w64(9, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000926fc5); };
            c.w64(8, t558);
            blk = 321; continue;
        }
        289 => {
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926fce); };
            c.w64(3, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926fd2); };
            c.w64(0, t560);
            blk = 78; continue;
        }
        290 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926fdb); };
            c.w64(3, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000926fdf); };
            c.w64(0, t562);
            blk = 78; continue;
        }
        291 => {
            c.w64(11, c.r64(9));
            c.w64(3, c.r64(11).wrapping_sub(0x1));
            let r563 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r563);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000926ff1); }
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000926ff5); };
            if !c.push64(t564) { return RegfileExit::fault(0x7000926ff5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000926ff8); }
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000926ff9); };
            if !c.push64(t565) { return RegfileExit::fault(0x7000926ff9); }
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000926ffc); };
            c.w64(3, t566);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000927010, 0x700092700b);
        }
        292 => {
            c.w64(11, c.r64(0));
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927013); };
            c.w64(9, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927017); };
            c.w64(8, t568);
            blk = 321; continue;
        }
        293 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r569 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r569);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000927029); }
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092702d); };
            if !c.push64(t570) { return RegfileExit::fault(0x700092702d); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x70007f2274, 0x700092703d, 0x7000927038);
        }
        294 => {
            c.w64(11, c.r64(0));
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927040); };
            c.w64(9, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927044); };
            c.w64(8, t572);
            blk = 321; continue;
        }
        295 => {
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092704d); };
            c.w64(0, t573);
            let Some(t574) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927051); };
            c.w32(3, t574);
            let r575 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r575);
            let Some(t576) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927057); };
            c.w32(2, t576);
            c.cmp(c.r16(2), 0x834u64, 16);
            if c.cc_e() { blk = 318; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w32(1, c.r64(2).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 306; continue; }
            blk = 297; continue;
        }
        297 => {
            c.cmp(c.r16(2), 0x411u64, 16);
            if c.cc_e() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092707c); };
            c.w64(3, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927080); };
            c.w64(0, t578);
            blk = 78; continue;
        }
        299 => {
            let Some(t579) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927089); };
            c.w32(11, t579);
            let Some(t580) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700092708d); };
            c.cmp(t580, c.r32(11), 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000927096); };
            c.w64(3, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092709a); };
            c.w64(0, t582);
            blk = 78; continue;
        }
        301 => {
            let Some(t583) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70009270a3); };
            c.w32(11, t583);
            let r584 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r584);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009270b0); };
            c.w64(3, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009270b4); };
            c.w64(0, t586);
            blk = 78; continue;
        }
        303 => {
            let Some(t587) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009270bd); };
            c.w32(0, t587);
            let r588 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r588);
            let Some(t589) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009270c4); };
            c.cmp(t589, 0x40u64, 16);
            if c.cc_b() { blk = 305; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009270cb); };
            c.w64(3, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009270cf); };
            c.w64(0, t591);
            blk = 78; continue;
        }
        305 => {
            let Some(t592) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009270d8); };
            c.w32(11, t592);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(0));
            blk = 321; continue;
        }
        306 => {
            let Some(t593) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009270e8); };
            c.w32(11, t593);
            let Some(t594) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70009270ec); };
            c.cmp(t594, c.r32(11), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009270f5); };
            c.w64(3, t595);
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009270f9); };
            c.w64(0, t596);
            blk = 78; continue;
        }
        308 => {
            let Some(t597) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000927102); };
            c.w32(11, t597);
            c.w64(3, c.r64(11));
            let r598 = c.r32(3) & 0xc0u64;
            c.w32(3, r598);
            c.logic(r598, 32);
            c.cmp(c.r32(3), 0xc0u64, 32);
            if c.cc_e() { blk = 311; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t599) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000927118); };
            c.w32(3, t599);
            let r600 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r600);
            let Some(t601) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092711e); };
            c.w32(3, t601);
            let r602 = c.r32(3) & 0x1fu64;
            c.w32(3, r602);
            c.logic(r602, 32);
            let r603 = c.sub(c.r32(3), 0x10u64, 32);
            c.w32(3, r603);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_be() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092712c); };
            c.w64(3, t604);
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927130); };
            c.w64(0, t605);
            blk = 78; continue;
        }
        311 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 317; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t606) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700092713f); };
            c.w32(0, t606);
            let r607 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r607);
            let Some(t608) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000927145); };
            c.cmp(t608, c.r32(0), 32);
            if c.cc_e() { blk = 316; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t609) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092714e); };
            c.w32(11, t609);
            let Some(t610) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000927152); };
            c.cmp(t610, c.r32(11), 32);
            if c.cc_ne() { blk = 315; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t611) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092715b); };
            c.w32(0, t611);
            let r612 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r612);
            blk = 315; continue;
        }
        315 => {
            c.w64(11, c.r64(0));
            blk = 321; continue;
        }
        316 => {
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000927166); };
            c.w64(3, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092716a); };
            c.w64(0, t614);
            blk = 78; continue;
        }
        317 => {
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000927173); };
            c.w64(3, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927177); };
            c.w64(0, t616);
            blk = 78; continue;
        }
        318 => {
            let Some(t617) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927180); };
            c.w32(11, t617);
            let Some(t618) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000927184); };
            c.cmp(t618, c.r32(11), 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092718d); };
            c.w64(3, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927191); };
            c.w64(0, t620);
            blk = 78; continue;
        }
        320 => {
            let Some(t621) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700092719a); };
            c.w32(0, t621);
            let r622 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r622);
            c.w64(11, c.r64(0));
            blk = 321; continue;
        }
        321 => {
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009271a3); };
            c.w64(15, t623);
            blk = 80; continue;
        }
        322 => {
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009271ac); };
            c.w64(3, t624);
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009271b0); };
            c.w64(0, t625);
            blk = 78; continue;
        }
        323 => {
            return RegfileExit::fault(0x70009271b9);
        }
        324 => {
            let Some(t626) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009271ba); };
            c.w64(0, t626);
            let Some(t627) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009271c1); };
            c.w64(3, t627);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009271c8); }
            let Some(t628) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009271cc); };
            c.w64(1, t628);
            let Some(t629) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70009271d3); };
            c.cmp(t629, c.r64(2), 64);
            if c.cc_a() { blk = 327; continue; }
            blk = 325; continue;
        }
        325 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009271e2, 0x70009271dd);
        }
        326 => {
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009271e2); };
            c.w64(11, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009271e6); };
            c.w64(15, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009271ea); };
            c.w64(9, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009271ee); };
            c.w64(8, t633);
            blk = 328; continue;
        }
        327 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70009271f4); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 328; continue;
        }
        328 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t634) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927204); };
            c.w64(11, t634);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092720b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092720f); }
            blk = 58; continue;
        }
        329 => {
            let Some(t635) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000927219); };
            c.w64(0, t635);
            let Some(t636) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000927220); };
            c.w64(3, t636);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000927227); }
            let Some(t637) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092722b); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000927232); };
            c.cmp(t638, c.r64(2), 64);
            if c.cc_a() { blk = 332; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000927241, 0x700092723c);
        }
        331 => {
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927241); };
            c.w64(11, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927245); };
            c.w64(15, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927249); };
            c.w64(9, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092724d); };
            c.w64(8, t642);
            blk = 333; continue;
        }
        332 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x7000927253); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 333; continue;
        }
        333 => {
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t643) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927266); };
            c.w64(11, t643);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092726d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000927271); }
            blk = 61; continue;
        }
        334 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000927280); }
            return c.call(0x70007febb4, 0x7000927289, 0x7000927284);
        }
        335 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927289); };
            c.w64(11, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092728d); };
            c.w64(15, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927291); };
            c.w64(9, t646);
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927295); };
            c.w64(8, t647);
            blk = 65; continue;
        }
        336 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70009272a8, 0x70009272a3);
        }
        337 => {
            c.w64(11, c.r64(0));
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009272ab); };
            c.w64(15, t648);
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009272af); };
            c.w64(9, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009272b3); };
            c.w64(8, t650);
            blk = 70; continue;
        }
        338 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70009272c1); }
            return c.call(0x70007febf4, 0x70009272cb, 0x70009272c6);
        }
        339 => {
            c.w64(11, c.r64(0));
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009272ce); };
            c.w64(15, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009272d2); };
            c.w64(9, t652);
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009272d6); };
            c.w64(8, t653);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70009272da); }
            blk = 76; continue;
        }
        340 => {
            c.w64(11, c.r64(9));
            let r654 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r654);
            let r655 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r655);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009272ee); }
            let Some(t656) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70009272f2); };
            c.w64(8, t656);
            let Some(t657) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009272f9); };
            c.w32(8, t657);
            let r658 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r658);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000927300); }
            let Some(t659) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000927302); };
            c.w64(3, t659);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000927315, 0x7000927310);
        }
        341 => {
            return RegfileExit::fault(0x7000927315);
        }
        342 => {
            blk = 78; continue;
        }
        343 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            let r660 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r660);
            let Some(t661) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927324); };
            c.w64(2, t661);
            let Some(t662) = c.ld32(c.r64(2).wrapping_add(c.r64(11)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927328); };
            c.w32(1, t662);
            let Some(t663) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092732d); };
            c.w32(2, t663);
            let r664 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r664);
            let Some(t665) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927333); };
            c.w64(6, t665);
            let Some(t666) = c.ld32(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927337); };
            c.w32(1, t666);
            let r667 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r667);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092733e); }
            c.w32(0, 0xau64);
            c.w64(2, c.r64(3));
            return c.call(0x7000804ff4, 0x700092734f, 0x700092734a);
        }
        344 => {
            c.w64(11, c.r64(0));
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927352); };
            c.w64(15, t668);
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927356); };
            c.w64(9, t669);
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092735a); };
            c.w64(8, t670);
            blk = 80; continue;
        }
        345 => {
            return RegfileExit::jump(0x7000927364);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927378); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092737e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927382); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927386); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092738b); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927392); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            let Some(t8) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927398); };
            c.w32(2, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092739d); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009273a5); };
            c.w64(3, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009273a9); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009273b0); };
            c.w32(6, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009273b5); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009273bb); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009273c2); };
            c.w32(0, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009273c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009273ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009273ce); }
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009273d2); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009273db); };
            c.w64(11, t21);
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(9).wrapping_sub(0x1));
            let Some(t22) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009273e6); };
            c.w64(2, t22);
            let r23 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r23);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009273f2); }
            c.w64(0, c.r64(6));
            c.w64(6, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000834434, 0x7000927407, 0x7000927402);
        }
        3 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927407); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092740b); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092740f); };
            c.w64(11, t26);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000927417); };
            c.w32(11, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092741c); };
            c.w64(15, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927420); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927424); };
            c.w64(5, t30);
            c.w64(12, c.r64(8));
            let r31 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000927434);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927454); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927458); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092745e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927462); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927466); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092746b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927472); };
            c.w64(11, t6);
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927476); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092747d); };
            c.w32(3, t9);
            let Some(t10) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927482); };
            c.w32(2, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000927486); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092748a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092748e); }
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927492); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092749b); };
            c.w64(8, t12);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t13) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009274a6); };
            c.w64(1, t13);
            let r14 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r14);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009274b2); }
            c.w64(2, c.r64(11));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(8));
            return c.call(0x7000836434, 0x70009274ca, 0x70009274c5);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009274ca); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009274ce); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009274d2); };
            c.w64(8, t17);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009274da); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009274df); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70009274e3); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009274e7); };
            c.w64(5, t21);
            c.w64(12, c.r64(7));
            let r22 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009274f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927517); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092751b); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927520); };
            c.w32(6, t2);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092752a); };
            c.w32(7, t3);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000927539); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let r6 = c.add(c.r32(6), 0xffffffffffffffffu64, 32);
            c.w32(6, r6);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000927546);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927547); };
            c.w32(6, t7);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000927556);
        }
        7 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000927557); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000927566); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092756c); };
            c.w32(6, t12);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, 0xffffffffffffffffu64);
            let r13 = c.sub(c.r64(7), c.r64(2), 64);
            c.w64(7, r13);
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_ae() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927584); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092758b); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            blk = 12; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927593); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927599); };
            c.w32(6, t20);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r21 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r21);
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009275ab); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009275b2); };
            c.w32(3, t24);
            let r25 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r25);
            blk = 12; continue;
        }
        12 => {
            let r26 = c.add(c.r64(9), 0x9u64, 64);
            c.w64(9, r26);
            let Some(t27) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70009275bc); };
            c.w32(2, t27);
            let Some(t28) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009275c1); };
            c.w64(1, t28);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            return RegfileExit::fault(0x70009275ca);
        }
        14 => {
            return RegfileExit::fault(0x70009275cb);
        }
        15 => {
            return RegfileExit::jump(0x70009275d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009275d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009275d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009275d8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009275de); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009275e2); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009275e6); };
            c.w32(8, t3);
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009275eb); };
            c.w32(11, t4);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70009275f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70009275f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70009275f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70009275fc); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927606); };
            c.w32(3, t5);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000927615); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let r8 = c.add(c.r32(11), 0xffffffffffffffffu64, 32);
            c.w32(11, r8);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x7000927623);
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927624); };
            c.w32(11, t9);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x7000927634);
        }
        7 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000927638); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(7, c.r64(9).wrapping_sub(0x1));
            let Some(t12) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927648); };
            c.w64(8, t12);
            let r13 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r13);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700092764e); }
            c.w32(7, 0x6au64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000927657); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000927658); };
            c.w64(3, t14);
            c.w64(6, c.r64(8));
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700092766f, 0x700092766a);
        }
        9 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092766f); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927673); };
            c.w64(15, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927677); };
            c.w64(9, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092767b); };
            c.w64(2, t18);
            blk = 13; continue;
        }
        10 => {
            let r19 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r19);
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927685); };
            c.w32(7, t20);
            let r21 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r21);
            let Some(t22) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092768b); };
            c.w32(11, t22);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092769a); };
            c.w32(7, t23);
            let r24 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r24);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70009276a2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r25 = c.r64(8) & c.r64(7);
            c.w64(8, r25);
            c.logic(r25, 64);
            let Some(t26) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009276b3); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t27) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009276ba); };
            c.w64(2, t27);
            c.w64(7, c.r64(9));
            c.w64(9, c.r64(7).wrapping_add(0x9));
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70009276c5); };
            c.w32(7, t28);
            let Some(t29) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70009276ca); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009276ce); };
            c.w64(5, t30);
            c.w64(12, c.r64(2));
            let r31 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r31);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            return RegfileExit::fault(0x70009276db);
        }
        15 => {
            c.w32(0, c.r32(0));
            let r32 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r32);
            c.w64(8, 0xfffffffffffc0000u64);
            let r33 = c.r64(8) & c.r64(0);
            c.w64(8, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70009276eb); };
            c.logic(t34 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(7).wrapping_add(0x3));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70009276f6); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000927700, 0x70009276fb);
        }
        17 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000927700); }
            blk = 13; continue;
        }
        18 => {
            return RegfileExit::jump(0x7000927708);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927714); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927718); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092771e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927722); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927726); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092772b); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927732); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927737); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x700092773f); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927743); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092774a); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092774f); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927756); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092775a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092775e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000927762); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927766); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092776f); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092777a); };
            c.w64(7, t18);
            let r19 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000927786); }
            c.w64(2, c.r64(3));
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x700092779e, 0x7000927799);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092779e); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009277a2); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009277a6); };
            c.w64(11, t22);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009277ae); };
            c.w32(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009277b3); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009277b7); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009277bb); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70009277cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009277d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009277d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009277d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009277de); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009277e2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009277e6); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009277eb); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009277f2); };
            c.w32(2, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009277f7); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009277ff); };
            c.w64(3, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927803); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x700092780a); };
            c.w32(6, t12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092780f); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927816); };
            c.w32(2, t15);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092781a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092781e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000927822); }
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927826); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092782f); };
            c.w64(11, t17);
            blk = 2; continue;
        }
        2 => {
            c.w64(2, c.r64(9).wrapping_sub(0x1));
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092783a); };
            c.w64(7, t18);
            let r19 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r19);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000927846); }
            c.w64(2, c.r64(3));
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(3, c.r64(11));
            return c.call(0x700083d494, 0x700092785e, 0x7000927859);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092785e); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927862); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927866); };
            c.w64(11, t22);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700092786e); };
            c.w32(11, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927873); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927877); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092787b); };
            c.w64(5, t26);
            c.w64(12, c.r64(8));
            let r27 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r27);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x700092788c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927894); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927898); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092789e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009278a2); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009278a6); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009278ab); };
            c.w64(2, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009278b0); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70009278b7); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70009278bb); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009278bf); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x70009278c6); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009278cb); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009278cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009278d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70009278d7); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009278db); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009278e4); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009278ef); };
            c.w64(7, t15);
            let r16 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r16);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009278fb); }
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x700083eb74, 0x7000927913, 0x700092790e);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927913); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927917); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092791b); };
            c.w64(11, t19);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000927923); };
            c.w32(11, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927928); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092792c); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927930); };
            c.w64(5, t23);
            c.w64(12, c.r64(8));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000927940);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927954); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927958); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092795e); };
            c.w64(8, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927962); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927966); };
            c.w64(3, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092796b); };
            c.w64(2, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000927970); };
            c.w32(11, t5);
            let r6 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927977); };
            c.w64(3, t7);
            let Some(t8) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x700092797b); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092797f); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000927986); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092798b); };
            c.w32(6, t12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092798f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000927993); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000927997); }
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092799b); };
            c.cmp(t13, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009279a4); };
            c.w64(11, t14);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t15) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009279af); };
            c.w64(7, t15);
            let r16 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r16);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70009279bb); }
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(3));
            c.w64(3, c.r64(11));
            return c.call(0x7000840434, 0x70009279d3, 0x70009279ce);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009279d3); };
            c.w64(8, t17);
            let Some(t18) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009279d7); };
            c.w64(8, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009279db); };
            c.w64(11, t19);
            c.w64(9, c.r64(11).wrapping_add(0xd));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70009279e3); };
            c.w32(11, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009279e8); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009279ec); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009279f0); };
            c.w64(5, t23);
            c.w64(12, c.r64(8));
            let r24 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r24);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000927a00);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927a14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927a14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927a18); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927a1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927a22); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927a26); };
            c.w64(8, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927a2b); };
            c.w64(11, (t4 as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000927a30); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927a36); };
            c.w64(8, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927a3a); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927a3e); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000927a44); };
            c.w32(3, t11);
            let Some(t12) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000927a4a); };
            c.w32(1, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927a4f); };
            c.w32(6, t13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000927a52); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000927a56); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000927a5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000927a5e); }
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927a62); };
            c.cmp(t14, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927a6b); };
            c.w64(7, t15);
            blk = 2; continue;
        }
        2 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927a76); };
            c.w64(12, t16);
            let r17 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r18 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000927a84); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000927a88); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000927a8a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000927a8c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000927a8d); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000927a8e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000927a8f); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2360)) else { return RegfileExit::fault(0x7000927a90); };
            c.w64(3, t19);
            c.w64(6, c.r64(12));
            c.w32(0, 0x6u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000927aa7, 0x7000927aa2);
        }
        3 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927aa7); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927aab); };
            c.w64(2, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927aaf); };
            c.w64(7, t22);
            c.w64(9, c.r64(7).wrapping_add(0xe));
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000927ab7); };
            c.w32(7, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927abc); };
            c.w64(15, t24);
            let Some(t25) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927ac0); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927ac4); };
            c.w64(5, t26);
            c.w64(12, c.r64(2));
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000927acb); };
            c.w64(0, t27);
            let r28 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r28);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000927ad8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927af4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927af8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927afe); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927b02); };
            c.w64(7, t2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000927b06); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927b0c); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927b12); };
            c.w32(8, t7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927b17); };
            c.w32(11, t8);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000927b1b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000927b1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000927b23); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000927b27); }
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927b2b); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927b34); };
            c.w64(7, t10);
            blk = 2; continue;
        }
        2 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927b3f); };
            c.w64(3, t11);
            let r12 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r12);
            let r13 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r13);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000927b49); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000927b4d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000927b4f); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000927b50); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2320)) else { return RegfileExit::fault(0x7000927b59); };
            c.w64(3, t14);
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000927b6b, 0x7000927b66);
        }
        3 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927b6b); };
            c.w64(2, t15);
            let Some(t16) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927b6f); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927b73); };
            c.w64(7, t17);
            c.w64(9, c.r64(7).wrapping_add(0x5));
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000927b7b); };
            c.w32(7, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927b80); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927b84); };
            c.w64(1, t20);
            let Some(t21) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927b88); };
            c.w64(5, t21);
            c.w64(12, c.r64(2));
            let Some(t22) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000927b8f); };
            c.w64(0, t22);
            let r23 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r23);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x7000927b9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927bb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927bb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927bb8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927bbe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927bc2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927bc6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000927bcb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927bd2); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927bd6); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927bda); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927be1); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927be6); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000927bea); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000927bee); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000927bf2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000927bf6); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927bfa); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927c03); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(11));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927c19); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927c36); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927c39); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000927c49); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        6 => {
            let r18 = c.add(c.r32(1), c.r32(6), 32);
            c.w32(1, r18);
            if c.cc_o() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927c65); };
            c.w32(11, t19);
            c.w32(0, c.r32(11));
            let r20 = c.r32(0) | 0x2u64;
            c.w32(0, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000927c78); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927c7d); }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        11 => {
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        12 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927ca5); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927cac); };
            c.cmp(t25, c.r32(1), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927cbf); };
            c.w32(1, t26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927cc2); };
            c.cmp(t27, c.r32(1), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000927ccf); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000927cd5); }
            blk = 17; continue;
        }
        16 => {
            let r28 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000927cde); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927cf1); };
            c.w32(11, t29);
            c.w32(0, c.r32(11));
            let r30 = c.r32(0) | 0xeu64;
            c.w32(0, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000927d04); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927d09); }
            blk = 20; continue;
        }
        20 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000927d11); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000927d18); };
            c.w64(11, t32);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000927d1f); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000927d26); };
            c.cmp(t34, c.r64(0), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000927d2f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927d3a); };
            c.w64(11, t35);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000927d41); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000927d45); }
            blk = 45; continue;
        }
        23 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let r36 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r36);
            let Some(t37) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927d56); };
            c.w32(1, t37);
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927d7e); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927d84); };
            c.cmp(t40, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 38; continue;
        }
        29 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927d94); };
            c.w32(1, t41);
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            let Some(t43) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927d9a); };
            c.cmp(t43, 0x40u64, 16);
            if c.cc_ae() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t44) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927da6); };
            c.w32(3, t44);
            c.w32(1, c.r32(3));
            let r45 = c.r32(1) | 0x20u64;
            c.w32(1, r45);
            c.logic(r45, 32);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x7000927db7); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927dbc); }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000927dc4); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927dce); };
            c.w64(6, t46);
            return c.call(0x700086f9d4, 0x7000927dd7, 0x7000927dd2);
        }
        34 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927de5); };
            c.w32(1, t47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927de8); };
            c.cmp(t48, c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927df1); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000927df7); };
            c.cmp(t51, 0x43u64, 16);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(1, 0xfeu64);
            blk = 40; continue;
        }
        39 => {
            c.w32(1, 0x1eu64);
            blk = 40; continue;
        }
        40 => {
            c.w64(6, c.r64(9));
            c.w64(9, c.r64(6).wrapping_sub(0x1));
            let r52 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r52);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927e19); };
            c.w32(3, t53);
            c.w32(1, c.r32(1));
            c.w32(12, c.r32(3));
            let r54 = c.r32(1) | c.r32(12);
            c.w32(1, r54);
            c.logic(r54, 32);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(3, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(3)) { return RegfileExit::fault(0x7000927e2d); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927e32); }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000927e3a); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927e44); };
            c.w64(6, t55);
            return c.call(0x70008ad794, 0x7000927e4d, 0x7000927e48);
        }
        44 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        45 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927e50); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927e54); };
            c.w64(7, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927e58); };
            c.w64(11, t58);
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000927e60); };
            c.w32(11, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927e65); };
            c.w64(15, t60);
            let Some(t61) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927e69); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927e6d); };
            c.w64(5, t62);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(8));
            let r63 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r63);
            return RegfileExit::jump(c.r64(1));
        }
        46 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000927e82); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x7000927e87); }
            return c.call(0x70007febf4, 0x7000927e91, 0x7000927e8c);
        }
        47 => {
            c.w64(8, c.r64(0));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927e94); };
            c.w64(15, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000927e98); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927e9c); };
            c.w64(7, t66);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000927ea0); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000927ea5); }
            blk = 22; continue;
        }
        48 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000927eaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000927eb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000927eb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000927ebb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000927ebf); }
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(11));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927ec9); };
            c.w64(6, t67);
            return c.call(0x700089b334, 0x7000927ed2, 0x7000927ecd);
        }
        49 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t68) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000927ed6); };
            c.cmp(t68, 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000927edc); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927ee0); };
            c.w64(2, t70);
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927ee4); };
            c.w32(7, t71);
            c.w32(11, c.r32(7));
            let r72 = c.r32(11) | 0x40u64;
            c.w32(11, r72);
            c.logic(r72, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000927ef8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927efd); }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, c.r64(0));
            blk = 45; continue;
        }
        54 => {
            let Some(t73) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000927f0d); };
            c.cmp(t73, 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000927f13); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000927f17); };
            c.w64(7, t75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000927f1b); };
            c.w32(11, t76);
            c.w32(0, c.r32(11));
            let r77 = c.r32(0) | 0xfeu64;
            c.w32(0, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000927f30); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000927f35); }
            blk = 57; continue;
        }
        57 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000927f3d); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000927f41); };
            c.w64(6, t79);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000927f45); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000927f4e); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x7000927f4f); };
            c.w64(3, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000927f5b); };
            c.w64(6, t81);
            return c.call(0x700086ed74, 0x7000927f64, 0x7000927f5f);
        }
        58 => {
            return RegfileExit::fault(0x7000927f64);
        }
        59 => {
            return RegfileExit::jump(0x7000927f68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000927f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000927f74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000927f78); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000927f7e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000927f82); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000927f86); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000927f8b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000927f92); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000927f96); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000927f9a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000927fa1); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927fa6); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000927faa); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000927fae); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000927fb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000927fb6); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000927fba); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927fc3); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(11));
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000927fcd); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(12, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000927fed); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000927ff0); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928000); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 19; continue;
        }
        6 => {
            let r18 = c.sub(c.r32(1), c.r32(12), 32);
            c.w32(1, r18);
            if c.cc_o() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        8 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928022); };
            c.w64(3, t19);
            let Some(t20) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000928029); };
            c.w64(1, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            let r22 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r22);
            c.w64(12, c.r64(1).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000928035); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700092803c); };
            c.cmp(t24, c.r64(12), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x7000928045); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928061); };
            c.w64(11, t25);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928068); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092806c); }
            c.w32(0, 0xeu64);
            c.w64(11, c.r64(3));
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928082); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r27 = c.r32(0) | c.r32(1);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000928098); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092809d); }
            blk = 41; continue;
        }
        14 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009280aa); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009280b1); };
            c.cmp(t30, c.r32(1), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009280c2); };
            c.w32(1, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009280c5); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009280d2); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009280d8); }
            blk = 19; continue;
        }
        18 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009280e1); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009280f4); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000928107); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092810c); }
            blk = 22; continue;
        }
        22 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928114); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092811b); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000928122); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000928129); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000928132); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092813d); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928144); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000928148); }
            c.w64(11, c.r64(8));
            blk = 41; continue;
        }
        25 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r41 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let Some(t42) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092815d); };
            c.w32(1, t42);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928171); };
            c.w32(1, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928174); };
            c.cmp(t44, c.r32(1), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        30 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928184); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092818a); };
            c.cmp(t47, 0x43u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        32 => {
            c.w32(1, 0x1eu64);
            blk = 36; continue;
        }
        33 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009281a3); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009281a9); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(1, 0xfeu64);
            blk = 36; continue;
        }
        36 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r51 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r51);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009281c5); };
            c.w32(6, t52);
            c.w32(1, c.r32(1));
            c.w32(11, c.r32(6));
            let r53 = c.r32(1) | c.r32(11);
            c.w32(1, r53);
            c.logic(r53, 32);
            c.cmp(c.r32(6), c.r32(1), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009281d9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009281de); }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009281e6); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009281f0); };
            c.w64(0, t54);
            return c.call(0x70008adb14, 0x70009281f9, 0x70009281f4);
        }
        40 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009281fc); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928200); };
            c.w64(7, t56);
            blk = 41; continue;
        }
        41 => {
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928204); };
            c.w64(7, t57);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092820f); };
            c.w32(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928214); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928218); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092821c); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092822c); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000928235); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000928239); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092823d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000928241); }
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x700092824d, 0x7000928248);
        }
        43 => {
            c.w64(3, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928250); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928254); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000928258); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092825c); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000928260); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000928264); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000928268); };
            c.w64(0, t69);
            blk = 10; continue;
        }
        44 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x7000928276); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x700092827b); }
            return c.call(0x70007febf4, 0x7000928285, 0x7000928280);
        }
        45 => {
            c.w64(8, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928288); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092828c); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928290); };
            c.w64(7, t72);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000928294); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000928299); }
            blk = 24; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70009282a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009282a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70009282ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009282af); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70009282b3); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009282bc); };
            c.w32(1, t73);
            c.w32(11, c.r32(1));
            let r74 = c.r32(11) | 0x40u64;
            c.w32(11, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009282d0); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009282d5); }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009282dd); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009282e7); };
            c.w64(0, t75);
            return c.call(0x700089bcd4, 0x70009282f0, 0x70009282eb);
        }
        50 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009282f7); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009282fb); };
            c.w64(7, t77);
            blk = 41; continue;
        }
        52 => {
            let Some(t78) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000928304); };
            c.cmp(t78, 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092830a); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092830e); };
            c.w64(2, t80);
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928312); };
            c.w32(7, t81);
            c.w32(11, c.r32(7));
            let r82 = c.r32(11) | 0xfeu64;
            c.w32(11, r82);
            c.logic(r82, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x7000928329); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092832e); }
            blk = 55; continue;
        }
        55 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928336); };
            c.w64(7, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092833a); };
            c.w64(12, t84);
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092833e); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000928347); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x7000928348); };
            c.w64(3, t85);
            c.w32(0, 0x1u64);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000928354); };
            c.w64(6, t86);
            return c.call(0x700086ed74, 0x700092835d, 0x7000928358);
        }
        56 => {
            return RegfileExit::fault(0x700092835d);
        }
        57 => {
            return RegfileExit::jump(0x7000928360);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000928374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000928374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000928378); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092837e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928382); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000928386); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092838b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000928392); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928396); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092839a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70009283a1); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009283a6); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70009283aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70009283ae); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70009283b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70009283b6); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009283ba); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009283c3); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009283ca); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009283e5); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009283e8); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009283f8); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 24; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.w64(6, c.r64(11));
            let r19 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r19);
            c.w64(12, c.r64(6));
            let r20 = c.imul2(c.r32(12), c.r32(3), 32);
            c.w32(12, r20);
            if c.cc_o() { blk = 52; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(11, c.r64(12));
            let r21 = c.add(c.r32(11), c.r32(12), 32);
            c.w32(11, r21);
            if c.cc_o() { blk = 47; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 13; continue;
        }
        10 => {
            let r22 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            let r23 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r23);
            let r24 = c.r32(0) | c.r32(11);
            c.w32(0, r24);
            c.logic(r24, 32);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x700092844b); };
            c.w64(11, t25);
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(0, 0xeu64);
            blk = 16; continue;
        }
        15 => {
            c.w32(0, 0x2u64);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092846d); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(1, c.r32(3));
            let r27 = c.r32(0) | c.r32(1);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000928483); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928488); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928495); };
            c.w32(6, t28);
            let r29 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092849c); };
            c.cmp(t30, c.r32(6), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009284ad); };
            c.w32(6, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009284b0); };
            c.cmp(t32, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009284bd); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009284c3); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70009284cc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009284df); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009284f2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009284f7); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009284ff); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000928506); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092850d); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000928514); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 57; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092851d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928528); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092852f); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000928533); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928541); };
            c.w32(6, t41);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r42 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092855c); };
            c.w32(6, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092855f); };
            c.cmp(t44, c.r32(6), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092856f); };
            c.w32(6, t45);
            let r46 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r46);
            let Some(t47) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928575); };
            c.cmp(t47, 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        37 => {
            c.w32(6, 0x1eu64);
            blk = 41; continue;
        }
        38 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092858e); };
            c.w32(6, t48);
            let r49 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r49);
            let Some(t50) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928594); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_e() { blk = 59; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(6, 0xfeu64);
            blk = 41; continue;
        }
        41 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r51 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r51);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009285b0); };
            c.w32(1, t52);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r53 = c.r32(6) | c.r32(11);
            c.w32(6, r53);
            c.logic(r53, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009285c4); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009285c9); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70009285d1); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009285db); };
            c.w64(0, t54);
            return c.call(0x70008adcd4, 0x70009285e4, 0x70009285df);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009285e7); };
            c.w64(15, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009285eb); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009285ef); };
            c.w64(7, t57);
            blk = 46; continue;
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009285f3); };
            c.w64(7, t58);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70009285fe); };
            c.w32(8, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928603); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000928607); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928617); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092861e); };
            c.w64(0, t64);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000928625); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000928629); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700092862d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000928631); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000928635); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700092863c); };
            c.cmp(t66, c.r64(3), 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700092864e, 0x7000928649);
        }
        49 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092864e); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928652); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928656); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092865a); };
            c.w64(7, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092865e); };
            c.w64(8, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000928662); };
            c.w64(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000928666); };
            c.w64(1, t73);
            blk = 51; continue;
        }
        50 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700092866c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 51; continue;
        }
        51 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092867f); };
            c.w64(11, t74);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928686); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092868a); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        52 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928697); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092869e); };
            c.w64(0, t76);
            c.w64(12, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70009286a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70009286a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70009286ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x70009286b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70009286b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70009286b9); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009286bd); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70009286c4); };
            c.cmp(t78, c.r64(12), 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70009286d6, 0x70009286d1);
        }
        54 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009286d6); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009286da); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009286de); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009286e2); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009286e6); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009286ea); };
            c.w64(2, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009286ee); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009286f2); };
            c.w64(6, t86);
            blk = 56; continue;
        }
        55 => {
            if !c.st64(c.r64(11), c.r64(12)) { return RegfileExit::fault(0x70009286f8); }
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009286fb); };
            c.w64(11, t87);
            let r88 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r88);
            c.w64(0, c.r64(11));
            blk = 56; continue;
        }
        56 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            c.arith_sd(2, 0, c.xmm_u64(1));
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092871a); };
            c.w64(11, t89);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928721); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000928725); }
            c.w64(11, c.r64(0));
            blk = 13; continue;
        }
        57 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000928737); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x700092873c); }
            return c.call(0x70007febf4, 0x7000928746, 0x7000928741);
        }
        58 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928749); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092874d); };
            c.w64(9, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928751); };
            c.w64(7, t92);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000928755); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700092875a); }
            blk = 29; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 62; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928769); };
            c.w32(1, t93);
            c.w32(6, c.r32(1));
            let r94 = c.r32(6) | 0x40u64;
            c.w32(6, r94);
            c.logic(r94, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700092877a); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092877f); }
            blk = 62; continue;
        }
        62 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000928787); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092878b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092878d); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000928794); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092879e); };
            c.w64(3, t95);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70009287ad, 0x70009287a8);
        }
        63 => {
            c.w64(11, c.r64(0));
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009287b0); };
            c.w64(15, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009287b4); };
            c.w64(9, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009287b8); };
            c.w64(7, t98);
            blk = 46; continue;
        }
        64 => {
            return RegfileExit::jump(0x70009287c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70009287d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70009287d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70009287d8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009287de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009287e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70009287e6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70009287eb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70009287f2); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70009287f6); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70009287fa); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000928801); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928806); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092880a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092880e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000928812); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000928816); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092881a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000928823); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092882a); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928847); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092884a); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092885a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 24; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.w64(12, c.r64(11));
            let r19 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r19);
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0xffffffffffffffffu64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0xc0000000u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(12));
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w64(11, c.r64(0));
            let r20 = c.imul2(c.r32(11), c.r32(3), 32);
            c.w32(11, r20);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(0, 0x2u64);
            blk = 16; continue;
        }
        13 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009288b3); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70009288ba); };
            c.w64(0, t22);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009288c1); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70009288c8); };
            c.cmp(t24, c.r64(2), 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70009288d1); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009288f0); };
            c.w64(0, t25);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70009288f7); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009288fb); }
            c.w32(0, 0x6u64);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092890f); };
            c.w32(3, t26);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r27 = c.r32(0) | c.r32(2);
            c.w32(0, r27);
            c.logic(r27, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000928925); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092892a); }
            blk = 46; continue;
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928937); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092893e); };
            c.cmp(t30, c.r32(2), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092894f); };
            c.w32(2, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928952); };
            c.cmp(t32, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700092895f); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928965); }
            blk = 24; continue;
        }
        23 => {
            let r33 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r33);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700092896e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928981); };
            c.w32(11, t34);
            c.w32(0, c.r32(11));
            let r35 = c.r32(0) | 0xeu64;
            c.w32(0, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000928994); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928999); }
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70009289a1); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70009289a8); };
            c.w64(11, t37);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009289af); };
            c.w64(3, t38);
            let Some(t39) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70009289b6); };
            c.cmp(t39, c.r64(0), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70009289bf); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            c.arith_sd(3, 1, c.xmm_u64(0));
            c.mov128_rr(1, 1);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009289cd); };
            c.w64(11, t40);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70009289d4); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x70009289d8); }
            c.w64(11, c.r64(8));
            blk = 46; continue;
        }
        30 => {
            let Some(t41) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009289e6); };
            c.w32(2, t41);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r42 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928a01); };
            c.w32(2, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928a04); };
            c.cmp(t44, c.r32(2), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        35 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928a14); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928a1a); };
            c.cmp(t47, 0x43u64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        37 => {
            c.w32(2, 0x1eu64);
            blk = 41; continue;
        }
        38 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928a33); };
            c.w32(2, t48);
            let r49 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r49);
            let Some(t50) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928a39); };
            c.cmp(t50, 0x41u64, 16);
            if c.cc_e() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(2, 0xfeu64);
            blk = 41; continue;
        }
        41 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r51 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r51);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928a55); };
            c.w32(6, t52);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r53 = c.r32(2) | c.r32(11);
            c.w32(2, r53);
            c.logic(r53, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000928a69); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928a6e); }
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000928a76); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000928a80); };
            c.w64(0, t54);
            return c.call(0x70008adf94, 0x7000928a89, 0x7000928a84);
        }
        45 => {
            c.w64(11, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928a8c); };
            c.w64(9, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928a90); };
            c.w64(7, t56);
            blk = 46; continue;
        }
        46 => {
            let Some(t57) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928a94); };
            c.w64(7, t57);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t58) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000928a9f); };
            c.w32(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928aa4); };
            c.w64(15, t59);
            let Some(t60) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928aa8); };
            c.w64(1, t60);
            let Some(t61) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000928aac); };
            c.w64(5, t61);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r62 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r62);
            return RegfileExit::jump(c.r64(1));
        }
        47 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000928ac1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000928ac5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000928ac9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000928acd); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x7000928ad1); }
            return c.call(0x70007febf4, 0x7000928ada, 0x7000928ad5);
        }
        48 => {
            c.w64(11, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928add); };
            c.w64(9, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928ae1); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000928ae5); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000928ae9); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000928aed); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000928af1); };
            c.w64(12, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000928af5); };
            c.w64(3, t69);
            blk = 15; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x7000928b03); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000928b08); }
            return c.call(0x70007febf4, 0x7000928b12, 0x7000928b0d);
        }
        50 => {
            c.w64(8, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928b15); };
            c.w64(15, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928b19); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928b1d); };
            c.w64(7, t72);
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000928b21); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000928b26); }
            blk = 29; continue;
        }
        51 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928b35); };
            c.w32(2, t73);
            c.w32(6, c.r32(2));
            let r74 = c.r32(6) | 0x40u64;
            c.w32(6, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000928b46); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928b4b); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000928b53); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000928b57); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000928b59); }
            c.w32(8, 0x6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000928b60); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000928b6a); };
            c.w64(3, t75);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000928b79, 0x7000928b74);
        }
        55 => {
            c.w64(11, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928b7c); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928b80); };
            c.w64(7, t77);
            blk = 46; continue;
        }
        56 => {
            return RegfileExit::jump(0x7000928b8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000928b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000928b94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000928b98); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000928b9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928ba2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000928ba6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000928bab); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000928bb2); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928bb6); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000928bba); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000928bc1); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928bc6); };
            c.w32(2, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000928bca); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000928bce); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000928bd2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000928bd6); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000928bda); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000928be3); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000928bea); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928c07); };
            c.w32(2, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928c0a); };
            c.cmp(t16, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928c1a); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 20; continue;
        }
        6 => {
            c.w64(3, c.r64(0));
            let r18 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r18);
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 7; continue;
        }
        7 => {
            let r19 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r19);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(11));
            c.cdq();
            c.idiv(c.r32(3), 32);
            c.w32(11, c.r64(2).wrapping_add(c.r64(2)));
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(0, 0xeu64);
            blk = 12; continue;
        }
        11 => {
            c.w32(0, 0x2u64);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928c6d); };
            c.w32(3, t20);
            c.w32(0, c.r32(0));
            c.w32(2, c.r32(3));
            let r21 = c.r32(0) | c.r32(2);
            c.w32(0, r21);
            c.logic(r21, 32);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x7000928c83); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928c88); }
            blk = 44; continue;
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928c95); };
            c.w32(2, t22);
            let r23 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928c9c); };
            c.cmp(t24, c.r32(2), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 16; continue;
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928cad); };
            c.w32(2, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928cb0); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928cbd); }
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928cc3); }
            blk = 20; continue;
        }
        19 => {
            let r27 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r27);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000928ccc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928cdf); };
            c.w32(11, t28);
            c.w32(0, c.r32(11));
            let r29 = c.r32(0) | 0xeu64;
            c.w32(0, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000928cf2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928cf7); }
            blk = 23; continue;
        }
        23 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928cff); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000928d06); };
            c.w64(11, t31);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000928d0d); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000928d14); };
            c.cmp(t33, c.r64(0), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000928d1d); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            let r34 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r34);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000928d28); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000928d2d); }
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000928d30); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000928d35); }
            blk = 26; continue;
        }
        26 => {
            c.fprem();
            c.fnstsw_ax();
            let r35 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r35);
            let r36 = c.r32(0) & 0xffu64;
            c.w32(0, r36);
            c.logic(r36, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000928d44); }
            if !c.popfq() { return RegfileExit::fault(0x7000928d45); }
            if c.cc_p() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000928d4a); }
            if !c.movsd_rm(1, c.r64(4)) { return RegfileExit::fault(0x7000928d4d); }
            let r37 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928d56); };
            c.w64(11, t38);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928d5d); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 1) { return RegfileExit::fault(0x7000928d61); }
            c.w64(11, c.r64(8));
            blk = 44; continue;
        }
        28 => {
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928d6f); };
            c.w32(2, t39);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r40 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            c.cmp(c.r16(2), 0x41u64, 16);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 30; continue;
        }
        30 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t41) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928d8a); };
            c.w32(2, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928d8d); };
            c.cmp(t42, c.r32(2), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        33 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928d9d); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            let Some(t45) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928da3); };
            c.cmp(t45, 0x43u64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        35 => {
            c.w32(2, 0x1eu64);
            blk = 39; continue;
        }
        36 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928dbc); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000928dc2); };
            c.cmp(t48, 0x41u64, 16);
            if c.cc_e() { blk = 58; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, 0xfeu64);
            blk = 39; continue;
        }
        39 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r49 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r49);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928dde); };
            c.w32(6, t50);
            c.w32(2, c.r32(2));
            c.w32(11, c.r32(6));
            let r51 = c.r32(2) | c.r32(11);
            c.w32(2, r51);
            c.logic(r51, 32);
            c.cmp(c.r32(6), c.r32(2), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000928df2); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928df7); }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000928dff); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000928e09); };
            c.w64(0, t52);
            return c.call(0x70008ae1b4, 0x7000928e12, 0x7000928e0d);
        }
        43 => {
            c.w64(11, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928e15); };
            c.w64(15, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928e19); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928e1d); };
            c.w64(7, t55);
            blk = 44; continue;
        }
        44 => {
            let Some(t56) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928e21); };
            c.w64(7, t56);
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t57) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000928e2c); };
            c.w32(8, t57);
            let Some(t58) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928e31); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000928e35); };
            c.w64(5, t59);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(11));
            let r60 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r60);
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            c.w64(0, c.r64(11));
            c.cdq();
            c.idiv(c.r32(3), 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000928e4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000928e4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x7000928e53); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000928e57); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(11, c.r64(2));
            let r61 = c.add(c.r32(11), c.r32(2), 32);
            c.w32(11, r61);
            if c.cc_o() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 53; continue;
        }
        48 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000928e74); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000928e7b); };
            c.w64(0, t63);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000928e82); };
            c.w64(12, t64);
            let Some(t65) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000928e89); };
            c.cmp(t65, c.r64(3), 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000928e9c, 0x7000928e97);
        }
        50 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000928e9c); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928ea0); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928ea4); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928ea8); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000928eac); };
            c.w64(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000928eb0); };
            c.w64(1, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000928eb4); };
            c.w64(6, t72);
            blk = 52; continue;
        }
        51 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x7000928eba); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000928ecc); };
            c.w64(11, t73);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000928ed3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000928ed7); }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(0));
            blk = 9; continue;
        }
        54 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x460)) else { return RegfileExit::fault(0x7000928ee4); };
            c.w64(11, t74);
            blk = 9; continue;
        }
        55 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000928ef0); };
            c.w64(11, t75);
            blk = 9; continue;
        }
        56 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000928f01); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 1) { return RegfileExit::fault(0x7000928f06); }
            return c.call(0x70007febf4, 0x7000928f10, 0x7000928f0b);
        }
        57 => {
            c.w64(8, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928f13); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928f17); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928f1b); };
            c.w64(7, t78);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000928f1f); }
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000928f24); }
            blk = 25; continue;
        }
        58 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000928f33); };
            c.w32(2, t79);
            c.w32(6, c.r32(2));
            let r80 = c.r32(6) | 0x40u64;
            c.w32(6, r80);
            c.logic(r80, 32);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x7000928f44); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000928f49); }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000928f51); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000928f55); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000928f57); }
            c.w32(8, 0x8u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000928f5e); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000928f68); };
            c.w64(3, t81);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000928f77, 0x7000928f72);
        }
        62 => {
            c.w64(11, c.r64(0));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928f7a); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000928f7e); };
            c.w64(9, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000928f82); };
            c.w64(7, t84);
            blk = 44; continue;
        }
        63 => {
            return RegfileExit::jump(0x7000928f8c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000928f94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000928f94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000928f98); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000928f9e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000928fa2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000928fa6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000928fab); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000928fb2); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000928fb6); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000928fba); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000928fc1); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000928fc6); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000928fca); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000928fce); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000928fd2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000928fd6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000928fda); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000928fde); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000928fe7); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000928fee); };
            c.cmp(t14, c.r32(8), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000928ffb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000928fff); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929011); };
            c.w32(6, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929014); };
            c.cmp(t16, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 5; continue;
        }
        5 => {
            let r17 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929024); }
            c.xorpd_rr(3, 3);
            c.cvtsi2sd(3, (c.r32(11) as u32 as i32 as i64));
            blk = 17; continue;
        }
        6 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000929037); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700092903e); };
            c.w64(6, t19);
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            let r21 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r21);
            c.w64(12, c.r64(6).wrapping_add(0xc));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092904a); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000929051); };
            c.cmp(t23, c.r64(12), 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x700092905a); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929072); };
            c.w64(6, t24);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000929079); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700092907c); }
            c.w64(10, c.r64(4));
            let r25 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r25);
            let r26 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r26);
            c.logic(r26, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700092908b); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x7000929090); };
            c.w64(0, t27);
            c.w64(10, 0x700092909eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700092909e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009290a2); }
            return c.call(c.r64(0), 0x70009290a8, 0x70009290a6);
        }
        9 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009290a8); }
            let Some(t28) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009290b0); };
            c.w64(4, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009290b5); };
            c.w64(0, t29);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009290b9); }
            let Some(t30) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009290be); };
            c.cmp(t30, 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009290c8); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009290cc); };
            c.w64(7, t32);
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009290d0); };
            c.w32(11, t33);
            c.w32(3, c.r32(11));
            let r34 = c.r32(3) | 0xeu64;
            c.w32(3, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(11), c.r32(3), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x70009290e7); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x70009290ec); }
            blk = 39; continue;
        }
        12 => {
            let Some(t35) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009290f9); };
            c.w32(6, t35);
            let r36 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929100); };
            c.cmp(t37, c.r32(6), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929111); };
            c.w32(6, t38);
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929114); };
            c.cmp(t39, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929121); }
            if !c.movsd_rm(2, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929127); }
            blk = 17; continue;
        }
        16 => {
            let r40 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r40);
            if !c.movsd_rm(3, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929130); }
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(0) as u32 as i32 as i64));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000929143); };
            c.w32(11, t41);
            c.w32(0, c.r32(11));
            let r42 = c.r32(0) | 0xeu64;
            c.w32(0, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000929156); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092915b); }
            blk = 20; continue;
        }
        20 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000929163); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092916a); };
            c.w64(11, t44);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000929171); };
            c.w64(3, t45);
            let Some(t46) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000929178); };
            c.cmp(t46, c.r64(0), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000929181); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 22; continue;
        }
        22 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929188); };
            c.w64(11, t47);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700092918f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000929193); }
            c.mov128_rr(0, 3);
            c.mov128_rr(1, 2);
            c.w64(10, c.r64(4));
            let r48 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r48);
            let r49 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r49);
            c.logic(r49, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70009291a8); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1360)) else { return RegfileExit::fault(0x70009291ad); };
            c.w64(0, t50);
            c.w64(10, 0x70009291bbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70009291bb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70009291bf); }
            return c.call(c.r64(0), 0x70009291c5, 0x70009291c3);
        }
        23 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70009291c5); }
            let Some(t51) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70009291cd); };
            c.w64(4, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70009291d2); };
            c.w64(0, t52);
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009291d6); }
            blk = 39; continue;
        }
        24 => {
            let Some(t53) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009291e0); };
            c.w32(6, t53);
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r54 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r54);
            c.cmp(c.r16(6), 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r16(6), 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009291fb); };
            c.w32(6, t55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009291fe); };
            c.cmp(t56, c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        29 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092920e); };
            c.w32(6, t57);
            let r58 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r58);
            let Some(t59) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929214); };
            c.cmp(t59, 0x43u64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        31 => {
            c.w32(6, 0x1eu64);
            blk = 35; continue;
        }
        32 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092922d); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            let Some(t62) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929233); };
            c.cmp(t62, 0x41u64, 16);
            if c.cc_e() { blk = 44; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(6, 0xfeu64);
            blk = 35; continue;
        }
        35 => {
            c.w64(12, c.r64(9).wrapping_sub(0x1));
            let r63 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r63);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092924f); };
            c.w32(1, t64);
            c.w32(6, c.r32(6));
            c.w32(11, c.r32(1));
            let r65 = c.r32(6) | c.r32(11);
            c.w32(6, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x7000929263); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000929268); }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000929270); }
            c.w64(6, c.r64(3));
            c.w64(3, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092927a); };
            c.w64(0, t66);
            return c.call(0x70008ae454, 0x7000929283, 0x700092927e);
        }
        39 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929283); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929287); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092928b); };
            c.w64(8, t69);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t70) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000929293); };
            c.w32(8, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929298); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092929c); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70009292a0); };
            c.w64(5, t73);
            c.w64(12, c.r64(7));
            let r74 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r74);
            return RegfileExit::jump(c.r64(1));
        }
        40 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009292b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70009292b6); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70009292c2, 0x70009292bd);
        }
        41 => {
            c.w64(7, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009292c5); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70009292c9); };
            c.w64(0, t76);
            blk = 8; continue;
        }
        42 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 2) { return RegfileExit::fault(0x70009292d7); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 3) { return RegfileExit::fault(0x70009292dc); }
            return c.call(0x70007febf4, 0x70009292e6, 0x70009292e1);
        }
        43 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70009292e9); }
            if !c.movsd_rm(3, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70009292ee); }
            blk = 22; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t77) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70009292fd); };
            c.w32(1, t77);
            c.w32(6, c.r32(1));
            let r78 = c.r32(6) | 0x40u64;
            c.w32(6, r78);
            c.logic(r78, 32);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(1, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(1)) { return RegfileExit::fault(0x700092930e); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000929313); }
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700092931b); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092931f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000929321); }
            c.w32(8, 0xau64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000929328); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000929332); };
            c.w64(3, t79);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000929341, 0x700092933c);
        }
        48 => {
            blk = 39; continue;
        }
        49 => {
            return RegfileExit::jump(0x7000929348);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000929354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000929354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000929358); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092935e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929362); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000929366); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092936b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000929372); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929376); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092937a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000929381); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929386); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092938a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092938e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000929392); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000929396); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092939a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700092939e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009293a2); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009293a6); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009293af); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009293bc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70009293c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70009293c6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70009293cc); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009293d9); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009293e1); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009293ee); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000929401); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929410, 0x700092940b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929419); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092941d); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929421); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929425); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929429); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700092942d); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929431); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929437); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092944a); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092945d); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000929461); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092946d); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929474); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092947d); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929493); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000929497); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009294a3, 0x700092949e);
        }
        17 => {
            c.w32(11, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009294a9); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009294ad); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009294b1); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009294b5); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009294b9); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009294bd); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009294c1); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009294c5); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009294cb); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(11, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(11, 0x40u64);
            blk = 22; continue;
        }
        20 => {
            c.w32(11, 0xeu64);
            blk = 22; continue;
        }
        21 => {
            c.w32(11, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(11));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929501); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092951a); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000929525); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000929529); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929536); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092953c); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929549); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929559); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929565, 0x7000929560);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092956a); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092956e); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929572); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929578); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009295a0); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009295a7); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009295b6); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009295ba); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009295bc); }
            c.w32(11, 0xeu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009295c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009295c5); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70009295d1); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70009295e0, 0x70009295db);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009295e0); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009295e6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929606); };
            let r68 = c.r32(0) | t67;
            c.w32(0, r68);
            c.logic(r68, 32);
            c.w64(11, c.r64(0));
            let r69 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r69);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(11, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(11, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092962d); };
            c.w32(3, t70);
            c.w32(11, c.r32(11));
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.r32(8) | c.r32(11);
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092963c); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000929640); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929649); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092964d); };
            c.w32(3, t76);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r77 = c.r32(8) | c.r32(2);
            c.w32(8, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000929662); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000929667); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092966e); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929672); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929676); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092967e); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929683); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929687); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092968b); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000929698); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700092969f); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70009296a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70009296aa); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70009296ae); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70009296b5); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70009296c4, 0x70009296bf);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009296c7); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009296cb); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009296cf); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70009296d5); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009296dc); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70009296e3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70009296ef); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000929701); }
            return c.call(0x70008708b4, 0x700092970b, 0x7000929706);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092970b); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092971b); }
            return c.call(0x70008708b4, 0x7000929725, 0x7000929720);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000929725); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000929734);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000929754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000929754); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000929758); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092975e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929762); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000929766); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092976b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000929772); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929776); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092977a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000929781); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929786); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092978a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092978e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000929792); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000929796); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092979a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700092979e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70009297a2); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70009297a6); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70009297af); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70009297bc); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70009297c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x70009297c6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70009297cc); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70009297d9); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70009297e1); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70009297ee); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000929801); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929810, 0x700092980b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929819); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092981d); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929821); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929825); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929829); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700092982d); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929831); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929837); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092984a); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092985d); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000929861); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092986d); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929874); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092987d); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929893); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000929897); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x70009298a3, 0x700092989e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70009298a8); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70009298ac); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70009298b0); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70009298b4); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70009298b8); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70009298bc); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009298c0); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009298c4); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70009298ca); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70009298ff); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929918); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000929923); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000929927); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929934); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092993a); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929947); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929957); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929963, 0x700092995e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929968); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092996c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929970); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929976); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092999e); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70009299a5); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70009299b4); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009299b8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70009299ba); }
            c.w32(11, 0x10u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70009299c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70009299c3); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x70009299cf); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x70009299de, 0x70009299d9);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70009299de); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70009299e4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929a04); };
            let r68 = c.r32(0) ^ t67;
            c.w32(0, r68);
            c.logic(r68, 32);
            c.w64(11, c.r64(0));
            let r69 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r69);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000929a29); };
            c.w32(11, t70);
            c.w32(3, c.r32(6));
            let r71 = c.r32(8) | c.r32(11);
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.r32(8) | c.r32(3);
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000929a38); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000929a3c); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929a45); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000929a49); };
            c.w32(3, t76);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r77 = c.r32(8) | c.r32(2);
            c.w32(8, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000929a5e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000929a63); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929a6a); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929a6e); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929a72); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000929a7a); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929a7f); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929a83); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000929a87); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000929a94); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000929a9b); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000929aa2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000929aa6); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000929aaa); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000929ab1); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000929ac0, 0x7000929abb);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929ac3); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929ac7); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000929acb); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000929ad1); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929ad8); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000929adf); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000929aeb); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000929afd); }
            return c.call(0x70008708b4, 0x7000929b07, 0x7000929b02);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000929b07); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000929b17); }
            return c.call(0x70008708b4, 0x7000929b21, 0x7000929b1c);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000929b21); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000929b30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000929b34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000929b34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000929b38); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000929b3e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929b42); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000929b46); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000929b4b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000929b52); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929b56); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000929b5a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000929b61); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929b66); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000929b6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000929b6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000929b72); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000929b76); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000929b7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000929b7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000929b82); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000929b86); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000929b8f); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000929b9c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000929ba2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000929ba6); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000929bac); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929bb9); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929bc1); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929bce); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000929be1); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929bf0, 0x7000929beb);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929bf9); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929bfd); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929c01); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929c05); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929c09); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000929c0d); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929c11); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929c17); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929c2a); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929c3d); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000929c41); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929c4d); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929c54); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929c5d); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929c73); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000929c77); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929c83, 0x7000929c7e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929c88); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929c8c); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929c90); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929c94); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929c98); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929c9c); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000929ca0); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000929ca4); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929caa); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929cdf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929cf8); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000929d03); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000929d07); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929d14); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929d1a); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929d27); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000929d37); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929d43, 0x7000929d3e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929d48); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929d4c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929d50); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929d56); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000929d7e); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000929d85); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000929d94); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000929d98); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000929d9a); }
            c.w32(11, 0xcu64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000929da1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000929da3); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x7000929daf); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x7000929dbe, 0x7000929db9);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000929dbe); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000929dc4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000929de4); };
            let r68 = c.r32(0) & t67;
            c.w32(0, r68);
            c.logic(r68, 32);
            c.w64(11, c.r64(0));
            let r69 = c.add(c.r32(11), c.r32(0), 32);
            c.w32(11, r69);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000929e09); };
            c.w32(11, t70);
            c.w32(3, c.r32(6));
            let r71 = c.r32(8) | c.r32(11);
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.r32(8) | c.r32(3);
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000929e18); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000929e1c); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929e25); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000929e29); };
            c.w32(3, t76);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r77 = c.r32(8) | c.r32(2);
            c.w32(8, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x7000929e3e); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x7000929e43); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929e4a); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929e4e); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929e52); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000929e5a); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929e5f); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929e63); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000929e67); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000929e74); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000929e7b); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000929e82); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000929e86); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000929e8a); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000929e91); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000929ea0, 0x7000929e9b);
        }
        51 => {
            c.w64(11, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929ea3); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929ea7); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000929eab); };
            c.w64(0, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x7000929eb1); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929eb8); };
            c.w64(3, t93);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000929ebf); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000929ecb); }
            c.w64(0, c.r64(11));
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000929edd); }
            return c.call(0x70008708b4, 0x7000929ee7, 0x7000929ee2);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000929ee7); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000929ef7); }
            return c.call(0x70008708b4, 0x7000929f01, 0x7000929efc);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000929f01); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x7000929f10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000929f14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000929f14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000929f18); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000929f1e); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929f22); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x7000929f26); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000929f2b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x7000929f32); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x7000929f36); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000929f3a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000929f41); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929f46); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x7000929f4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x7000929f4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x7000929f52); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x7000929f56); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x7000929f5a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000929f5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x7000929f62); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x7000929f66); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000929f6f); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000929f7c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000929f82); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000929f86); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000929f8c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000929f99); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000929fa1); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000929fae); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x7000929fc1); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x7000929fd0, 0x7000929fcb);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000929fd9); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000929fdd); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000929fe1); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000929fe5); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000929fe9); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000929fed); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000929ff1); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000929ff7); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a00a); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a01d); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700092a021); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a02d); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a034); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a03d); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a053); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700092a057); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a063, 0x700092a05e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a068); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a06c); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a070); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a074); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a078); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a07c); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a080); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a084); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a08a); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a0bf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a0d8); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a0e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700092a0e7); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a0f4); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a0fa); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a107); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a117); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a123, 0x700092a11e);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a128); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a12c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a130); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a136); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a15e); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092a165); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092a174); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a178); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092a17a); }
            c.w32(11, 0x12u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a181); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a183); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092a18f); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700092a19e, 0x700092a199);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a19e); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a1a4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a1c7); };
            c.w64(11, t67);
            let r68 = c.shl(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r68);
            c.w64(0, c.r64(11));
            let r69 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r69);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a1f0); };
            c.w32(11, t70);
            c.w32(3, c.r32(6));
            let r71 = c.r32(8) | c.r32(11);
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.r32(8) | c.r32(3);
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092a1ff); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092a203); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a20c); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092a210); };
            c.w32(3, t76);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r77 = c.r32(8) | c.r32(2);
            c.w32(8, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092a225); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092a22a); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a231); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a235); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a239); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092a241); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a246); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092a24a); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092a24e); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092a25b); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092a262); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700092a269); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700092a26d); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092a271); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700092a278); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092a287, 0x700092a282);
        }
        51 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a287); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a28b); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a28f); };
            c.w64(8, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700092a295); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a29c); };
            c.w64(3, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700092a2a3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092a2af); }
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092a2bd); }
            return c.call(0x70008708b4, 0x700092a2c7, 0x700092a2c2);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092a2c7); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092a2d7); }
            return c.call(0x70008708b4, 0x700092a2e1, 0x700092a2dc);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092a2e1); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x700092a2f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092a2f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092a2f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092a2f8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092a2fe); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a302); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092a306); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092a30b); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092a312); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092a316); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092a31a); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092a321); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a326); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092a32a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092a32e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700092a332); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092a336); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092a33a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700092a33e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700092a342); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092a346); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092a34f); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092a35c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700092a362); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700092a366); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700092a36c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a379); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a381); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a38e); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092a3a1); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a3b0, 0x700092a3ab);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a3b9); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a3bd); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a3c1); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a3c5); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a3c9); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700092a3cd); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a3d1); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a3d7); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a3ea); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a3fd); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700092a401); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a40d); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a414); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a41d); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a433); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700092a437); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a443, 0x700092a43e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a448); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a44c); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a450); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a454); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a458); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a45c); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a460); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a464); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a46a); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a49f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a4b8); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a4c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700092a4c7); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a4d4); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a4da); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a4e7); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a4f7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a503, 0x700092a4fe);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a508); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a50c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a510); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a516); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a53e); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092a545); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092a554); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a558); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092a55a); }
            c.w32(11, 0x14u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a561); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a563); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092a56f); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700092a57e, 0x700092a579);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a57e); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a584); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a5a7); };
            c.w64(11, t67);
            let r68 = c.sar(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r68);
            c.w64(0, c.r64(11));
            let r69 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r69);
            if c.cc_o() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t70) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a5d0); };
            c.w32(11, t70);
            c.w32(3, c.r32(6));
            let r71 = c.r32(8) | c.r32(11);
            c.w32(8, r71);
            c.logic(r71, 32);
            let r72 = c.r32(8) | c.r32(3);
            c.w32(8, r72);
            c.logic(r72, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092a5df); };
            c.w64(7, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092a5e3); };
            c.cmp(t74, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a5ec); };
            c.w64(11, t75);
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092a5f0); };
            c.w32(3, t76);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r77 = c.r32(8) | c.r32(2);
            c.w32(8, r77);
            c.logic(r77, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092a605); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092a60a); }
            blk = 48; continue;
        }
        48 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a611); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a615); };
            c.w64(7, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a619); };
            c.w64(8, t80);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t81) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092a621); };
            c.w32(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a626); };
            c.w64(15, t82);
            let Some(t83) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092a62a); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092a62e); };
            c.w64(5, t84);
            c.w64(12, c.r64(7));
            let r85 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r85);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092a63b); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092a642); };
            c.w64(3, t87);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700092a649); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700092a64d); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092a651); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700092a658); };
            c.cmp(t89, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092a667, 0x700092a662);
        }
        51 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a667); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a66b); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a66f); };
            c.w64(8, t92);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700092a675); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a67c); };
            c.w64(3, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700092a683); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092a68f); }
            blk = 41; continue;
        }
        54 => {
            let r94 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r94);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092a69d); }
            return c.call(0x70008708b4, 0x700092a6a7, 0x700092a6a2);
        }
        55 => {
            let Some(t95) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092a6a7); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            blk = 38; continue;
        }
        56 => {
            let r97 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r97);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092a6b7); }
            return c.call(0x70008708b4, 0x700092a6c1, 0x700092a6bc);
        }
        57 => {
            let Some(t98) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092a6c1); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r99);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x700092a6d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092a6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092a6d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092a6d8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092a6de); };
            c.w64(7, t1);
            let Some(t2) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a6e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092a6e6); };
            c.w64(11, (t3 as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092a6eb); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092a6f2); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092a6f6); };
            c.w64(11, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092a6fa); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092a701); };
            c.w32(2, t10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a706); };
            c.w32(1, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092a70a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092a70e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700092a712); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092a716); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700092a71a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700092a71e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700092a722); }
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092a726); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092a72f); };
            c.w64(8, t13);
            blk = 2; continue;
        }
        2 => {
            c.w64(1, c.r64(9).wrapping_sub(0x1));
            let r14 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700092a73c); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700092a742); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x700092a746); }
            blk = 4; continue;
        }
        3 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700092a74c); }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, c.r64(11));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a759); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a761); };
            c.cmp(t17, c.r32(8), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a76e); };
            c.w32(8, t18);
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092a781); }
            c.w64(0, c.r64(12));
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a790, 0x700092a78b);
        }
        9 => {
            c.w32(8, 0xfeu64);
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a799); };
            c.w64(15, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a79d); };
            c.w64(9, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a7a1); };
            c.w64(7, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a7a5); };
            c.w64(2, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a7a9); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700092a7ad); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a7b1); };
            c.w64(1, t25);
            blk = 3; continue;
        }
        10 => {
            let Some(t26) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a7b7); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(8, 0x1eu64);
            c.w64(11, c.r64(12));
            blk = 3; continue;
        }
        11 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a7ca); };
            c.w32(8, t28);
            let r29 = c.r32(8) | 0x2u64;
            c.w32(8, r29);
            c.logic(r29, 32);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        12 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a7dd); };
            c.w32(8, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700092a7e1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a7ed); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a7f4); };
            c.cmp(t34, c.r32(11), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a7fd); };
            c.w32(11, t35);
            c.cmp(c.r16(11), 0x41u64, 16);
            if c.cc_e() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(11), 0x43u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a813); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700092a817); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a823, 0x700092a81e);
        }
        17 => {
            c.w32(6, 0xfeu64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a828); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092a82c); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a830); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a834); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a838); };
            c.w64(2, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a83c); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a840); };
            c.w64(12, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a844); };
            c.w64(8, t43);
            blk = 22; continue;
        }
        18 => {
            let Some(t44) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a84a); };
            c.w32(0, t44);
            let r45 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r45);
            c.w32(6, 0x1eu64);
            blk = 22; continue;
        }
        19 => {
            c.w32(6, 0x40u64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        20 => {
            c.w32(6, 0xeu64);
            c.w64(11, c.r64(6));
            blk = 22; continue;
        }
        21 => {
            c.w32(6, 0x2u64);
            blk = 22; continue;
        }
        22 => {
            let r46 = c.r32(8) | 0x40u64;
            c.w32(8, r46);
            c.logic(r46, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(12));
            blk = 35; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a87f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_o() { blk = 56; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w32(12, c.r32(12));
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a898); };
            c.w32(8, t47);
            let r48 = c.r32(8) | 0xeu64;
            c.w32(8, r48);
            c.logic(r48, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a8a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700092a8a7); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092a8b4); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092a8ba); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092a8c7); };
            c.w32(1, t52);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700092a8d7); }
            c.w64(6, c.r64(3));
            return c.call(0x70008a10d4, 0x700092a8e3, 0x700092a8de);
        }
        31 => {
            c.w32(6, 0xfeu64);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700092a8e8); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a8ec); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092a8f0); };
            c.w64(3, t55);
            blk = 26; continue;
        }
        32 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700092a8f6); };
            c.w32(0, t56);
            let r57 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r57);
            c.w32(6, 0x1eu64);
            blk = 26; continue;
        }
        33 => {
            c.w32(8, c.r32(6));
            let r58 = c.r32(8) | 0x2u64;
            c.w32(8, r58);
            c.logic(r58, 32);
            let r59 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        34 => {
            c.w32(8, c.r32(6));
            let r60 = c.r32(8) | 0x40u64;
            c.w32(8, r60);
            c.logic(r60, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(8));
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a91e); };
            c.w64(8, t61);
            c.w64(1, c.r64(11));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092a925); };
            c.w64(11, t62);
            blk = 35; continue;
        }
        35 => {
            c.w32(6, c.r32(6));
            c.w32(8, c.r32(8));
            let r63 = c.r32(6) | c.r32(8);
            c.w32(6, r63);
            c.logic(r63, 32);
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700092a934); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a938); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700092a93a); }
            c.w32(11, 0x16u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700092a941); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092a943); }
            c.w64(6, c.r64(3));
            c.w32(0, 0x3u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x1d20)) else { return RegfileExit::fault(0x700092a94f); };
            c.w64(3, t64);
            c.w64(11, c.r64(3));
            return c.call(0x700086ed74, 0x700092a95e, 0x700092a959);
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a95e); };
            c.w64(8, t65);
            blk = 45; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092a964); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 54; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(0, c.r32(0));
            c.w32(8, c.r32(6));
            let r66 = c.r32(8) | 0xeu64;
            c.w32(8, r66);
            c.logic(r66, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 39; continue;
        }
        39 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092a987); };
            c.w64(11, t67);
            let r68 = c.shr(c.r32(11), c.r8(1), 32).unwrap_or(c.r32(11));
            c.w32(11, r68);
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(6, 0xeu64);
            blk = 44; continue;
        }
        43 => {
            c.w32(6, 0x2u64);
            blk = 44; continue;
        }
        44 => {
            c.w32(8, c.r32(8));
            let Some(t69) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092a9b2); };
            c.w32(11, t69);
            c.w32(3, c.r32(6));
            let r70 = c.r32(8) | c.r32(11);
            c.w32(8, r70);
            c.logic(r70, 32);
            let r71 = c.r32(8) | c.r32(3);
            c.w32(8, r71);
            c.logic(r71, 32);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 45; continue;
        }
        45 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092a9c1); };
            c.w64(7, t72);
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092a9c5); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092a9ce); };
            c.w64(11, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092a9d2); };
            c.w32(3, t75);
            c.w32(8, c.r32(8));
            c.w32(2, c.r32(3));
            let r76 = c.r32(8) | c.r32(2);
            c.w32(8, r76);
            c.logic(r76, 32);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x700092a9e7); }
            if !c.st32(c.r64(7).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092a9ec); }
            blk = 48; continue;
        }
        48 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a9f3); };
            c.w64(7, t77);
            let Some(t78) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092a9f7); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092a9fb); };
            c.w64(8, t79);
            c.w64(9, c.r64(8).wrapping_add(0x9));
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092aa03); };
            c.w32(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092aa08); };
            c.w64(15, t81);
            let Some(t82) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700092aa0c); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700092aa10); };
            c.w64(5, t83);
            c.w64(12, c.r64(7));
            let r84 = c.add(c.r64(4), 0x68u64, 64);
            c.w64(4, r84);
            return RegfileExit::jump(c.r64(1));
        }
        49 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092aa1d); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700092aa24); };
            c.w64(3, t86);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700092aa2b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x700092aa2f); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092aa33); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700092aa3a); };
            c.cmp(t88, c.r64(2), 64);
            if c.cc_a() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092aa49, 0x700092aa44);
        }
        51 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092aa49); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092aa4d); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092aa51); };
            c.w64(8, t91);
            blk = 53; continue;
        }
        52 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700092aa57); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 53; continue;
        }
        53 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092aa5e); };
            c.w64(3, t92);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700092aa65); }
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700092aa74); }
            blk = 41; continue;
        }
        54 => {
            let r93 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r93);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092aa82); }
            return c.call(0x70008708b4, 0x700092aa8c, 0x700092aa87);
        }
        55 => {
            let Some(t94) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092aa8c); };
            c.w32(0, t94);
            let r95 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r95);
            blk = 38; continue;
        }
        56 => {
            let r96 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r96);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700092aa9c); }
            return c.call(0x70008708b4, 0x700092aaa6, 0x700092aaa1);
        }
        57 => {
            let Some(t97) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700092aaa6); };
            c.w32(12, t97);
            let r98 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r98);
            blk = 24; continue;
        }
        58 => {
            return RegfileExit::jump(0x700092aab4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700092aad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092aad7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700092aadb); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700092aae2); };
            c.w64(11, t3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700092aae6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x700092aaed); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x5)) else { return RegfileExit::fault(0x700092aaf2); };
            c.w32(2, t7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092aaf7); };
            c.w32(1, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700092aafb); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ab04); };
            c.w64(8, t10);
            blk = 2; continue;
        }
        2 => {
            let r11 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r11);
            c.w32(1, c.r32(0));
            c.w32(6, c.r32(3));
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700092ab11); };
            c.cmp(t12, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            let r13 = c.add(c.r32(6), c.r32(1), 32);
            c.w32(6, r13);
            if c.cc_o() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ab2e); };
            c.w32(11, t14);
            c.w32(0, c.r32(11));
            let r15 = c.r32(0) | 0x2u64;
            c.w32(0, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092ab41); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ab46); }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 8; continue;
        }
        8 => {
            let Some(t16) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ab51); };
            c.w64(7, t16);
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x9));
            let Some(t17) = c.ld8(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700092ab5c); };
            c.w32(11, t17);
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x700092ab61); };
            c.w64(1, t18);
            c.w64(12, c.r64(7));
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092ab6d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092ab71); }
            let r19 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r19);
            let r20 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r20);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092ab81); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092ab85); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092ab89); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x700092ab8d); }
            blk = 12; continue;
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700092ab93); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700092ab97); }
            let r21 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r21);
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ab9d); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x700092aba3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700092aba7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700092abab); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700092abaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x700092abb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700092abb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700092abbb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700092abbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x700092abc3); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092abc7); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700092abd4); }
            blk = 12; continue;
        }
        12 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700092abd9); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092abe4); };
            c.w32(11, t25);
            c.w32(0, c.r32(11));
            let r26 = c.r32(0) | 0xeu64;
            c.w32(0, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092abf7); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092abfc); }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700092ac04); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700092ac0b); };
            c.w64(11, t28);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700092ac12); };
            c.w64(2, t29);
            let Some(t30) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700092ac19); };
            c.cmp(t30, c.r64(0), 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700092ac28, 0x700092ac23);
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700092ac28); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ac2c); };
            c.w64(15, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092ac30); };
            c.w64(9, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ac34); };
            c.w64(7, t34);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700092ac38); }
            blk = 19; continue;
        }
        18 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700092ac3f); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            let r35 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r35);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.arith_sd(0, 1, c.xmm_u64(0));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092ac57); };
            c.w64(8, t36);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700092ac5e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 1) { return RegfileExit::fault(0x700092ac62); }
            c.w64(8, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092ac6d); }
            blk = 8; continue;
        }
        20 => {
            c.w64(6, c.r64(9).wrapping_sub(0x1));
            let Some(t37) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ac77); };
            c.w32(1, t37);
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r38 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r38);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700092ac86); }
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 42; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t39) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092acb1); };
            c.w32(0, t39);
            let r40 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r40);
            let Some(t41) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092acb7); };
            c.cmp(t41, 0x41u64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092acc2); }
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092acc6); };
            c.w64(0, t42);
            c.w64(6, c.r64(11));
            return c.call(0x700089b334, 0x700092acd2, 0x700092accd);
        }
        26 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t43) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092acd6); };
            c.cmp(t43, 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092acdc); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092ace0); };
            c.w64(2, t45);
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ace4); };
            c.w32(7, t46);
            c.w32(11, c.r32(7));
            let r47 = c.r32(11) | 0x40u64;
            c.w32(11, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(7)) { return RegfileExit::fault(0x700092acf8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092acfd); }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, c.r64(0));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ad08); };
            c.w64(15, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092ad0c); };
            c.w64(9, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ad10); };
            c.w64(7, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092ad17); }
            blk = 8; continue;
        }
        31 => {
            let Some(t51) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700092ad1d); };
            c.cmp(t51, 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700092ad23); };
            c.w64(8, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700092ad27); };
            c.w64(7, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ad2b); };
            c.w32(11, t54);
            c.w32(0, c.r32(11));
            let r55 = c.r32(0) | 0xfeu64;
            c.w32(0, r55);
            c.logic(r55, 32);
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x1f), c.r32(11)) { return RegfileExit::fault(0x700092ad40); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ad45); }
            blk = 34; continue;
        }
        34 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ad4d); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700092ad51); };
            c.w64(2, t57);
            if !c.st64(c.r64(11).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700092ad55); }
            c.w32(7, 0x180u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700092ad5e); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x700092ad5f); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700092ad6b); };
            c.w64(6, t59);
            return c.call(0x700086ed74, 0x700092ad74, 0x700092ad6f);
        }
        35 => {
            return RegfileExit::fault(0x700092ad74);
        }
        36 => {
            let Some(t60) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092ad75); };
            c.w32(0, t60);
            let r61 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r61);
            let Some(t62) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ad7b); };
            c.cmp(t62, 0x40u64, 16);
            if c.cc_ae() { blk = 45; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092ad88); };
            c.w32(0, t63);
            c.w32(1, c.r32(0));
            let r64 = c.r32(1) | 0x20u64;
            c.w32(1, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092ad99); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ad9e); }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092ada6); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092adaa); };
            c.w64(0, t65);
            c.w64(6, c.r64(11));
            return c.call(0x700086f9d4, 0x700092adb6, 0x700092adb1);
        }
        41 => {
            c.w64(8, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092adb9); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092adbd); };
            c.w64(9, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092adc1); };
            c.w64(7, t68);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092adc8); }
            blk = 8; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t69) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700092add3); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700092add9); };
            c.cmp(t71, c.r32(0), 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t72) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700092ade2); };
            c.cmp(t72, 0x43u64, 16);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(0, 0xfeu64);
            blk = 47; continue;
        }
        46 => {
            c.w32(0, 0x1eu64);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700092adfb); };
            c.w32(1, t73);
            c.w32(0, c.r32(0));
            c.w32(12, c.r32(1));
            let r74 = c.r32(0) | c.r32(12);
            c.w32(0, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f), c.r32(0)) { return RegfileExit::fault(0x700092ae0f); }
            if !c.st32(c.r64(8).wrapping_add(0xb), 0x0u64) { return RegfileExit::fault(0x700092ae14); }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(7).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700092ae1c); }
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700092ae20); };
            c.w64(0, t75);
            c.w64(6, c.r64(11));
            return c.call(0x70008ad794, 0x700092ae2c, 0x700092ae27);
        }
        51 => {
            c.w64(8, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700092ae2f); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700092ae33); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700092ae37); };
            c.w64(7, t78);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700092ae3e); }
            blk = 8; continue;
        }
        52 => {
            return RegfileExit::jump(0x700092ae48);
        }
            _ => unreachable!(),
        }
    }
}
