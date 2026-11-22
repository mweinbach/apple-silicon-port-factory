#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700089e014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089e014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089e018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089e019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089e01a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089e01f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700089e029); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e035); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e03b); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 63; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089e046); };
            c.w64(1, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700089e04a); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x700089e05b, 0x700089e056);
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e05e); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e06e); };
            c.w32(7, t8);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e07d); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e084); };
            c.w32(8, t11);
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(0));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(6));
            blk = 10; continue;
        }
        10 => {
            c.w64(12, c.r64(11));
            let r12 = c.r32(12) & 0x7u64;
            c.w32(12, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 70; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 70; continue;
        }
        18 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089e0d7); };
            c.w32(11, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089e0db); };
            c.cmp(t14, c.r32(11), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e0e8); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            let Some(t17) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e0ef); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            let Some(t19) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e0f6); };
            c.w32(11, t19);
            blk = 10; continue;
        }
        20 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089e0fd); };
            c.w32(11, t20);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r21 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r21);
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e10a); };
            c.w32(9, t22);
            let r23 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r23);
            let Some(t24) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e111); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e118); };
            c.w32(11, t26);
            blk = 10; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e122); };
            c.w32(9, t27);
            let r28 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r28);
            let Some(t29) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e129); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e130); };
            c.w32(11, t31);
            blk = 10; continue;
        }
        22 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        23 => {
            c.w32(12, 0x1u64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 25; continue;
        }
        24 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(6));
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 26; continue;
        }
        26 => {
            let r32 = c.add(c.r64(11), 0xbu64, 64);
            c.w64(11, r32);
            blk = 27; continue;
        }
        27 => {
            let r33 = c.add(c.r64(9), c.r64(1), 64);
            c.w64(9, r33);
            let r34 = c.r32(8) & 0x8u64;
            c.w32(8, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x700089e191); };
            c.w32(8, t35);
            blk = 30; continue;
        }
        29 => {
            let Some(t36) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700089e198); };
            c.w32(8, t36);
            blk = 30; continue;
        }
        30 => {
            c.w64(9, c.r64(8));
            let r37 = c.r32(9) & 0xfc00u64;
            c.w32(9, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(9), 0xd800u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 31; continue;
        }
        31 => {
            let r38 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r38);
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ge() { blk = 56; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e1c1); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e1c7); };
            c.w32(7, t41);
            c.logic(c.r8(7) & 0x7u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(6));
            c.w64(9, c.r64(0));
            blk = 34; continue;
        }
        34 => {
            c.w64(12, c.r64(11));
            let r42 = c.r32(12) & 0x7u64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_l() { blk = 39; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 38; continue;
        }
        38 => {
            blk = 87; continue;
        }
        39 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 87; continue;
        }
        42 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089e217); };
            c.w32(11, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089e21b); };
            c.cmp(t44, c.r32(11), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e228); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e22f); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            let Some(t49) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e236); };
            c.w32(11, t49);
            blk = 34; continue;
        }
        44 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089e23d); };
            c.w32(11, t50);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r51 = c.add(c.r64(7), c.r64(11), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e24a); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e251); };
            c.w32(11, t54);
            let r55 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r55);
            let Some(t56) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e258); };
            c.w32(11, t56);
            blk = 34; continue;
        }
        45 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e262); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            let Some(t59) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e269); };
            c.w32(11, t59);
            let r60 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r60);
            let Some(t61) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e270); };
            c.w32(11, t61);
            blk = 34; continue;
        }
        46 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        47 => {
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            c.w32(12, 0x1u64);
            blk = 49; continue;
        }
        48 => {
            c.w64(9, c.r64(6));
            c.w64(11, c.r64(0));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 50; continue;
        }
        50 => {
            let r62 = c.add(c.r64(11), 0xbu64, 64);
            c.w64(11, r62);
            blk = 51; continue;
        }
        51 => {
            let r63 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r63);
            let r64 = c.r32(7) & 0x8u64;
            c.w32(7, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t65) = c.ld16(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2))) else { return RegfileExit::fault(0x700089e2cd); };
            c.w32(1, t65);
            blk = 54; continue;
        }
        53 => {
            let Some(t66) = c.ld8(c.r64(1).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700089e2d4); };
            c.w32(1, t66);
            blk = 54; continue;
        }
        54 => {
            c.w64(7, c.r64(1));
            let r67 = c.r32(7) & 0xfc00u64;
            c.w32(7, r67);
            c.logic(r67, 32);
            c.cmp(c.r32(7), 0xdc00u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let r68 = c.shl(c.r32(8), 0xau64, 32).unwrap_or(c.r32(8));
            c.w32(8, r68);
            c.w32(8, c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x35fdc00));
            blk = 56; continue;
        }
        56 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 57; continue;
        }
        57 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089e2fa); };
            c.w64(1, t69);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089e301); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return c.ret(0x10, 0x700089e308);
        }
        59 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089e30b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089e312); }
            return c.ret(0x0, 0x700089e314);
        }
        60 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e315); };
            c.w64(0, t70);
            blk = 57; continue;
        }
        61 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089e31e); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089e324); };
            c.w64(3, t71);
            return c.call(0x700086ed74, 0x700089e330, 0x700089e32b);
        }
        62 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e330); };
            c.w64(6, t72);
            blk = 1; continue;
        }
        63 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089e339); };
            c.cmp(t73, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e346); };
            c.cmp(t74, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089e355); }
            let r75 = c.r32(1) | c.r32(7);
            c.w32(1, r75);
            c.logic(r75, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            return c.call(0x700089c794, 0x700089e362, 0x700089e35d);
        }
        65 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e362); };
            c.w64(6, t76);
            blk = 3; continue;
        }
        66 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089e370); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089e371); };
            c.w64(1, t77);
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(0x197)) else { return RegfileExit::fault(0x700089e378); };
            c.w32(1, t78);
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089e381); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089e382); };
            c.w64(3, t80);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089e393, 0x700089e38e);
        }
        67 => {
            return RegfileExit::fault(0x700089e393);
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700089e394); }
            return c.call(0x70008a1214, 0x700089e39d, 0x700089e398);
        }
        69 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e39d); };
            c.w64(6, t81);
            blk = 3; continue;
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e3a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700089e3aa); }
            blk = 74; continue;
        }
        71 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e3b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700089e3b4); }
            blk = 74; continue;
        }
        72 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e3ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700089e3be); }
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t82) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089e3c8); };
            c.w64(11, t82);
            blk = 27; continue;
        }
        74 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(8, c.r64(1).wrapping_add(c.r64(1)));
            blk = 85; continue;
        }
        76 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089e3e0); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089e3e7); };
            c.w64(9, t84);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089e3ee); };
            c.w64(12, t85);
            let Some(t86) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700089e3f5); };
            c.cmp(t86, c.r64(11), 64);
            if c.cc_a() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089e405, 0x700089e400);
        }
        78 => {
            c.w64(8, c.r64(0));
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e408); };
            c.w64(0, t87);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e40e); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089e412); };
            c.w64(7, t89);
            blk = 80; continue;
        }
        79 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700089e418); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 84; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            let r90 = c.r64(10) | 0x1u64;
            c.w64(10, r90);
            c.logic(r90, 64);
            blk = 83; continue;
        }
        83 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 84; continue;
        }
        84 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089e446); };
            c.w64(9, t91);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089e44d); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089e451); }
            blk = 85; continue;
        }
        85 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089e457); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089e458); }
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089e45a); };
            c.w64(3, t92);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x700089e46e, 0x700089e469);
        }
        86 => {
            let r93 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r93);
            c.w64(8, c.r64(0));
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e473); };
            c.w64(0, t94);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e479); };
            c.w64(1, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089e47d); };
            c.w64(7, t96);
            blk = 30; continue;
        }
        87 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089e486); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e48a); }
            blk = 91; continue;
        }
        88 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089e490); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e494); }
            blk = 91; continue;
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089e49a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089e49e); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t97) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089e4a8); };
            c.w64(11, t97);
            blk = 51; continue;
        }
        91 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let r98 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r98);
            blk = 102; continue;
        }
        93 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089e4c1); };
            c.w64(7, t99);
            let Some(t100) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089e4c8); };
            c.w64(9, t100);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089e4cf); };
            c.w64(12, t101);
            let Some(t102) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700089e4d6); };
            c.cmp(t102, c.r64(11), 64);
            if c.cc_a() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089e4e6, 0x700089e4e1);
        }
        95 => {
            c.w64(7, c.r64(0));
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e4e9); };
            c.w64(8, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e4ed); };
            c.w64(0, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e4f1); };
            c.w64(1, t105);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 97; continue;
        }
        96 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700089e4f9); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 97; continue;
        }
        97 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089e500); };
            c.w64(9, t106);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700089e507); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 101; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            let r107 = c.r64(10) | 0x1u64;
            c.w64(10, r107);
            c.logic(r107, 64);
            blk = 100; continue;
        }
        100 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 101; continue;
        }
        101 => {
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089e532); }
            c.w64(1, c.r64(7));
            blk = 102; continue;
        }
        102 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089e53a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089e53b); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089e53c); };
            c.w64(3, t108);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089e54d, 0x700089e548);
        }
        103 => {
            let r109 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r109);
            c.w64(1, c.r64(0));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e552); };
            c.w64(8, t110);
            blk = 54; continue;
        }
        104 => {
            return RegfileExit::jump(0x700089e55c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089e574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089e574); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089e578); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089e579); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089e57a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700089e57f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089e583); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089e59b); };
            c.w64(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700089e5a0); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e5ad); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e5b3); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700089e5c4); }
            blk = 6; continue;
        }
        4 => {
            let r6 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r6);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700089e5ce); }
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(9));
            return c.call(0x700086f9d4, 0x700089e5dd, 0x700089e5d8);
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089e5dd); };
            c.w64(8, t7);
            c.w64(2, c.r64(0));
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e5e4); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e5e8); };
            c.w64(6, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089e5ec); };
            c.w64(1, t10);
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ge() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x700089e5fa); };
            c.w64(9, t11);
            blk = 10; continue;
        }
        9 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e600); };
            c.w64(9, t12);
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e611); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e618); };
            c.cmp(t15, 0x40u64, 16);
            if c.cc_b() { blk = 4; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 23; continue;
        }
        13 => {
            c.w64(1, c.r64(7));
            let r16 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r16);
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089e632); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089e633); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089e639); }
            return c.ret(0x0, 0x700089e63b);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089e63c); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089e640); };
            c.w64(3, t17);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x700089e651, 0x700089e64c);
        }
        15 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e651); };
            c.w64(6, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e655); };
            c.w64(0, t19);
            blk = 1; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700089e65e); };
            c.cmp(t20, c.r32(2), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e66d); };
            c.cmp(t21, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089e67c); }
            let r22 = c.r32(7) | c.r32(8);
            c.w32(7, r22);
            c.logic(r22, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(2));
            return c.call(0x700089c794, 0x700089e68d, 0x700089e688);
        }
        18 => {
            c.w64(2, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e690); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089e694); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e698); };
            c.w64(0, t25);
            blk = 3; continue;
        }
        19 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089e6a6); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700089e6a7); };
            c.w64(2, t26);
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(0x19b)) else { return RegfileExit::fault(0x700089e6ae); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089e6b7); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089e6b8); };
            c.w64(3, t29);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089e6c9, 0x700089e6c4);
        }
        20 => {
            return RegfileExit::fault(0x700089e6c9);
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089e6ca); }
            c.w64(0, c.r64(2));
            return c.call(0x70008a1214, 0x700089e6d6, 0x700089e6d1);
        }
        22 => {
            c.w64(2, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e6d9); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089e6dd); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e6e1); };
            c.w64(0, t32);
            blk = 3; continue;
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700089e6ea); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700089e6ee); }
            c.w64(0, c.r64(9));
            return c.call(0x700089c794, 0x700089e6fa, 0x700089e6f5);
        }
        24 => {
            c.w64(9, c.r64(0));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089e6fd); };
            c.w64(2, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089e701); };
            c.w64(6, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089e705); };
            c.w64(8, t35);
            blk = 4; continue;
        }
        25 => {
            return RegfileExit::jump(0x700089e710);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089e714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089e714); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089e718); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089e719); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089e71a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700089e71f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(5));
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e72f); };
            c.cmp(t2, c.r32(2), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x700089e73e); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            c.w8(11, (c.cc_be() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e75e); };
            c.w64(12, t3);
            blk = 11; continue;
        }
        4 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089e76d); };
            c.w64(12, t4);
            c.w64(15, c.r64(12));
            blk = 7; continue;
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e776); };
            c.w64(15, t5);
            blk = 7; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e77d); };
            c.w64(12, t6);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e78a); };
            c.w32(0, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089e78e); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700089e797); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x27e8)) else { return RegfileExit::fault(0x700089e799); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089e7aa, 0x700089e7a5);
        }
        10 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089e7aa); };
            c.w64(1, t10);
            let r11 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089e7b5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089e7b6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089e7bc); }
            return c.ret(0x0, 0x700089e7be);
        }
        11 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089e7c5); };
            c.w64(8, t12);
            blk = 14; continue;
        }
        13 => {
            c.w64(8, c.r64(12));
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e7d8); };
            c.w32(11, t13);
            let r14 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r14);
            let Some(t15) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089e7df); };
            c.cmp(t15, 0x40u64, 16);
            if c.cc_b() { blk = 18; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 54; continue;
        }
        17 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089e7ec); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e7f3); };
            c.w64(12, t17);
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089e7fa); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e808); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700089e80f); };
            c.logic(t20 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700089e816); };
            c.w32(9, t21);
            let r22 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r22);
            let Some(t23) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089e81d); };
            c.w32(11, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700089e821); };
            c.cmp(t24, c.r32(11), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089e82a); };
            c.cmp(t25, c.r32(7), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(9));
            blk = 24; continue;
        }
        23 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089e835); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089e836); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089e837); };
            c.w64(7, t26);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089e83e); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x700089e83f); };
            c.w64(3, t27);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700089e850, 0x700089e84b);
        }
        24 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e850); };
            c.logic(t28 & 0x200000u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089e859); };
            c.w64(9, t29);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089e860); };
            c.w64(7, t30);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e869); };
            c.w64(12, t31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089e870); };
            c.w64(1, t32);
            blk = 38; continue;
        }
        26 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089e87c); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089e883); };
            c.w64(7, t34);
            c.w64(8, c.r64(7).wrapping_add(0x4c));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089e88a); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089e891); };
            c.cmp(t36, c.r64(8), 64);
            if c.cc_be() { blk = 56; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089e89a); }
            let r37 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r37);
            blk = 28; continue;
        }
        28 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x700089e8a1); };
            c.w64(8, t38);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700089e8a8); }
            c.w32(8, 0x22u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700089e8b2); }
            let Some(t39) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e8b6); };
            c.w32(8, t39);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 58; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700089e8cc); }
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e8cf); };
            c.w32(8, t40);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700089e8e3); }
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e8e6); };
            c.w32(8, t41);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(8, 0x8u64);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700089e900); }
            let Some(t42) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e904); };
            c.w32(8, t42);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(8, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x700089e91e); }
            let Some(t43) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e922); };
            c.w32(8, t43);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(6)) { return RegfileExit::fault(0x700089e936); }
            c.w64(8, c.r64(7).wrapping_add(0x4b));
            c.w64(11, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e946); };
            c.w64(12, t44);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(11), c.r32(12)) { return RegfileExit::fault(0x700089e94d); }
            let r45 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r45);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 35; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089e95b); };
            c.w64(12, t46);
            blk = 38; continue;
        }
        38 => {
            let Some(t47) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089e962); };
            c.w32(8, t47);
            c.w64(2, c.r64(8).wrapping_mul(4));
            let Some(t48) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089e96f); };
            c.w64(8, t48);
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t49) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089e976); };
            c.cmp(t49, c.r64(11), 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x700089e97f); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700089e98a); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089e98d); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089e990); };
            c.w64(7, t50);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089e997); }
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089e99a); };
            c.w32(7, t51);
            c.w64(9, c.r64(7));
            let r52 = c.r32(9) & 0xe0000000u64;
            c.w32(9, r52);
            c.logic(r52, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 42; continue;
        }
        42 => {
            let r53 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r53);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700089e9b3); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 50; continue;
        }
        44 => {
            let r54 = c.sub(c.r32(7), 0x20000000u64, 32);
            c.w32(7, r54);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x700089e9c5); }
            let Some(t55) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x700089e9c8); };
            c.w32(7, t55);
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x700089e9d9); };
            c.w64(11, t56);
            blk = 46; continue;
        }
        46 => {
            let r57 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r57);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700089e9e4); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 46; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let r58 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r58);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700089e9f7); }
            c.cmp(c.r64(7), 0xbu64, 64);
            if c.cc_ne() { blk = 48; continue; }
            blk = 49; continue;
        }
        49 => {
            let r59 = c.add(c.r32(9), 0xe0000000u64, 32);
            c.w32(9, r59);
            if c.cc_e() { blk = 65; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ea0e); };
            c.w64(0, t60);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700089ea12); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089ea15); };
            c.w64(7, t61);
            let r62 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r62);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089ea23); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089ea24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089ea2a); }
            return c.ret(0x0, 0x700089ea2c);
        }
        51 => {
            let r63 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r63);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089ea37); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x700089ea38); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089ea3e); }
            return c.ret(0x0, 0x700089ea40);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700089ea41); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700089ea45); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700089ea49); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089ea4d); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089ea51); };
            c.w64(3, t64);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x700089ea62, 0x700089ea5d);
        }
        53 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089ea62); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089ea66); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089ea6a); };
            c.w64(6, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ea6e); };
            c.w64(0, t68);
            blk = 1; continue;
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700089ea77); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700089ea7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700089ea7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700089ea83); }
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x700089ea8f, 0x700089ea8a);
        }
        55 => {
            c.w64(8, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089ea92); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089ea96); };
            c.w64(12, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089ea9d); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089eaa1); };
            c.w64(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089eaa5); };
            c.w64(6, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089eaa9); };
            c.w64(9, t74);
            blk = 18; continue;
        }
        56 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700089eab7); }
            return c.call(0x70007febf4, 0x700089eac0, 0x700089eabb);
        }
        57 => {
            c.w64(7, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089eac3); };
            c.w64(0, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089eac7); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089eace); };
            c.w64(9, t77);
            blk = 28; continue;
        }
        58 => {
            return RegfileExit::fault(0x700089eada);
        }
        59 => {
            return RegfileExit::fault(0x700089eadb);
        }
        60 => {
            return RegfileExit::fault(0x700089eadc);
        }
        61 => {
            return RegfileExit::fault(0x700089eadd);
        }
        62 => {
            return RegfileExit::fault(0x700089eade);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700089eadf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700089eae3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700089eae7); }
            return c.call(0x70007febf4, 0x700089eaf0, 0x700089eaeb);
        }
        64 => {
            c.w64(1, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089eaf3); };
            c.w64(0, t78);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089eaf9); };
            c.w64(12, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089eb00); };
            c.w64(7, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089eb04); };
            c.w64(2, t81);
            blk = 40; continue;
        }
        65 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089eb0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700089eb0e); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x700089eb12); };
            c.w64(3, t82);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089eb23, 0x700089eb1e);
        }
        66 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089eb23); };
            c.w64(1, t83);
            blk = 50; continue;
        }
        67 => {
            return RegfileExit::jump(0x700089eb30);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089eb34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089eb34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089eb38); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700089eb3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700089eb42); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089eb4a); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089eb50); };
            c.w32(1, t3);
            let r4 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r4);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.call(0x70008a2374, 0x700089eb67, 0x700089eb62);
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089eb67); };
            c.w64(6, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089eb6b); };
            c.w64(3, t6);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089eb6f); }
            blk = 7; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a2574, 0x700089eb7d, 0x700089eb78);
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089eb7d); };
            c.w64(6, t7);
            blk = 7; continue;
        }
        7 => {
            c.w64(1, c.r64(0));
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089eb94); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089eb9b); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t11) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089eba4); };
            c.w32(7, t11);
            c.cmp(c.r16(7), 0x3fu64, 16);
            if c.cc_be() { blk = 66; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(7, c.r16(7));
            c.cmp(c.r32(7), 0x43u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 11; continue;
        }
        11 => {
            let r12 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r12);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 61; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 5; continue;
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x700089ebd3); };
            c.w64(8, t13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ebda); };
            c.w32(9, t14);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            let r15 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r15);
            let r16 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r16);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700089ebf3); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 68; continue; }
            blk = 15; continue;
        }
        15 => {
            if c.cc_ne() { blk = 68; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 68; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(12, c.r64(7));
            let r17 = c.add(c.r32(12), c.r32(7), 32);
            c.w32(12, r17);
            if c.cc_o() { blk = 68; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 21; continue;
        }
        20 => {
            c.w64(7, c.r64(1));
            blk = 21; continue;
        }
        21 => {
            c.w64(12, c.r64(7));
            let r18 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r18);
            c.w32(15, c.r32(7));
            let r19 = c.r32(11) & c.r32(12);
            c.w32(11, r19);
            c.logic(r19, 32);
            let r20 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r20);
            c.w64(3, c.r64(11).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700089ec57); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x700089ec5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089ec5f); }
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ec63); };
            c.cmp(t21, c.r32(7), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089ec6a); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            blk = 66; continue;
        }
        23 => {
            c.cmp(c.r64(9), 0x8000u64, 64);
            if c.cc_l() { blk = 91; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(9, (c.cc_l() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(0, 0x80000000u64);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 26; continue;
        }
        26 => {
            let r24 = c.neg(c.r32(12), 32);
            c.w32(12, r24);
            blk = 28; continue;
        }
        27 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_l() { blk = 53; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            blk = 31; continue;
        }
        30 => {
            c.w32(0, 0x1u64);
            blk = 31; continue;
        }
        31 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(12));
            c.w32(2, 0x66666667u64);
            blk = 33; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700089ecd2); }
            c.w64(6, c.r64(2));
            c.imul1(c.r32(6), 32);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089ecdb); };
            c.w64(0, t25);
            let r26 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r26);
            let r27 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r27);
            let r28 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r28);
            let r29 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r29);
            c.w64(2, c.r64(6));
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_g() { blk = 32; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            c.w32(2, 0xcu64);
            let r30 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r30);
            if c.cc_o() { blk = 52; continue; }
            blk = 35; continue;
        }
        35 => {
            let r31 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ed0a); };
            c.w64(7, t32);
            let Some(t33) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089ed11); };
            c.w64(7, t33);
            let r34 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r34);
            c.logic(r34, 64);
            c.w64(3, c.r64(7).wrapping_add(c.r64(2)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089ed1c); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089ed23); };
            c.cmp(t36, c.r64(3), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 73; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ed39); };
            c.w64(11, t37);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700089ed40); }
            let r38 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r38);
            blk = 38; continue;
        }
        38 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089ed47); };
            c.w64(11, t39);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089ed4e); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089ed52); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700089ed59); }
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            blk = 40; continue;
        }
        39 => {
            if !c.st8(c.r64(7).wrapping_add(c.r64(11)).wrapping_add(0xb), c.r8(12)) { return RegfileExit::fault(0x700089ed62); }
            let r40 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r40);
            c.w64(12, c.r64(0));
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_le() { blk = 45; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t41) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089ed74); };
            c.w64(0, (t41 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ae() { blk = 51; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(6));
            c.imul1(c.r32(12), 32);
            c.w64(0, c.r64(12));
            let r42 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r42);
            let r43 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r43);
            let r44 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r44);
            let r45 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(3, r45);
            let r46 = c.sub(c.r32(12), c.r32(3), 32);
            c.w32(12, r46);
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let r47 = c.add(c.r32(12), 0x57u64, 32);
            c.w32(12, r47);
            blk = 39; continue;
        }
        44 => {
            let r48 = c.add(c.r32(12), 0x30u64, 32);
            c.w32(12, r48);
            blk = 39; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let r49 = c.shl(c.r32(1), 0x1au64, 32).unwrap_or(c.r32(1));
            c.w32(1, r49);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            let r50 = c.r32(1) | c.r32(9);
            c.w32(1, r50);
            c.logic(r50, 32);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700089edb6); }
            blk = 50; continue;
        }
        47 => {
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089edbb); };
            c.w64(1, (t51 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            return RegfileExit::fault(0x700089edc5);
        }
        49 => {
            if !c.st8(c.r64(7).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x700089edc6); }
            blk = 50; continue;
        }
        50 => {
            c.w64(15, c.r64(7));
            blk = 55; continue;
        }
        51 => {
            return RegfileExit::fault(0x700089edcf);
        }
        52 => {
            return RegfileExit::fault(0x700089edd0);
        }
        53 => {
            c.w32(1, c.r64(12).wrapping_add(0x30));
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 75; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089edea); };
            c.w64(12, t52);
            let Some(t53) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089edf1); };
            c.w32(15, t53);
            let r54 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089edf9); };
            c.cmp(t55, c.r32(15), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t56) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ee06); };
            c.w32(1, t56);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089ee10); };
            c.w64(7, t57);
            c.cmp(c.r64(7), c.r64(1), 64);
            if c.cc_ae() { blk = 89; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(1, c.r64(7).wrapping_add(0x1));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089ee21); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089ee25); };
            c.w64(9, t59);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700089ee29); }
            let Some(t60) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ee2e); };
            c.w32(7, t60);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ae() { blk = 90; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(15)) { return RegfileExit::fault(0x700089ee49); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r61 = c.r64(7) & c.r64(8);
            c.w64(7, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ee5d); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 96; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(1, c.r64(15));
            blk = 66; continue;
        }
        60 => {
            let Some(t63) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089ee6c); };
            c.w32(1, t63);
            let r64 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r64);
            blk = 66; continue;
        }
        61 => {
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089ee74); };
            c.cmp(t65, c.r32(8), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(1, 0x168u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089ee82); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089ee83); };
            c.w64(3, t66);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089ee94, 0x700089ee8f);
        }
        63 => {
            return RegfileExit::fault(0x700089ee94);
        }
        64 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ee95); };
            if !c.push64(t67) { return RegfileExit::fault(0x700089ee95); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x24d0)) else { return RegfileExit::fault(0x700089ee98); };
            c.w64(3, t68);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089eea9, 0x700089eea4);
        }
        65 => {
            c.w64(1, c.r64(0));
            blk = 66; continue;
        }
        66 => {
            c.w64(0, c.r64(1));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089eeaf); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089eeb3); };
            c.w64(6, t70);
            return c.call(0x700086f9d4, 0x700089eebc, 0x700089eeb7);
        }
        67 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089eebf); }
            return c.ret(0x0, 0x700089eec0);
        }
        68 => {
            let Some(t71) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089eec1); };
            c.w32(7, t71);
            let Some(t72) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089eec4); };
            c.w32(9, t72);
            c.w64(12, c.r64(7));
            let r73 = c.r32(12) ^ c.r32(9);
            c.w32(12, r73);
            c.logic(r73, 32);
            let r74 = c.r32(12) & c.r32(11);
            c.w32(12, r74);
            c.logic(r74, 32);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089eed9); };
            c.w32(12, t75);
            let r76 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r76);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t77) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089eeeb); };
            c.w32(15, t77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089eef0); };
            c.cmp(t78, c.r32(15), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089eefd); };
            c.w32(15, t79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089ef02); };
            c.cmp(t80, c.r32(7), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t81) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089ef16); };
            c.w32(1, t81);
            let r82 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r82);
            blk = 66; continue;
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089ef20); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700089ef24); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700089ef28); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700089ef2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700089ef30); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700089ef34); }
            return c.call(0x70007febb4, 0x700089ef3d, 0x700089ef38);
        }
        74 => {
            c.w64(7, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ef40); };
            c.w64(8, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089ef44); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089ef48); };
            c.w64(1, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089ef4c); };
            c.w64(9, t86);
            c.w32(6, 0x66666667u64);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089ef55); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089ef59); };
            c.w64(0, t88);
            blk = 38; continue;
        }
        75 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ef62); };
            c.w64(9, t89);
            let Some(t90) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089ef69); };
            c.w64(12, t90);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089ef71); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700089ef75); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089ef79); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089ef80); };
            c.cmp(t92, c.r64(15), 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089ef8f, 0x700089ef8a);
        }
        77 => {
            blk = 79; continue;
        }
        78 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700089ef91); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 79; continue;
        }
        79 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089ef9c); };
            c.w64(1, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089efa3); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089efa6); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089efad); }
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089efb4); };
            c.w64(1, t94);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x700089efb8); }
            c.w64(15, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089efbf); };
            c.w64(6, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089efc3); };
            c.w64(8, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089efc7); };
            c.w64(3, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089efcb); };
            c.w64(7, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089efcf); };
            c.w64(11, t99);
            blk = 55; continue;
        }
        80 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089efd8); };
            c.w64(15, t100);
            let Some(t101) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089efdf); };
            c.w64(0, t101);
            c.w64(2, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700089efe6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700089efea); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700089efee); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700089eff2); }
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089eff6); };
            c.w64(0, t102);
            let Some(t103) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089effd); };
            c.cmp(t103, c.r64(2), 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089f00c, 0x700089f007);
        }
        82 => {
            blk = 84; continue;
        }
        83 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700089f00e); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089f011); };
            c.w64(15, t104);
            let r105 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r105);
            c.w64(0, c.r64(15));
            blk = 84; continue;
        }
        84 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089f01c); };
            c.w64(1, t106);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089f023); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089f026); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f02d); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089f034); };
            c.w64(1, t107);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700089f038); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089f03b); };
            c.w64(12, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f042); };
            c.w32(7, t109);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            return RegfileExit::fault(0x700089f052);
        }
        86 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089f053); };
            c.w64(9, t110);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700089f057); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r111 = c.r64(1) & c.r64(12);
            c.w64(1, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f069); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 93; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(15, c.r64(0));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f072); };
            c.w64(6, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089f076); };
            c.w64(8, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089f07a); };
            c.w64(3, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089f07e); };
            c.w64(7, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089f082); };
            c.w64(11, t117);
            blk = 55; continue;
        }
        89 => {
            return RegfileExit::fault(0x700089f08b);
        }
        90 => {
            return RegfileExit::fault(0x700089f08c);
        }
        91 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089f08d); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x700089f08e); };
            c.w64(3, t118);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700089f0a4, 0x700089f09f);
        }
        92 => {
            c.w64(1, c.r64(0));
            blk = 66; continue;
        }
        93 => {
            c.w32(0, c.r32(0));
            let r119 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r119);
            c.w64(1, 0xfffffffffffc0000u64);
            let r120 = c.r64(1) & c.r64(0);
            c.w64(1, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f0bb); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089f0c5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089f0c6); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x700089f0d3, 0x700089f0ce);
        }
        95 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089f0d3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089f0d5); }
            blk = 88; continue;
        }
        96 => {
            c.w32(15, c.r32(15));
            let r122 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r122);
            c.w64(7, 0xfffffffffffc0000u64);
            let r123 = c.r64(7) & c.r64(15);
            c.w64(7, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f0e8); };
            c.logic(t124 & 0x2u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089f0f6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089f0f7); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089f104, 0x700089f0ff);
        }
        98 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089f104); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089f106); }
            blk = 59; continue;
        }
        99 => {
            return RegfileExit::jump(0x700089f110);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089f114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f114); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f118); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089f11e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700089f122); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f12b); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f131); };
            c.w32(1, t3);
            let r4 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r4);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a2374, 0x700089f14b, 0x700089f146);
        }
        3 => {
            c.w64(1, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089f14e); };
            c.w64(0, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f152); };
            c.w64(6, t6);
            blk = 5; continue;
        }
        4 => {
            c.w64(1, c.r64(3));
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700089f15b); }
            c.w64(7, c.r64(1));
            blk = 8; continue;
        }
        6 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a2574, 0x700089f16c, 0x700089f167);
        }
        7 => {
            c.w64(7, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f16f); };
            c.w64(6, t7);
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f185); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089f18c); };
            c.cmp(t10, c.r32(9), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f195); };
            c.w32(8, t11);
            c.cmp(c.r16(8), 0x3fu64, 16);
            if c.cc_be() { blk = 67; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0x43u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 12; continue;
        }
        12 => {
            let r12 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r12);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 62; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 6; continue;
        }
        14 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x700089f1c9); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f1d0); };
            c.w32(9, t14);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            let r15 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r15);
            let r16 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r16);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089f1ea); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 69; continue; }
            blk = 16; continue;
        }
        16 => {
            if c.cc_ne() { blk = 69; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.movq_rx(12, 0);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(12, c.r64(8));
            let r17 = c.add(c.r32(12), c.r32(8), 32);
            c.w32(12, r17);
            if c.cc_o() { blk = 69; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 22; continue;
        }
        21 => {
            c.w64(8, c.r64(7));
            blk = 22; continue;
        }
        22 => {
            c.w64(12, c.r64(8));
            let r18 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r18);
            c.w32(15, c.r32(8));
            let r19 = c.r32(11) & c.r32(12);
            c.w32(11, r19);
            c.logic(r19, 32);
            let r20 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r20);
            c.w64(3, c.r64(11).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700089f251); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x700089f255); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089f259); }
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f25d); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f264); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            blk = 67; continue;
        }
        24 => {
            c.cmp(c.r64(9), 0x8000u64, 64);
            if c.cc_l() { blk = 92; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(9, (c.cc_l() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(0, 0x80000000u64);
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 27; continue;
        }
        27 => {
            let r24 = c.neg(c.r32(12), 32);
            c.w32(12, r24);
            blk = 29; continue;
        }
        28 => {
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            blk = 32; continue;
        }
        31 => {
            c.w32(0, 0x1u64);
            blk = 32; continue;
        }
        32 => {
            c.w64(7, c.r64(0));
            c.w64(0, c.r64(12));
            c.w32(2, 0x66666667u64);
            blk = 34; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700089f2cb); }
            c.w64(6, c.r64(2));
            c.imul1(c.r32(6), 32);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089f2d4); };
            c.w64(0, t25);
            let r26 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r26);
            let r27 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r27);
            let r28 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r28);
            let r29 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r29);
            c.w64(2, c.r64(6));
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_g() { blk = 33; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(2));
            c.w32(2, 0xcu64);
            let r30 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r30);
            if c.cc_o() { blk = 53; continue; }
            blk = 36; continue;
        }
        36 => {
            let r31 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f303); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089f30a); };
            c.w64(8, t33);
            let r34 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r34);
            c.logic(r34, 64);
            c.w64(3, c.r64(8).wrapping_add(c.r64(2)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f315); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089f31c); };
            c.cmp(t36, c.r64(3), 64);
            if c.cc_be() { blk = 74; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 74; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f332); };
            c.w64(11, t37);
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x700089f339); }
            let r38 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r38);
            blk = 39; continue;
        }
        39 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089f340); };
            c.w64(11, t39);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089f347); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f34b); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x700089f353); }
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            blk = 41; continue;
        }
        40 => {
            if !c.st8(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0xb), c.r8(12)) { return RegfileExit::fault(0x700089f35d); }
            let r40 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r40);
            c.w64(12, c.r64(0));
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_le() { blk = 46; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f36f); };
            c.w64(0, (t41 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ae() { blk = 52; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(0, c.r64(6));
            c.imul1(c.r32(12), 32);
            c.w64(0, c.r64(12));
            let r42 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r42);
            let r43 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r43);
            let r44 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r44);
            let r45 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(3, r45);
            let r46 = c.sub(c.r32(12), c.r32(3), 32);
            c.w32(12, r46);
            c.cmp(c.r32(12), 0xau64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r47 = c.add(c.r32(12), 0x57u64, 32);
            c.w32(12, r47);
            blk = 40; continue;
        }
        45 => {
            let r48 = c.add(c.r32(12), 0x30u64, 32);
            c.w32(12, r48);
            blk = 40; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let r49 = c.shl(c.r32(7), 0x1au64, 32).unwrap_or(c.r32(7));
            c.w32(7, r49);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            let r50 = c.r32(7) | c.r32(9);
            c.w32(7, r50);
            c.logic(r50, 32);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089f3b1); }
            blk = 51; continue;
        }
        48 => {
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f3b7); };
            c.w64(7, (t51 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            return RegfileExit::fault(0x700089f3c1);
        }
        50 => {
            if !c.st8(c.r64(8).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x700089f3c2); }
            blk = 51; continue;
        }
        51 => {
            c.w64(15, c.r64(8));
            blk = 56; continue;
        }
        52 => {
            return RegfileExit::fault(0x700089f3cc);
        }
        53 => {
            return RegfileExit::fault(0x700089f3cd);
        }
        54 => {
            c.w32(7, c.r64(12).wrapping_add(0x30));
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 76; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089f3e7); };
            c.w64(12, t52);
            let Some(t53) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f3ee); };
            c.w32(15, t53);
            let r54 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089f3f6); };
            c.cmp(t55, c.r32(15), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f403); };
            c.w32(7, t56);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089f40c); };
            c.w64(8, t57);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 90; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(7, c.r64(8).wrapping_add(0x1));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089f41d); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089f421); };
            c.w64(9, t59);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700089f425); }
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f42a); };
            c.w32(8, t60);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ae() { blk = 91; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(15)) { return RegfileExit::fault(0x700089f445); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r61 = c.r64(8) & c.r64(1);
            c.w64(8, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f459); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(7, c.r64(15));
            blk = 67; continue;
        }
        61 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f469); };
            c.w32(7, t63);
            let r64 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r64);
            blk = 67; continue;
        }
        62 => {
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700089f471); };
            c.cmp(t65, c.r32(9), 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(1, 0x168u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089f47f); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700089f480); };
            c.w64(3, t66);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089f491, 0x700089f48c);
        }
        64 => {
            return RegfileExit::fault(0x700089f491);
        }
        65 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089f492); };
            if !c.push64(t67) { return RegfileExit::fault(0x700089f492); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x24d0)) else { return RegfileExit::fault(0x700089f495); };
            c.w64(3, t68);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700089f4a6, 0x700089f4a1);
        }
        66 => {
            c.w64(7, c.r64(0));
            blk = 67; continue;
        }
        67 => {
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089f4a9); };
            c.w64(0, t69);
            c.w64(3, c.r64(7));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f4b0); };
            c.w64(6, t70);
            return c.call(0x700086f9d4, 0x700089f4b9, 0x700089f4b4);
        }
        68 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089f4bc); }
            return c.ret(0x0, 0x700089f4bd);
        }
        69 => {
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f4be); };
            c.w32(8, t71);
            let Some(t72) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f4c2); };
            c.w32(9, t72);
            c.w64(12, c.r64(8));
            let r73 = c.r32(12) ^ c.r32(9);
            c.w32(12, r73);
            c.logic(r73, 32);
            let r74 = c.r32(12) & c.r32(11);
            c.w32(12, r74);
            c.logic(r74, 32);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f4d7); };
            c.w32(12, t75);
            let r76 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r76);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t77) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f4e9); };
            c.w32(15, t77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089f4ee); };
            c.cmp(t78, c.r32(15), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f4fb); };
            c.w32(15, t79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f500); };
            c.cmp(t80, c.r32(8), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(15), c.r32(9), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t81) = c.ld32(c.r64(1).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f514); };
            c.w32(7, t81);
            let r82 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r82);
            blk = 67; continue;
        }
        74 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700089f51e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700089f522); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700089f526); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700089f52a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700089f52e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x700089f532); }
            return c.call(0x70007febb4, 0x700089f53b, 0x700089f536);
        }
        75 => {
            c.w64(8, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089f53e); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089f542); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089f546); };
            c.w64(7, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089f54a); };
            c.w64(9, t86);
            c.w32(6, 0x66666667u64);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089f553); };
            c.w64(12, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700089f557); };
            c.w64(0, t88);
            blk = 39; continue;
        }
        76 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f560); };
            c.w64(9, t89);
            let Some(t90) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089f567); };
            c.w64(12, t90);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700089f56f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700089f573); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f577); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089f57e); };
            c.cmp(t92, c.r64(15), 64);
            if c.cc_a() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089f58d, 0x700089f588);
        }
        78 => {
            blk = 80; continue;
        }
        79 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700089f58f); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 80; continue;
        }
        80 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089f59a); };
            c.w64(1, t93);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089f5a1); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089f5a4); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f5ab); }
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089f5b2); };
            c.w64(7, t94);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x700089f5b6); }
            c.w64(15, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f5bd); };
            c.w64(6, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089f5c1); };
            c.w64(1, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089f5c5); };
            c.w64(3, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089f5c9); };
            c.w64(8, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089f5cd); };
            c.w64(11, t99);
            blk = 56; continue;
        }
        81 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f5d6); };
            c.w64(15, t100);
            let Some(t101) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089f5dd); };
            c.w64(0, t101);
            c.w64(2, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700089f5e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700089f5e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700089f5ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700089f5f0); }
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f5f4); };
            c.w64(0, t102);
            let Some(t103) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089f5fb); };
            c.cmp(t103, c.r64(2), 64);
            if c.cc_a() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089f60a, 0x700089f605);
        }
        83 => {
            blk = 85; continue;
        }
        84 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x700089f60c); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089f60f); };
            c.w64(15, t104);
            let r105 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r105);
            c.w64(0, c.r64(15));
            blk = 85; continue;
        }
        85 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089f61a); };
            c.w64(1, t106);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089f621); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089f624); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f62b); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089f632); };
            c.w64(1, t107);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700089f636); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089f639); };
            c.w64(12, t108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089f640); };
            c.w32(7, t109);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x700089f650);
        }
        87 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089f651); };
            c.w64(9, t110);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700089f655); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r111 = c.r64(1) & c.r64(12);
            c.w64(1, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f667); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(15, c.r64(0));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f670); };
            c.w64(6, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089f674); };
            c.w64(1, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089f678); };
            c.w64(3, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089f67c); };
            c.w64(8, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089f680); };
            c.w64(11, t117);
            blk = 56; continue;
        }
        90 => {
            return RegfileExit::fault(0x700089f689);
        }
        91 => {
            return RegfileExit::fault(0x700089f68a);
        }
        92 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089f68b); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x700089f68c); };
            c.w64(3, t118);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700089f6a2, 0x700089f69d);
        }
        93 => {
            c.w64(7, c.r64(0));
            blk = 67; continue;
        }
        94 => {
            c.w32(0, c.r32(0));
            let r119 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r119);
            c.w64(1, 0xfffffffffffc0000u64);
            let r120 = c.r64(1) & c.r64(0);
            c.w64(1, r120);
            c.logic(r120, 64);
            let Some(t121) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f6b9); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089f6c3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089f6c4); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x700089f6d1, 0x700089f6cc);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089f6d1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089f6d3); }
            blk = 89; continue;
        }
        97 => {
            c.w32(15, c.r32(15));
            let r122 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r122);
            c.w64(8, 0xfffffffffffc0000u64);
            let r123 = c.r64(8) & c.r64(15);
            c.w64(8, r123);
            c.logic(r123, 64);
            let Some(t124) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089f6e6); };
            c.logic(t124 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089f6f5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089f6f6); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089f703, 0x700089f6fe);
        }
        99 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089f703); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089f705); }
            blk = 60; continue;
        }
        100 => {
            return RegfileExit::jump(0x700089f70c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089f714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f714); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f71a); };
            c.w32(1, t2);
            c.logic(c.r8(1) & 0x7u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            blk = 2; continue;
        }
        2 => {
            c.w64(9, c.r64(8));
            let r3 = c.r32(9) & 0x7u64;
            c.w32(9, r3);
            c.logic(r3, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 25; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 25; continue;
        }
        10 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089f76b); };
            c.w32(8, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089f76f); };
            c.cmp(t5, c.r32(8), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f77c); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f782); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f789); };
            c.w32(8, t10);
            blk = 2; continue;
        }
        12 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089f790); };
            c.w32(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r12 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f79d); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f7a3); };
            c.w32(8, t15);
            let r16 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r16);
            let Some(t17) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f7aa); };
            c.w32(8, t17);
            blk = 2; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089f7b1); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            let Some(t20) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089f7b7); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            let Some(t22) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f7be); };
            c.w32(8, t22);
            blk = 2; continue;
        }
        14 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 17; continue;
        }
        15 => {
            c.w32(9, 0x1u64);
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 17; continue;
        }
        16 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(6));
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 18; continue;
        }
        18 => {
            let r23 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r23);
            blk = 19; continue;
        }
        19 => {
            let r24 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r24);
            let r25 = c.r32(1) & 0x8u64;
            c.w32(1, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t26) = c.ld16(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x700089f81d); };
            c.w32(1, t26);
            blk = 22; continue;
        }
        21 => {
            let Some(t27) = c.ld8(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700089f824); };
            c.w32(1, t27);
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 42; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089f83d); };
            c.w64(8, t28);
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089f844); };
            c.w32(0, t29);
            let r30 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r30);
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089f84c); };
            c.cmp(t31, c.r32(0), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 24; continue;
        }
        24 => {
            return c.ret(0x0, 0x700089f859);
        }
        25 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f85a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f85e); }
            let r32 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089f864); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700089f868); }
            blk = 29; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f86e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f872); }
            let r33 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r33);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089f878); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700089f87c); }
            blk = 29; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f882); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f886); }
            let r34 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700089f88c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700089f890); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t35) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089f899); };
            c.w64(8, t35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089f8a0); }
            blk = 19; continue;
        }
        29 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 40; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f8b4); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089f8bb); };
            c.w64(7, t37);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f8c2); };
            c.w64(9, t38);
            let Some(t39) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089f8c9); };
            c.cmp(t39, c.r64(8), 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089f8d8, 0x700089f8d3);
        }
        33 => {
            c.w64(1, c.r64(0));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089f8db); };
            c.w64(0, t40);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f8e1); };
            c.w64(3, t41);
            blk = 35; continue;
        }
        34 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089f8e7); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 35; continue;
        }
        35 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let r42 = c.r64(10) | 0x1u64;
            c.w64(10, r42);
            c.logic(r42, 64);
            blk = 38; continue;
        }
        38 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089f915); };
            c.w64(7, t43);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089f91c); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089f91f); }
            blk = 40; continue;
        }
        40 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089f924); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089f925); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700089f926); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700089f937, 0x700089f932);
        }
        41 => {
            let r45 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r45);
            c.w64(1, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089f93f); }
            blk = 22; continue;
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f945); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f949); }
            let r46 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r46);
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f94f); };
            c.w64(7, t47);
            let Some(t48) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089f956); };
            c.w64(8, t48);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700089f95d); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f961); };
            c.w64(11, t49);
            let Some(t50) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089f968); };
            c.cmp(t50, c.r64(9), 64);
            if c.cc_a() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089f977, 0x700089f972);
        }
        44 => {
            blk = 46; continue;
        }
        45 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x700089f979); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 46; continue;
        }
        46 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x700089f983); };
            c.w64(1, t51);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089f98a); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089f98d); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f994); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089f99b); };
            c.w64(1, t52);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x700089f99f); }
            blk = 47; continue;
        }
        47 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089f9a6); }
            return c.ret(0x0, 0x700089f9a7);
        }
        48 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089f9a8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089f9ac); }
            let r53 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089f9b2); };
            c.w64(9, t54);
            let Some(t55) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089f9b9); };
            c.w64(11, t55);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700089f9c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700089f9c4); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089f9c8); };
            c.w64(15, t56);
            let Some(t57) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089f9cf); };
            c.cmp(t57, c.r64(12), 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700089f9de, 0x700089f9d9);
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700089f9e0); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 52; continue;
        }
        52 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x700089f9ea); };
            c.w64(1, t58);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089f9f1); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x700089f9f4); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700089f9fb); }
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089fa02); };
            c.w64(1, t59);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x700089fa06); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700089fa09); };
            c.w64(8, t60);
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089fa10); };
            c.w32(7, t61);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            return RegfileExit::fault(0x700089fa1f);
        }
        54 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089fa20); };
            c.w64(7, t62);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700089fa24); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r63 = c.r64(1) & c.r64(8);
            c.w64(1, r63);
            c.logic(r63, 64);
            let Some(t64) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089fa36); };
            c.logic(t64 & 0x4u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 47; continue;
        }
        57 => {
            c.w32(0, c.r32(0));
            let r65 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r65);
            c.w64(1, 0xfffffffffffc0000u64);
            let r66 = c.r64(1) & c.r64(0);
            c.w64(1, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089fa50); };
            c.logic(t67 & 0x2u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089fa5a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089fa5b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700089fa68, 0x700089fa63);
        }
        59 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089fa68); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089fa6a); }
            blk = 56; continue;
        }
        60 => {
            return RegfileExit::jump(0x700089fa70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089fa74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700089fa77); };
            c.w64(2, t0);
            c.w64(6, c.r64(2));
            return RegfileExit::jump(0x70007f9f34);
        }
        1 => {
            return RegfileExit::jump(0x700089fa84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089fa94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(3, c.r32(3));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089fa96); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089fa9d); };
            c.w64(7, t1);
            c.w64(2, c.r64(3).wrapping_mul(4).wrapping_add(0x10));
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089faac); };
            c.w64(9, t2);
            let Some(t3) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089fab3); };
            c.cmp(t3, c.r64(8), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089fab8); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fabf); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089fac5); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089facb); };
            c.w32(8, t8);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x7bu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700089fadb);
        }
        4 => {
            let r9 = c.add(c.r64(3), 0x2u64, 64);
            c.w64(3, r9);
            let r10 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x700089fae2); };
            c.w32(7, t11);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089fae8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700089faeb); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700089faee); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700089faf1); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089faff); };
            c.w64(6, t12);
            blk = 7; continue;
        }
        6 => {
            c.w64(3, c.r64(7));
            blk = 7; continue;
        }
        7 => {
            c.w64(7, c.r64(3).wrapping_add(0x4));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700089fb0f); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700089fb1b);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fb1c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fb20); }
            let r13 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700089fb26); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700089fb2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089fb2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089fb32); }
            return c.call(0x70007febf4, 0x700089fb3b, 0x700089fb36);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089fb3e); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089fb42); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089fb46); };
            c.w64(0, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089fb4a); };
            c.w64(3, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089fb51); }
            blk = 2; continue;
        }
        11 => {
            return RegfileExit::jump(0x700089fb58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089fb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(3, c.r32(3));
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089fb76); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089fb7d); };
            c.w64(7, t1);
            c.w64(2, c.r64(3).wrapping_mul(4).wrapping_add(0x10));
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089fb8c); };
            c.w64(9, t2);
            let Some(t3) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089fb93); };
            c.cmp(t3, c.r64(8), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x700089fb98); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fb9f); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089fba5); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089fbab); };
            c.w32(8, t8);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x79u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x700089fbbb);
        }
        4 => {
            let r9 = c.add(c.r64(3), 0x2u64, 64);
            c.w64(3, r9);
            let r10 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x700089fbc2); };
            c.w32(7, t11);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089fbc8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700089fbcb); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700089fbce); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x700089fbd1); }
            c.cmp(c.r64(2), 0x10u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x10u64);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089fbdf); };
            c.w64(6, t12);
            blk = 7; continue;
        }
        6 => {
            c.w64(3, c.r64(7));
            blk = 7; continue;
        }
        7 => {
            c.w64(7, c.r64(3).wrapping_add(0x4));
            if !c.st32(c.r64(1).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700089fbef); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700089fbfb);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fbfc); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fc00); }
            let r13 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r13);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700089fc06); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700089fc0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700089fc0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089fc12); }
            return c.call(0x70007febf4, 0x700089fc1b, 0x700089fc16);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089fc1e); };
            c.w64(2, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089fc22); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089fc26); };
            c.w64(0, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089fc2a); };
            c.w64(3, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089fc31); }
            blk = 2; continue;
        }
        11 => {
            return RegfileExit::jump(0x700089fc38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089fc54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089fc54); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700089fc61); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089fc72); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700089fc79); };
            c.w64(8, t4);
            c.w64(9, c.r64(8).wrapping_add(0x1c));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089fc80); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089fc87); };
            c.cmp(t6, c.r64(9), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x700089fc8c); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fc93); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089fc9a); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089fca1); };
            c.w32(9, t11);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x9au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700089fcb4);
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x26f)) else { return RegfileExit::fault(0x700089fcb5); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700089fcbf); };
            c.w32(8, t14);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700089fcc3); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089fcc7); };
            c.w64(8, t15);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700089fcce); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700089fcd2); }
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089fcd6); };
            c.w32(8, t16);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700089fcda); }
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089fcde); };
            c.w32(8, t17);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x700089fce2); }
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089fce6); };
            c.w32(7, t18);
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x700089fce9); }
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x700089fcee); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x700089fcf4);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fcf5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fcf9); }
            let r19 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r19);
            blk = 11; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fd01); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fd05); }
            let r20 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r20);
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089fd10); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089fd14); }
            return c.call(0x70007febf4, 0x700089fd1d, 0x700089fd18);
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089fd20); };
            c.w64(6, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700089fd24); };
            c.w64(7, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089fd2b); }
            blk = 4; continue;
        }
        10 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fd31); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fd35); }
            let r23 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r23);
            blk = 11; continue;
        }
        11 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089fd3b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700089fd3c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700089fd3d); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089fd3e); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2290)) else { return RegfileExit::fault(0x700089fd3f); };
            c.w64(3, t24);
            c.w32(0, 0x4u64);
            return c.call(0x700086ed74, 0x700089fd50, 0x700089fd4b);
        }
        12 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089fd53); }
            return c.ret(0x0, 0x700089fd54);
        }
        13 => {
            return RegfileExit::jump(0x700089fd58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700089fd74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700089fd74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700089fd78); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fd7e); };
            c.w32(7, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700089fd81); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700089fd85); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700089fd92); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700089fd99); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089fda7); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089fdae); };
            c.w32(11, t7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fdb2); };
            c.w32(12, t8);
            let r9 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r9);
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089fdb9); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t12) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x700089fdc3); };
            c.w32(12, t12);
            let Some(t13) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089fdc9); };
            c.w32(0, t13);
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700089fdd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x700089fdd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700089fddb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700089fddf); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 27; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089fded); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700089fdfa); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 38; continue; }
            blk = 6; continue;
        }
        6 => {
            let r16 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r16);
            c.w64(8, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089fe1a); };
            c.w64(15, t17);
            let Some(t18) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700089fe21); };
            c.w64(15, t18);
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089fe2c); };
            c.w64(7, t19);
            let Some(t20) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700089fe33); };
            c.cmp(t20, c.r64(1), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 36; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089fe49); };
            c.w64(7, t21);
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x700089fe50); }
            c.w64(7, c.r64(15).wrapping_add(0x1));
            blk = 9; continue;
        }
        9 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700089fe57); }
            c.w64(15, c.r64(7));
            let r22 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r22);
            c.logic(r22, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x700089fe64); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x700089fe67); }
            let Some(t23) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089fe6b); };
            c.logic(t23 & 0x4u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(15, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(0, 0x7u64);
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x700089fe81); };
            c.w32(3, t24);
            if !c.st32(c.r64(7).wrapping_add(c.r64(0)), c.r32(3)) { return RegfileExit::fault(0x700089fe85); }
            let r25 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r25);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ne() { blk = 12; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 16; continue;
        }
        14 => {
            c.w64(2, c.r64(9).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r26 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r26);
            let r27 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r27);
            c.logic(r27, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089fea6); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700089feab); };
            c.w64(0, t28);
            c.w64(10, 0x700089feb9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089feb9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089febd); }
            return c.call(c.r64(0), 0x700089fec3, 0x700089fec1);
        }
        15 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089fec3); }
            let Some(t29) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089fecb); };
            c.w64(4, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089fed0); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089fed4); };
            c.w64(11, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089fed8); };
            c.w64(12, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089fedc); };
            c.w64(7, t33);
            blk = 16; continue;
        }
        16 => {
            c.w64(9, c.r64(7));
            blk = 28; continue;
        }
        17 => {
            c.cmp(c.r64(3), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 44; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(8).wrapping_add(0x8));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700089ff01); }
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 19; continue;
        }
        19 => {
            let r34 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ff14); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700089ff1b); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ff1e); };
            c.w64(2, t37);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x700089ff22); }
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let r38 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r38);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700089ff32); }
            c.w32(15, 0x4u64);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700089ff3a); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700089ff3e); };
            c.w64(2, t39);
            c.w32(0, 0x2u64);
            c.w64(15, c.r64(0));
            c.w64(3, c.r64(2));
            return c.call(0x700086ed74, 0x700089ff55, 0x700089ff50);
        }
        21 => {
            blk = 24; continue;
        }
        22 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(1)));
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089ff64); };
            c.w64(15, t40);
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x700089ff6b); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 24; continue;
        }
        24 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089ff75); };
            c.w64(7, t41);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089ff79); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r42 = c.r64(9) & c.r64(0);
            c.w64(9, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089ff86); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089ff91); };
            c.w64(7, t44);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700089ff95); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089ff98); };
            c.w64(7, t45);
            c.w64(2, c.r64(7).wrapping_add(0x7));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r46 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r46);
            let r47 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r47);
            c.logic(r47, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700089ffaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x700089ffb4); }
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089ffb8); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x700089ffbc); };
            c.w64(0, t49);
            c.w64(10, 0x700089ffcau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089ffca); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700089ffce); }
            return c.call(c.r64(0), 0x700089ffd4, 0x700089ffd2);
        }
        26 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700089ffd4); }
            let Some(t50) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089ffdc); };
            c.w64(4, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089ffe1); };
            c.w64(9, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089ffe5); };
            c.w64(6, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089ffe9); };
            c.w64(11, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089ffed); };
            c.w64(12, t54);
            blk = 28; continue;
        }
        27 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700089fff3); };
            c.w64(0, t55);
            c.w64(9, c.r64(0));
            blk = 28; continue;
        }
        28 => {
            let r56 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r56);
            let r57 = c.r32(12) & 0x3fu64;
            c.w32(12, r57);
            c.logic(r57, 32);
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_le() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(7, 0x2u64);
            c.w64(12, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            let Some(t58) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0013); };
            c.w32(7, t58);
            let r59 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a0019); };
            c.w32(7, t60);
            let r61 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r61);
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a001f); };
            c.w32(15, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r63 = c.add(c.r64(12), 0x69u64, 64);
            c.w64(12, r63);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008a0035);
        }
        32 => {
            let Some(t64) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a0036); };
            c.w32(7, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a003b); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a0042); };
            c.w64(15, t66);
            c.w64(0, c.r64(15).wrapping_add(0x18));
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a004a); };
            c.w64(3, t67);
            let Some(t68) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a0051); };
            c.cmp(t68, c.r64(0), 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008a005a); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 34; continue;
        }
        34 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008a0062); }
            c.w64(7, c.r64(11));
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008a006a); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a006f); };
            c.w64(7, t69);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008a0076); }
            c.w64(7, c.r64(12).wrapping_add(0x10));
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008a0080); };
            c.w64(11, t70);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008a0087); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a008b); };
            c.w64(11, t71);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008a008f); }
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a0093); };
            c.w32(7, t72);
            let r73 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r73);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008a009a); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008a009e); }
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a00a9); }
            return c.ret(0x0, 0x70008a00aa);
        }
        35 => {
            blk = 48; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a00b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008a00b4); }
            return c.call(0x70007febb4, 0x70008a00bd, 0x70008a00b8);
        }
        37 => {
            c.w64(7, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a00c0); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a00c4); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a00c8); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a00cc); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a00d0); };
            c.w64(0, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a00d4); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a00d8); };
            c.w64(3, t80);
            blk = 9; continue;
        }
        38 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008a00e5); };
            c.w64(3, t81);
            c.w64(7, c.r64(0));
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a00f7, 0x70008a00f2);
        }
        39 => {
            return RegfileExit::fault(0x70008a00f7);
        }
        40 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008a00f8); };
            c.w64(3, t82);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a0108, 0x70008a0103);
        }
        41 => {
            return RegfileExit::fault(0x70008a0108);
        }
        42 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a010d); }
            c.w32(7, 0x4u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0113); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008a0116); };
            c.w64(3, t83);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x70008a012a, 0x70008a0125);
        }
        43 => {
            blk = 24; continue;
        }
        44 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008a012f); };
            c.w64(3, t84);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a013f, 0x70008a013a);
        }
        45 => {
            return RegfileExit::fault(0x70008a013f);
        }
        46 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a0145); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a0149); }
            return c.call(0x70007febf4, 0x70008a0152, 0x70008a014d);
        }
        47 => {
            c.w64(12, c.r64(0));
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a0155); };
            c.w64(9, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0159); };
            c.w64(11, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a015d); };
            c.w64(7, t87);
            blk = 34; continue;
        }
        48 => {
            let Some(t88) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a0166); };
            c.w64(5, t88);
            let r89 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r89);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008a016e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a016f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a0170); }
            let Some(t90) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a0171); };
            if !c.push64(t90) { return RegfileExit::fault(0x70008a0171); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a0175); }
            c.w32(0, 0x4u64);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2270)) else { return RegfileExit::fault(0x70008a017f); };
            c.w64(3, t91);
            return RegfileExit::jump(0x700086ed74);
        }
        49 => {
            c.w32(7, c.r32(7));
            let r92 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r92);
            c.w64(9, 0xfffffffffffc0000u64);
            let r93 = c.r64(9) & c.r64(7);
            c.w64(9, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a019a); };
            c.logic(t94 & 0x2u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a01a9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a01aa); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008a01b7, 0x70008a01b2);
        }
        51 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a01b7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a01b9); }
            blk = 25; continue;
        }
        52 => {
            return RegfileExit::jump(0x70008a01c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a01d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(1, c.r64(3).wrapping_mul(2).wrapping_add(0x1f));
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a01dc); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a01ec); };
            c.w32(1, t2);
            let Some(t3) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a01ef); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            let r5 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r5);
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a01f7); };
            c.w32(6, t6);
            let r7 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r7);
            let r8 = c.r32(1) & 0x3fu64;
            c.w32(1, r8);
            c.logic(r8, 32);
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0200); };
            c.w32(7, t9);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r10 = c.add(c.r64(1), 0x69u64, 64);
            c.w64(1, r10);
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008a0215);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a0216); };
            c.w32(1, t11);
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a021a); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a0221); };
            c.w64(7, t13);
            c.w64(8, c.r64(7).wrapping_add(0x18));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a0228); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a022f); };
            c.cmp(t15, c.r64(8), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x70008a0238); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a023f); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008a0244); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0247); };
            c.w64(1, t16);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008a024e); }
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008a0255); };
            c.w64(8, t17);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008a025c); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008a0260); }
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a0263); };
            c.w32(7, t18);
            let r19 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r19);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008a0269); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008a026c); }
            c.w64(0, c.r64(6));
            return c.ret(0x0, 0x70008a0272);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0273); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a0277); }
            let r20 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2dc8)) else { return RegfileExit::fault(0x70008a027d); };
            c.w64(3, t21);
            let Some(t22) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a0284); };
            c.w64(7, t22);
            c.w64(8, c.r64(7).wrapping_add(0x1c));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008a028b); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a028f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a0293); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2dd0)) else { return RegfileExit::fault(0x70008a0297); };
            c.w64(9, t23);
            let Some(t24) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a029e); };
            c.cmp(t24, c.r64(8), 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, 0x1cu64);
            return c.call(0x70007fec74, 0x70008a02ad, 0x70008a02a8);
        }
        8 => {
            blk = 10; continue;
        }
        9 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008a02af); }
            c.w64(3, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 10; continue;
        }
        10 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xed8)) else { return RegfileExit::fault(0x70008a02b9); };
            c.w64(3, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008a02c0); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008a02c5); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008a02c8); }
            if !c.st32(c.r64(0).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70008a02cb); }
            if !c.st32(c.r64(0).wrapping_add(0x13), 0x0u64) { return RegfileExit::fault(0x70008a02d2); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x438)) else { return RegfileExit::fault(0x70008a02d9); };
            c.w64(3, t26);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008a02e0); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2d98)) else { return RegfileExit::fault(0x70008a02e3); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a02ea); };
            c.w64(1, t28);
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008a02ed); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r29 = c.r64(6) & c.r64(0);
            c.w64(6, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a02ff); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008a0309); }
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a030c); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0310); };
            c.w64(1, t32);
            if !c.st32(c.r64(3).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008a0314); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r33 = c.r64(6) & c.r64(3);
            c.w64(6, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a0325); };
            c.logic(t34 & 0x4u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(3, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a032e); };
            c.w64(6, t35);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0335); }
            blk = 1; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a033b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a033f); }
            let r36 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r36);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008a034a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008a034e); }
            return c.call(0x70007febf4, 0x70008a0357, 0x70008a0352);
        }
        16 => {
            c.w64(6, c.r64(0));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a035a); };
            c.w64(3, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a035e); };
            c.w64(1, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0365); }
            blk = 5; continue;
        }
        17 => {
            c.w32(0, c.r32(0));
            let r39 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r39);
            c.w64(6, 0xfffffffffffc0000u64);
            let r40 = c.r64(6) & c.r64(0);
            c.w64(6, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a037a); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0384); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a0385); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008a0392, 0x70008a038d);
        }
        19 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a0392); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a0394); }
            blk = 14; continue;
        }
        20 => {
            c.w32(1, c.r32(1));
            let r42 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r42);
            c.w64(6, 0xfffffffffffc0000u64);
            let r43 = c.r64(6) & c.r64(1);
            c.w64(6, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a03a6); };
            c.logic(t44 & 0x2u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(7, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a03b4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a03b5); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x70008a03c2, 0x70008a03bd);
        }
        22 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a03c2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a03c4); }
            blk = 12; continue;
        }
        23 => {
            return RegfileExit::jump(0x70008a03cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a03d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a03d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a03d8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a03de); };
            c.w32(7, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008a03e1); };
            c.cmp(t2, c.r32(7), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008a03ee); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008a03f5); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0403); };
            c.w32(9, t5);
            let r6 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r6);
            let Some(t7) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a040a); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a0411); };
            c.w32(12, t9);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008a0415); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a0419); }
            c.logic(c.r32(12) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0433); };
            c.w32(12, t10);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(15, c.r64(12));
            blk = 7; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0442); };
            c.w64(15, t11);
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0449); };
            c.w64(12, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0450); };
            c.cmp(t13, c.r32(15), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(3, c.r64(12));
            blk = 24; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0465); };
            c.w32(12, t14);
            let r15 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r15);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008a0472); };
            c.w64(12, t16);
            blk = 11; continue;
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a0479); };
            c.w32(15, t17);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1000u64, 64);
            if c.cc_a() { blk = 93; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(0, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a0496); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a049d); };
            c.w64(2, t19);
            c.w64(1, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a04ac); };
            c.w64(8, t20);
            let Some(t21) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008a04b3); };
            c.cmp(t21, c.r64(6), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008a04bc); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 14; continue;
        }
        14 => {
            let r22 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r22);
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008a04c5); };
            c.w64(2, t23);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008a04cc); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008a04cf); }
            let Some(t24) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a04d2); };
            c.w32(0, t24);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 74; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(0, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008a04e7); }
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a04ea); };
            c.w32(2, t25);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 75; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008a04fd); }
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0500); };
            c.w32(0, t26);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x2u64, 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 17; continue;
        }
        17 => {
            let r27 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r27);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008a0516); }
            let Some(t28) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a051a); };
            c.w32(15, t28);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 77; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008a0534); }
            let Some(t29) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0538); };
            c.w32(15, t29);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 78; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008a054f); }
            c.w64(15, c.r64(3).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(0, c.r64(3).wrapping_add(0x1b));
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0561); };
            c.w64(2, t30);
            blk = 21; continue;
        }
        21 => {
            if !c.st32(c.r64(0), c.r32(2)) { return RegfileExit::fault(0x70008a0568); }
            let r31 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r31);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ne() { blk = 21; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t32) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0573); };
            c.w32(15, t32);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r33 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r33);
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008a058d); };
            c.w32(0, t34);
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)), c.r32(0)) { return RegfileExit::fault(0x70008a0591); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 23; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a059b); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008a05a2); }
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a05a6); };
            c.cmp(t37, c.r32(12), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t38) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a05b3); };
            c.w32(15, t38);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a05bb); };
            c.w32(0, t39);
            c.w64(2, c.r64(15));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008a05d0); };
            c.cmp(t40, c.r32(0), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008a05dd); };
            c.cmp(t41, c.r32(0), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 81; continue; }
            blk = 29; continue;
        }
        29 => {
            let r42 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r42);
            c.w64(8, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a05fd); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008a0604); };
            c.w64(1, t44);
            c.w64(6, c.r64(8).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(1)));
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a060f); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a0616); };
            c.cmp(t46, c.r64(7), 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x70008a061f); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008a0626); }
            c.w64(9, c.r64(7));
            let r47 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r47);
            c.logic(r47, 64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008a0633); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a0636); }
            let Some(t48) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a063a); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(9, c.r64(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(15, 0x7u64);
            blk = 34; continue;
        }
        34 => {
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008a0651); };
            c.w32(0, t49);
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008a0655); }
            let r50 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r50);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 38; continue;
        }
        36 => {
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r51 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            let r52 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r52);
            c.logic(r52, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a0678); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008a067d); };
            c.w64(0, t53);
            c.w64(10, 0x70008a068bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a068b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a068f); }
            return c.call(c.r64(0), 0x70008a0695, 0x70008a0693);
        }
        37 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a0695); }
            let Some(t54) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a069d); };
            c.w64(4, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a06a2); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a06a6); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a06aa); };
            c.w64(7, t57);
            blk = 38; continue;
        }
        38 => {
            c.w64(12, c.r64(7));
            blk = 47; continue;
        }
        39 => {
            c.cmp(c.r64(2), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 87; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(8).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 41; continue;
        }
        41 => {
            let r58 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a06de); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a06e5); };
            c.w64(6, t60);
            c.w64(7, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t61) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a06ec); };
            c.cmp(t61, c.r64(7), 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008a06f6); }
            c.w64(7, c.r64(6).wrapping_add(0x1));
            blk = 43; continue;
        }
        43 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008a06fd); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r62 = c.r64(15) & c.r64(7);
            c.w64(15, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a070a); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_ne() { blk = 97; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008a0715); }
            c.w64(2, c.r64(12).wrapping_add(0x7));
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r64 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r64);
            let r65 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r65);
            c.logic(r65, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008a072c); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a0731); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008a0735); };
            c.w64(0, t66);
            c.w64(10, 0x70008a0743u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008a0743); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008a0747); }
            return c.call(c.r64(0), 0x70008a074d, 0x70008a074b);
        }
        45 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008a074d); }
            let Some(t67) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008a0755); };
            c.w64(4, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a075a); };
            c.w64(12, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a075e); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0762); };
            c.w64(11, t70);
            blk = 47; continue;
        }
        46 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0768); };
            c.w64(12, t71);
            blk = 47; continue;
        }
        47 => {
            c.w64(7, c.r64(11));
            let Some(t72) = c.ld8(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0772); };
            c.w32(9, t72);
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a077b); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a0782); };
            c.w64(15, t74);
            c.w64(2, c.r64(9).wrapping_add(0x8));
            c.w64(0, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a078d); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a0794); };
            c.cmp(t76, c.r64(0), 64);
            if c.cc_be() { blk = 89; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008a079d); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 49; continue;
        }
        49 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008a07a4); }
            c.w64(7, c.r64(3));
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008a07ab); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008a07af); }
            c.w64(7, c.r64(9).wrapping_add(c.r64(11)));
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008a07b7); };
            c.w64(12, t77);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008a07be); }
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a07c2); };
            c.w64(12, t78);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008a07c6); }
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a07ca); };
            c.w32(7, t79);
            let r80 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r80);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008a07d2); }
            c.cmp(c.r64(9), 0xcu64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, 0xcu64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a07e6); };
            c.w64(12, t81);
            c.w64(7, c.r64(2));
            blk = 52; continue;
        }
        51 => {
            c.w64(7, c.r64(0));
            blk = 52; continue;
        }
        52 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a07f2); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t84) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0800); };
            c.w32(0, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a0804); };
            c.cmp(t85, c.r32(0), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(0, c.r64(7).wrapping_add(0x4));
            if !c.st32(c.r64(11).wrapping_add(c.r64(7)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008a0811); }
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ne() { blk = 51; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 70; continue;
        }
        56 => {
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(15, c.r64(7));
            blk = 58; continue;
        }
        58 => {
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0828); };
            c.w32(0, t86);
            if !c.st32(c.r64(11).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008a082d); }
            let r87 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r87);
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ne() { blk = 58; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 61; continue;
        }
        60 => {
            c.w32(12, 0x1u64);
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a0850); };
            c.w64(12, t88);
            blk = 63; continue;
        }
        63 => {
            c.w64(15, c.r64(7).wrapping_sub(0x1));
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(c.r64(7)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a085b); };
            c.w32(0, t89);
            let r90 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r90);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t91) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0867); };
            c.w32(3, t91);
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a086a); };
            c.cmp(t92, c.r32(3), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a0873); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a0878); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008a087f); };
            c.w64(3, t94);
            c.w64(1, c.r64(3).wrapping_add(0xc));
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a0886); };
            c.w64(6, t95);
            let Some(t96) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a088d); };
            c.cmp(t96, c.r64(1), 64);
            if c.cc_be() { blk = 91; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70008a0896); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 67; continue;
        }
        67 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008a089d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a08a1); }
            if !c.st32(c.r64(11).wrapping_add(c.r64(15)), c.r32(0)) { return RegfileExit::fault(0x70008a08a6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r97 = c.r64(3) & c.r64(11);
            c.w64(3, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a08b8); };
            c.logic(t98 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
            blk = 69; continue;
        }
        69 => {
            let r99 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r99);
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_ne() { blk = 63; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a08d1); }
            return c.ret(0x0, 0x70008a08d2);
        }
        71 => {
            blk = 93; continue;
        }
        72 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70008a08d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008a08dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008a08e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008a08e4); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008a08f0, 0x70008a08eb);
        }
        73 => {
            c.w64(3, c.r64(0));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a08f3); };
            c.w64(9, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a08f7); };
            c.w64(7, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a08fb); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a08ff); };
            c.w64(12, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a0903); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0907); };
            c.w64(15, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a090b); };
            c.w64(0, t106);
            blk = 14; continue;
        }
        74 => {
            return RegfileExit::fault(0x70008a0914);
        }
        75 => {
            return RegfileExit::fault(0x70008a0915);
        }
        76 => {
            return RegfileExit::fault(0x70008a0916);
        }
        77 => {
            return RegfileExit::fault(0x70008a0917);
        }
        78 => {
            return RegfileExit::fault(0x70008a0918);
        }
        79 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a0919); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008a091d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008a0921); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008a0925); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008a0931, 0x70008a092c);
        }
        80 => {
            c.w64(7, c.r64(0));
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0934); };
            c.w64(3, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0938); };
            c.w64(11, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a093c); };
            c.w64(12, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a0940); };
            c.w64(0, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a0944); };
            c.w64(8, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a0948); };
            c.w64(2, t112);
            blk = 31; continue;
        }
        81 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008a0955); };
            c.w64(3, t113);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008a0964, 0x70008a095f);
        }
        82 => {
            return RegfileExit::fault(0x70008a0964);
        }
        83 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008a0965); };
            c.w64(3, t114);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a0975, 0x70008a0970);
        }
        84 => {
            return RegfileExit::fault(0x70008a0975);
        }
        85 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a097a); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a097d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a097e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008a0982); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008a0986); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008a098a); }
            c.w32(0, 0x2u64);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008a0993); };
            c.w64(3, t115);
            c.w64(7, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x70008a09a5, 0x70008a09a0);
        }
        86 => {
            c.w64(7, c.r64(0));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a09a8); };
            c.w64(12, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a09ac); };
            c.w64(8, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a09b0); };
            c.w64(0, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a09b4); };
            c.w64(2, t119);
            blk = 43; continue;
        }
        87 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008a09bd); };
            c.w64(3, t120);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a09cd, 0x70008a09c8);
        }
        88 => {
            return RegfileExit::fault(0x70008a09cd);
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008a09ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a09d2); }
            return c.call(0x70007febf4, 0x70008a09db, 0x70008a09d6);
        }
        90 => {
            c.w64(11, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a09de); };
            c.w64(9, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a09e2); };
            c.w64(12, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a09e6); };
            c.w64(3, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a09ea); };
            c.w64(7, t124);
            blk = 49; continue;
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70008a09f3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008a09f7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008a09fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008a09ff); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008a0a03); }
            return c.call(0x70007febf4, 0x70008a0a0d, 0x70008a0a08);
        }
        92 => {
            c.w32(2, 0xcu64);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a0a12); };
            c.w64(11, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a0a16); };
            c.w64(9, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0a1a); };
            c.w64(7, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a0a1e); };
            c.w64(15, t128);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008a0a22); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a0a27); };
            c.w64(12, t129);
            blk = 67; continue;
        }
        93 => {
            let Some(t130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a0a33); };
            c.w64(5, t130);
            let r131 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r131);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008a0a3b); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a0a3c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a0a3d); }
            let Some(t132) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a0a3e); };
            if !c.push64(t132) { return RegfileExit::fault(0x70008a0a3e); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a0a42); }
            c.w32(0, 0x4u64);
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2280)) else { return RegfileExit::fault(0x70008a0a4c); };
            c.w64(3, t133);
            return RegfileExit::jump(0x700086ed74);
        }
        94 => {
            c.w32(0, c.r32(0));
            let r134 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r134);
            c.w64(3, 0xfffffffffffc0000u64);
            let r135 = c.r64(3) & c.r64(0);
            c.w64(3, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a0a67); };
            c.logic(t136 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0a75); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a0a76); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008a0a83, 0x70008a0a7e);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a0a83); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a0a85); }
            blk = 69; continue;
        }
        97 => {
            c.w32(0, c.r32(0));
            let r137 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r137);
            c.w64(15, 0xfffffffffffc0000u64);
            let r138 = c.r64(15) & c.r64(0);
            c.w64(15, r138);
            c.logic(r138, 64);
            let Some(t139) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a0a9a); };
            c.logic(t139 & 0x2u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(1, c.r64(7).wrapping_sub(0x1));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a0aa9); }
            c.w64(8, c.r64(1));
            return c.call(0x70007f13b4, 0x70008a0ab3, 0x70008a0aae);
        }
        99 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a0ab3); }
            blk = 44; continue;
        }
        100 => {
            return RegfileExit::jump(0x70008a0abc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a0ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0ad4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a0ad8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0ad9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a0ada); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a0adf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(5));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a0aef); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0af3); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.call(0x70007f9c54, 0x70008a0b05, 0x70008a0b00);
        }
        4 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0b05); };
            c.w64(1, t3);
            let r4 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r4);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0b10); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0b11); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0b17); }
            return c.ret(0x0, 0x70008a0b19);
        }
        5 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a0b20); };
            c.w64(8, t5);
            c.w64(9, c.r64(8));
            blk = 8; continue;
        }
        7 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0b29); };
            c.w64(9, t6);
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0b30); };
            c.w64(8, t7);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_le() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0b3d); };
            c.cmp(t8, c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a0b46); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2054, 0x70008a0b57, 0x70008a0b52);
        }
        12 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0b57); };
            c.w64(1, t10);
            let r11 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0b62); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0b63); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0b69); }
            return c.ret(0x0, 0x70008a0b6b);
        }
        13 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0b6c); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a0b73); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0b7a); };
            c.w32(11, t16);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8cu64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008a0b8d);
        }
        15 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008a0b8e); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a0b98); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a0b9f); };
            c.w64(11, t21);
            let Some(t22) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a0ba6); };
            c.w64(12, t22);
            let Some(t23) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0ba9); };
            c.w32(15, t23);
            c.w64(2, c.r64(15).wrapping_mul(4));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a0bba); };
            c.w64(0, t24);
            let Some(t25) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008a0bc1); };
            c.cmp(t25, c.r64(15), 64);
            if c.cc_be() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008a0bca); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 17; continue;
        }
        17 => {
            c.w64(12, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008a0bd6); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0bda); };
            c.w64(9, t26);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008a0be1); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008a0be5); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let r27 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r27);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(8)) { return RegfileExit::fault(0x70008a0bf3); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 19; continue;
        }
        19 => {
            let r28 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r28);
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0c07); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0c08); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0c0e); }
            return c.ret(0x0, 0x70008a0c10);
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008a0c11); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a0c15); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008a0c19); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a0c1d); }
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a0c21); };
            c.w64(3, t29);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a0c32, 0x70008a0c2d);
        }
        21 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a0c32); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0c36); };
            c.w64(7, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0c3a); };
            c.w64(2, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0c3e); };
            c.w64(0, t33);
            blk = 1; continue;
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a0c47); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008a0c4b); }
            return c.call(0x70007febf4, 0x70008a0c54, 0x70008a0c4f);
        }
        23 => {
            c.w64(11, c.r64(0));
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0c57); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0c5b); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a0c62); };
            c.w64(9, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0c66); };
            c.w64(2, t37);
            blk = 17; continue;
        }
        24 => {
            return RegfileExit::jump(0x70008a0c70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a0c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0c74); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a0c7a); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0c80); };
            c.w32(1, t4);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x8cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008a0c92);
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008a0c93); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let Some(t7) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a0c9c); };
            c.w32(3, t7);
            let r8 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a0ca2); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a0ca9); };
            c.w64(6, t10);
            let Some(t11) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0cac); };
            c.w32(7, t11);
            c.w64(2, c.r64(7).wrapping_mul(4));
            c.w64(7, c.r64(2).wrapping_add(c.r64(6)));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a0cbc); };
            c.w64(8, t12);
            let Some(t13) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008a0cc3); };
            c.cmp(t13, c.r64(7), 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008a0cc8); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            c.w64(6, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008a0cd3); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a0cd6); };
            c.w64(3, t14);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008a0cdd); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008a0ce0); }
            c.cmp(c.r64(6), 0xbu64, 64);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0ce9); };
            c.w64(3, t15);
            blk = 6; continue;
        }
        6 => {
            let r16 = c.sub(c.r64(6), 0x4u64, 64);
            c.w64(6, r16);
            if !c.st32(c.r64(6).wrapping_add(c.r64(1)), c.r32(3)) { return RegfileExit::fault(0x70008a0cf4); }
            c.cmp(c.r64(6), 0xbu64, 64);
            if c.cc_ne() { blk = 6; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008a0d00);
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0d01); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a0d05); }
            let r17 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r17);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008a0d0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008a0d0f); }
            return c.call(0x70007febf4, 0x70008a0d18, 0x70008a0d13);
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a0d1b); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a0d1f); };
            c.w64(2, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0d26); }
            blk = 4; continue;
        }
        10 => {
            return RegfileExit::jump(0x70008a0d2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a0d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0d34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a0d38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0d39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a0d3a); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a0d3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(5));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 7; continue;
        }
        3 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a0d60); };
            c.w64(9, t2);
            blk = 6; continue;
        }
        5 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0d66); };
            c.w64(9, t3);
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            let r4 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a0d7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a0d7f); }
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a0d83); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t6) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a0d8c); };
            c.w64(8, t6);
            c.w64(7, c.r64(8));
            return c.call(0x70007f9c54, 0x70008a0d98, 0x70008a0d93);
        }
        9 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a0d98); };
            c.w64(1, t7);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008a0d9c); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r8 = c.r64(8) & c.r64(0);
            c.w64(8, r8);
            c.logic(r8, 64);
            let Some(t9) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a0dae); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0db9); };
            c.w64(8, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0dc0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0dc1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0dc7); }
            return c.ret(0x0, 0x70008a0dc9);
        }
        12 => {
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0dd0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0dd1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0dd7); }
            return c.ret(0x0, 0x70008a0dd9);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008a0dda); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008a0dde); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008a0de2); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a0de6); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a0df7, 0x70008a0df2);
        }
        14 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0df7); };
            c.w64(2, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0dfb); };
            c.w64(6, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0dff); };
            c.w64(0, t14);
            blk = 1; continue;
        }
        15 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0e08); };
            c.w32(11, t15);
            let r16 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008a0e0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008a0e13); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008a0e17); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a0e1b); }
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a0e1f); };
            c.cmp(t17, c.r32(11), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t18) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a0e2c); };
            c.w32(11, t18);
            c.cmp(c.r16(11), 0xbau64, 16);
            if c.cc_b() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a0e41, 0x70008a0e3c);
        }
        18 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0e49); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a0e4f); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a0e5c); };
            c.w32(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0e61); };
            c.w64(1, t23);
            c.w64(9, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0e68); };
            c.w64(2, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0e6c); };
            c.w64(6, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a0e70); };
            c.w64(8, t26);
            blk = 21; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a0e74); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a0e7b); };
            c.w32(12, t29);
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_b() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r16(12), 0x43u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), 0x41u64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a0e95); }
            c.w32(0, 0x1u64);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a0e9c); };
            c.w64(3, t30);
            return c.call(0x700086ed74, 0x70008a0ea8, 0x70008a0ea3);
        }
        25 => {
            blk = 33; continue;
        }
        26 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a0eaa); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1d60)) else { return RegfileExit::fault(0x70008a0eac); };
            c.w64(3, t31);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a0ebd, 0x70008a0eb8);
        }
        27 => {
            blk = 33; continue;
        }
        28 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a0ebf); };
            c.w32(9, t32);
            let r33 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r33);
            c.w64(0, c.r64(9));
            blk = 33; continue;
        }
        29 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0ecb); };
            c.w32(11, t34);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r35 = c.r64(11) & 0xffffffu64;
            c.w64(11, r35);
            c.logic(r35, 64);
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(0, c.r64(9));
            blk = 33; continue;
        }
        31 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a0eec); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a0eee); };
            c.w64(3, t36);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a0f01, 0x70008a0efc);
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0f01); };
            c.w64(1, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0f05); };
            c.w64(2, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0f09); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a0f0d); };
            c.w64(8, t40);
            blk = 33; continue;
        }
        33 => {
            c.w64(9, c.r64(0));
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a0f14); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a0f18); };
            c.w64(6, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a0f1c); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a0f20); };
            c.w64(1, t44);
            blk = 7; continue;
        }
        34 => {
            c.w32(1, c.r32(1));
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            c.w64(8, 0xfffffffffffc0000u64);
            let r46 = c.r64(8) & c.r64(1);
            c.w64(8, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a0f38); };
            c.logic(t47 & 0x2u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(9, c.r64(0).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0f47); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a0f48); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008a0f55, 0x70008a0f50);
        }
        36 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a0f55); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a0f57); }
            blk = 11; continue;
        }
        37 => {
            return RegfileExit::jump(0x70008a0f60);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a0f74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0f74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a0f78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a0f79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a0f7a); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a0f7b); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a0f81); };
            c.w64(0, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a0f85); };
            c.w64(1, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a0f8c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x10, 0x70008a0f93);
        }
        3 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a0f96); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a0f9d); }
            return c.ret(0x0, 0x70008a0f9f);
        }
        4 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a0fa0); };
            c.w64(3, t3);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a0fae, 0x70008a0fa9);
        }
        5 => {
            blk = 1; continue;
        }
        6 => {
            return RegfileExit::jump(0x70008a0fb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a0fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a0fd4); };
            c.w32(2, t0);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r1 = c.r64(2) & 0xffffffu64;
            c.w64(2, r1);
            c.logic(r1, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            return c.ret(0x0, 0x70008a0fed);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a0fee); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a0ff2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a0ff4); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a0ff5); };
            c.w64(3, t2);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a1008, 0x70008a1003);
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a100b); }
            return c.ret(0x0, 0x70008a100c);
        }
        4 => {
            return RegfileExit::jump(0x70008a1010);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a1014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1014); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a101a); };
            c.cmp(t2, 0xbau64, 16);
            if c.cc_ae() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1022); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1028); };
            c.w32(2, t5);
            c.cmp(c.r16(2), 0x40u64, 16);
            if c.cc_b() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(2), 0x43u64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a1038); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            return c.ret(0x0, 0x70008a103e);
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a103f); };
            c.w32(2, t8);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r9 = c.r64(2) & 0xffffffu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            return c.ret(0x0, 0x70008a1058);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1059); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a105d); }
            let r10 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a1063); }
            return c.call(0x70008a2474, 0x70008a106c, 0x70008a1067);
        }
        7 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1070); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1076); };
            c.cmp(t13, c.r32(2), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a1082); }
            return c.ret(0x0, 0x70008a1083);
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1084); };
            c.w64(6, t14);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a108b); }
            blk = 1; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a108e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1092); }
            let r15 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a1098); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a1099); };
            c.w64(3, t16);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a10aa, 0x70008a10a5);
        }
        12 => {
            blk = 9; continue;
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a10ac); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a10b0); }
            let r17 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a10b6); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a10b7); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a10ca, 0x70008a10c5);
        }
        14 => {
            blk = 9; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008a10d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a10d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a10d4); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a10da); };
            c.w32(2, t2);
            c.cmp(c.r16(2), 0xbau64, 16);
            if c.cc_ae() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a10e5); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a10eb); };
            c.w32(1, t5);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(2), 0x41u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x0, 0x70008a1104);
        }
        5 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a1105); };
            c.w32(0, t6);
            let r7 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r7);
            return c.ret(0x0, 0x70008a110b);
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a110c); };
            c.w32(2, t8);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 7; continue;
        }
        7 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r9 = c.r64(2) & 0xffffffu64;
            c.w64(2, r9);
            c.logic(r9, 64);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            return c.ret(0x0, 0x70008a1129);
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a112a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a112e); }
            let r10 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a1134); }
            return c.call(0x70008a2474, 0x70008a113d, 0x70008a1138);
        }
        9 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1145); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a114b); };
            c.cmp(t13, c.r32(2), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(1, 0x1u64);
            blk = 13; continue;
        }
        12 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            let Some(t14) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a115d); };
            c.w32(2, t14);
            c.cmp(c.r16(2), 0x41u64, 16);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let r15 = c.r32(7) | c.r32(1);
            c.w32(7, r15);
            c.logic(r15, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(2, c.r16(2));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1174); };
            c.w64(6, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a117b); }
            blk = 1; continue;
        }
        15 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1181); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1185); }
            let r17 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a118b); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x24c0)) else { return RegfileExit::fault(0x70008a118c); };
            c.w64(3, t18);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a119d, 0x70008a1198);
        }
        16 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a11a0); }
            return c.ret(0x0, 0x70008a11a1);
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a11a2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a11a6); }
            let r19 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r19);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a11ac); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a11ad); };
            c.w64(3, t20);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008a11c0, 0x70008a11bb);
        }
        18 => {
            blk = 16; continue;
        }
        19 => {
            blk = 16; continue;
        }
        20 => {
            return RegfileExit::jump(0x70008a11c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a11d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a11d8); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a11db); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x0, 0x70008a11e4);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a11e5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a11e9); }
            return c.call(0x70008a10d4, 0x70008a11f0, 0x70008a11eb);
        }
        4 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a11f3); }
            return c.ret(0x0, 0x70008a11f4);
        }
        5 => {
            return RegfileExit::jump(0x70008a11f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a1214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1214); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1218); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x70008a121e); };
            c.w64(1, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1225); };
            c.w32(7, t2);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            let r3 = c.shr(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r3);
            let r4 = c.sub(c.r32(8), 0x1u64, 32);
            c.w32(8, r4);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a123c); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 48; continue; }
            blk = 2; continue;
        }
        2 => {
            if c.cc_ne() { blk = 48; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(9));
            let r5 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r5);
            if c.cc_o() { blk = 48; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 8; continue;
        }
        7 => {
            c.w64(9, c.r64(0));
            blk = 8; continue;
        }
        8 => {
            c.w64(11, c.r64(9));
            let r6 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r6);
            c.w32(12, c.r32(9));
            let r7 = c.r32(8) & c.r32(11);
            c.w32(8, r7);
            c.logic(r7, 32);
            let r8 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r8);
            c.w64(15, c.r64(8).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008a12a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008a12a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008a12ab); }
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a12af); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a12b6); };
            c.w32(0, t10);
            let r11 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r11);
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a12c1); }
            return c.ret(0x0, 0x70008a12c2);
        }
        11 => {
            c.cmp(c.r64(7), 0x8000u64, 64);
            if c.cc_l() { blk = 71; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(7, (c.cc_l() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(3, 0x80000000u64);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 14; continue;
        }
        14 => {
            let r12 = c.neg(c.r32(11), 32);
            c.w32(11, r12);
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            blk = 19; continue;
        }
        18 => {
            c.w32(3, 0x1u64);
            blk = 19; continue;
        }
        19 => {
            c.w64(6, c.r64(11));
            c.w32(0, 0x66666667u64);
            blk = 21; continue;
        }
        20 => {
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(6));
            c.imul1(c.r32(9), 32);
            let r13 = c.shr(c.r32(6), 0x1fu64, 32).unwrap_or(c.r32(6));
            c.w32(6, r13);
            let r14 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r14);
            let r15 = c.add(c.r32(6), c.r32(2), 32);
            c.w32(6, r15);
            let r16 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r16);
            c.w64(0, c.r64(9));
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_g() { blk = 20; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(9, c.r64(0));
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r17 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r17);
            if c.cc_o() { blk = 40; continue; }
            blk = 23; continue;
        }
        23 => {
            let r18 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r18);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a134d); };
            c.w64(6, t19);
            let Some(t20) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a1354); };
            c.w64(6, t20);
            let r21 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r21);
            c.logic(r21, 64);
            c.w64(15, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a135f); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008a1366); };
            c.cmp(t23, c.r64(15), 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a137c); };
            c.w64(8, t24);
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70008a1383); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 26; continue;
        }
        26 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008a138a); };
            c.w64(15, t25);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008a1391); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1395); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008a139d); }
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            blk = 28; continue;
        }
        27 => {
            if !c.st8(c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008a13a7); }
            let r26 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r26);
            c.w64(11, c.r64(0));
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_le() { blk = 33; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a13b9); };
            c.w64(0, (t27 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ae() { blk = 39; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(9));
            c.imul1(c.r32(11), 32);
            c.w64(0, c.r64(11));
            let r28 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r28);
            let r29 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r29);
            let r30 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r30);
            let r31 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r31);
            let r32 = c.sub(c.r32(11), c.r32(2), 32);
            c.w32(11, r32);
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let r33 = c.add(c.r32(11), 0x57u64, 32);
            c.w32(11, r33);
            blk = 27; continue;
        }
        32 => {
            let r34 = c.add(c.r32(11), 0x30u64, 32);
            c.w32(11, r34);
            blk = 27; continue;
        }
        33 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let r35 = c.shl(c.r32(3), 0x1au64, 32).unwrap_or(c.r32(3));
            c.w32(3, r35);
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let r36 = c.r32(3) | c.r32(7);
            c.w32(3, r36);
            c.logic(r36, 32);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008a13f9); }
            blk = 38; continue;
        }
        35 => {
            let Some(t37) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a13ff); };
            c.w64(7, (t37 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008a1409);
        }
        37 => {
            if !c.st8(c.r64(8).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x70008a140a); }
            blk = 38; continue;
        }
        38 => {
            c.w64(0, c.r64(8));
            blk = 43; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008a1414);
        }
        40 => {
            return RegfileExit::fault(0x70008a1415);
        }
        41 => {
            c.w32(7, c.r64(11).wrapping_add(0x30));
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 55; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008a142e); };
            c.w64(12, t38);
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1435); };
            c.w32(0, t39);
            let r40 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r40);
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a143d); };
            c.cmp(t41, c.r32(0), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a144a); };
            c.w32(7, t42);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a1453); };
            c.w64(8, t43);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 69; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a1460); };
            c.w64(7, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a1464); };
            c.w64(9, t45);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008a1468); }
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a146d); };
            c.w32(7, t46);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r47 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r47);
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 70; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008a148b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r48 = c.r64(8) & c.r64(1);
            c.w64(8, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a149c); };
            c.logic(t49 & 0x4u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 10; continue;
        }
        48 => {
            let Some(t50) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a14ac); };
            c.w32(7, t50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a14af); };
            c.w32(9, t51);
            c.w64(11, c.r64(7));
            let r52 = c.r32(11) ^ c.r32(9);
            c.w32(11, r52);
            c.logic(r52, 32);
            let r53 = c.r32(11) & c.r32(8);
            c.w32(11, r53);
            c.logic(r53, 32);
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            if let Some(sr) = c.shl(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a14c4); };
            c.w32(11, t54);
            let r55 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r55);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t56) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a14d6); };
            c.w32(12, t56);
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a14da); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t58) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a14e7); };
            c.w32(12, t58);
            let Some(t59) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a14eb); };
            c.cmp(t59, c.r32(7), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t60) = c.ld32(c.r64(1).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a14fe); };
            c.w32(0, t60);
            let r61 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r61);
            blk = 10; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008a150b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008a150f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008a1513); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a1517); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008a151b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008a151f); }
            return c.call(0x70007febb4, 0x70008a1528, 0x70008a1523);
        }
        54 => {
            c.w64(8, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a152b); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a152f); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a1533); };
            c.w64(3, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a1537); };
            c.w64(7, t65);
            c.w32(9, 0x66666667u64);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a1541); };
            c.w64(11, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a1545); };
            c.w64(0, t67);
            blk = 26; continue;
        }
        55 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a154e); };
            c.w64(11, t68);
            let Some(t69) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a1555); };
            c.w64(12, t69);
            c.w64(0, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008a155d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a1561); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a1565); };
            c.w64(3, t70);
            let Some(t71) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a156c); };
            c.cmp(t71, c.r64(0), 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008a157b, 0x70008a1576);
        }
        57 => {
            blk = 59; continue;
        }
        58 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008a157d); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 59; continue;
        }
        59 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008a1588); };
            c.w64(1, t72);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a158f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008a1592); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1599); }
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a15a0); };
            c.w64(7, t73);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008a15a4); }
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a15a8); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a15ac); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a15b0); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a15b4); };
            c.w64(9, t77);
            blk = 43; continue;
        }
        60 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a15bd); };
            c.w64(0, t78);
            let Some(t79) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008a15c4); };
            c.w64(3, t79);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008a15cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008a15cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a15d3); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a15d7); };
            c.w64(6, t80);
            let Some(t81) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a15de); };
            c.cmp(t81, c.r64(2), 64);
            if c.cc_a() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008a15ed, 0x70008a15e8);
        }
        62 => {
            blk = 64; continue;
        }
        63 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008a15ef); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008a15f6); };
            c.w64(1, t82);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a15fd); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008a1600); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1607); }
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a160e); };
            c.w64(1, t83);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008a1612); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008a1615); };
            c.w64(12, t84);
            let Some(t85) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a161c); };
            c.w32(7, t85);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return RegfileExit::fault(0x70008a162c);
        }
        66 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a162d); };
            c.w64(11, t86);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008a1631); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r87 = c.r64(1) & c.r64(12);
            c.w64(1, r87);
            c.logic(r87, 64);
            let Some(t88) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a1643); };
            c.logic(t88 & 0x4u64, 8);
            if c.cc_ne() { blk = 73; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a1649); };
            c.w64(8, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a164d); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a1651); };
            c.w64(15, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a1655); };
            c.w64(9, t92);
            blk = 43; continue;
        }
        69 => {
            return RegfileExit::fault(0x70008a165e);
        }
        70 => {
            return RegfileExit::fault(0x70008a165f);
        }
        71 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a1660); }
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x70008a1661); };
            c.w64(3, t93);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a1674, 0x70008a166f);
        }
        72 => {
            blk = 10; continue;
        }
        73 => {
            c.w32(0, c.r32(0));
            let r94 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r94);
            c.w64(1, 0xfffffffffffc0000u64);
            let r95 = c.r64(1) & c.r64(0);
            c.w64(1, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a1688); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a1692); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a1693); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008a16a0, 0x70008a169b);
        }
        75 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a16a0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a16a2); }
            blk = 68; continue;
        }
        76 => {
            c.w32(0, c.r32(0));
            let r97 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r97);
            c.w64(8, 0xfffffffffffc0000u64);
            let r98 = c.r64(8) & c.r64(0);
            c.w64(8, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a16b4); };
            c.logic(t99 & 0x2u64, 8);
            if c.cc_e() { blk = 47; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a16c3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a16c4); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008a16d1, 0x70008a16cc);
        }
        78 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a16d1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a16d3); }
            blk = 47; continue;
        }
        79 => {
            return RegfileExit::jump(0x70008a16dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a16f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a16f4); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a1703); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a170c); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a1712); };
            c.logic(t4 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1718); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1721); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a172f); };
            c.w64(0, t7);
            return c.ret(0x0, 0x70008a1736);
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a1737); };
            c.w64(0, t8);
            return c.ret(0x0, 0x70008a173e);
        }
        10 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a173f); };
            c.logic(t9 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a174a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008a175c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a1774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a1774); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a1783); };
            c.cmp(t1, c.r32(0), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a178c); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a1792); };
            c.logic(t4 & 0x10u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1798); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a17a1); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 8; continue;
        }
        7 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(2, 0x2u64);
            return c.ret(0x0, 0x70008a17b4);
        }
        9 => {
            c.w32(2, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008a17b7);
        }
        10 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a17b8); };
            c.logic(t7 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 8; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a17c3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 8; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008a17d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a17f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a17ff); };
            c.w32(8, t0);
            let r1 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1806); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a180f); };
            c.cmp(t3, 0xbau64, 16);
            if c.cc_ae() { blk = 32; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a181c); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1823); };
            c.w32(8, t6);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a183a); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            blk = 1; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1842); };
            c.w32(8, t9);
            c.logic(c.r32(8) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 8; continue;
        }
        8 => {
            if let Some(sr) = c.shr(c.r64(8), 0x2u64, 64) { c.w64(8, sr); }
            let r10 = c.r64(8) & 0xffffffu64;
            c.w64(8, r10);
            c.logic(r10, 64);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 1; continue;
        }
        9 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x0, 0x70008a1869);
        }
        11 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008a186d);
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a186e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            c.w8(1, (c.cc_a() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 30; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 24; continue; }
            blk = 16; continue;
        }
        16 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_be() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a18cb); };
            c.w64(10, t11);
            if !c.xorpd_rm(0, c.r64(10)) { return RegfileExit::fault(0x70008a18d2); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(1, 10);
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_be() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(2));
            c.mov128_rr(2, 0);
            blk = 19; continue;
        }
        19 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008a190b); };
            c.w64(10, t12);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008a1912); }
            c.mov128_rr(0, 2);
            blk = 24; continue;
        }
        20 => {
            let Some(t13) = c.ld64(0x70008a18d8u64) else { return RegfileExit::fault(0x70008a191b); };
            c.w64(10, t13);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.mov128_rr(0, 2);
            blk = 24; continue;
        }
        23 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(0));
            c.mov128_rr(0, 2);
            blk = 24; continue;
        }
        24 => {
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 41; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_ne() { blk = 41; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(7, c.r64(1));
            let r14 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r14);
            if c.cc_o() { blk = 42; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008a1999);
        }
        30 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a199a); };
            c.w64(0, t15);
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x19f)) else { return RegfileExit::fault(0x70008a19a1); };
            c.w32(0, t16);
            let r17 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r17);
            return c.ret(0x0, 0x70008a19aa);
        }
        31 => {
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008a19ae);
        }
        32 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a19af); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a19b3); }
            let r18 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a19b9); }
            return c.call(0x70008a2474, 0x70008a19c2, 0x70008a19bd);
        }
        33 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a19c6); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a19cc); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a19dc); };
            c.w64(6, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a19e3); }
            blk = 1; continue;
        }
        36 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a19f0); };
            c.w64(6, t23);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a19f7); }
            blk = 4; continue;
        }
        37 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a19fd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1a01); }
            let r24 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r24);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a1a07); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a1a08); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a1a0c); };
            c.w64(3, t25);
            c.w64(0, c.r64(1));
            return c.call(0x700086ed74, 0x70008a1a1b, 0x70008a1a16);
        }
        38 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1a22); };
            c.w64(6, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a1a29); }
            blk = 1; continue;
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1a2f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1a33); }
            let r27 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r27);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a1a39); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a1a3a); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a1a3e); };
            c.w64(3, t28);
            c.w64(0, c.r64(1));
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008a1a50, 0x70008a1a4b);
        }
        40 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1a57); };
            c.w64(6, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a1a5e); }
            blk = 1; continue;
        }
        41 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1a64); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1a68); }
            let r30 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r30);
            blk = 44; continue;
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1a70); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1a74); }
            let r31 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r31);
            blk = 44; continue;
        }
        43 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1a7c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1a80); }
            let r32 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r32);
            blk = 44; continue;
        }
        44 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a1a86); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a1a8d); };
            c.w64(7, t34);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008a1a94); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a1a99); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a1aa0); };
            c.cmp(t36, c.r64(8), 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a1aaf, 0x70008a1aaa);
        }
        46 => {
            blk = 48; continue;
        }
        47 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008a1ab1); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 48; continue;
        }
        48 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1abb); };
            c.w64(1, t37);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a1ac2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008a1ac5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a1aca); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a1ad2); }
            return c.ret(0x0, 0x70008a1ad3);
        }
        49 => {
            return RegfileExit::jump(0x70008a1ad8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a1af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a1af4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a1af8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a1afe); }
            blk = 3; continue;
        }
        1 => {
            return c.call(0x70008a2574, 0x70008a1b09, 0x70008a1b04);
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1b09); };
            c.w64(6, t1);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1b1a); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1b20); };
            c.cmp(t4, 0x40u64, 16);
            if c.cc_be() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1b2c); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1b32); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1b3b); };
            c.w32(1, t8);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 59; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x43u64, 32);
            if c.cc_ne() { blk = 1; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 58; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a1b59); }
            return c.ret(0x0, 0x70008a1b5a);
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x10f0)) else { return RegfileExit::fault(0x70008a1b5b); };
            c.w64(7, t9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1b62); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            let r11 = c.shr(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r11);
            let r12 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r12);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a1b7b); }
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 13; continue;
        }
        13 => {
            if c.cc_ne() { blk = 60; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(11, c.r64(1));
            let r13 = c.add(c.r32(11), c.r32(1), 32);
            c.w32(11, r13);
            if c.cc_o() { blk = 60; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(1, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 19; continue;
        }
        18 => {
            c.w64(1, c.r64(0));
            blk = 19; continue;
        }
        19 => {
            c.w64(11, c.r64(1));
            let r14 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r14);
            c.w32(12, c.r32(1));
            let r15 = c.r32(9) & c.r32(11);
            c.w32(9, r15);
            c.logic(r15, 32);
            let r16 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r16);
            c.w64(15, c.r64(9).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a1bdf); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a1be3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008a1be7); }
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1beb); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a1bf2); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            blk = 10; continue;
        }
        21 => {
            c.cmp(c.r64(8), 0x8000u64, 64);
            if c.cc_l() { blk = 83; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(8, (c.cc_l() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(3, 0x80000000u64);
            c.cmp(c.r32(3), c.r32(11), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 24; continue;
        }
        24 => {
            let r20 = c.neg(c.r32(11), 32);
            c.w32(11, r20);
            blk = 26; continue;
        }
        25 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_l() { blk = 51; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            blk = 29; continue;
        }
        28 => {
            c.w32(3, 0x1u64);
            blk = 29; continue;
        }
        29 => {
            c.w64(6, c.r64(11));
            c.w32(0, 0x66666667u64);
            blk = 31; continue;
        }
        30 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(6));
            c.imul1(c.r32(1), 32);
            let r21 = c.shr(c.r32(6), 0x1fu64, 32).unwrap_or(c.r32(6));
            c.w32(6, r21);
            let r22 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r22);
            let r23 = c.add(c.r32(6), c.r32(2), 32);
            c.w32(6, r23);
            let r24 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r24);
            c.w64(0, c.r64(1));
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_g() { blk = 30; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(1, c.r64(0));
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r25 = c.add(c.r64(2), c.r64(0), 64);
            c.w64(2, r25);
            if c.cc_o() { blk = 50; continue; }
            blk = 33; continue;
        }
        33 => {
            let r26 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a1c8a); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a1c91); };
            c.w64(6, t28);
            let r29 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r29);
            c.logic(r29, 64);
            c.w64(15, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a1c9c); };
            c.w64(9, t30);
            let Some(t31) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a1ca3); };
            c.cmp(t31, c.r64(15), 64);
            if c.cc_be() { blk = 65; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 65; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a1cb9); };
            c.w64(9, t32);
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008a1cc0); }
            c.w64(9, c.r64(6).wrapping_add(0x1));
            blk = 36; continue;
        }
        36 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008a1cc7); };
            c.w64(15, t33);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008a1cce); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1cd2); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008a1cda); }
            c.w64(15, c.r64(0).wrapping_sub(0x1));
            blk = 38; continue;
        }
        37 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(15)).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008a1ce4); }
            let r34 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r34);
            c.w64(11, c.r64(0));
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_le() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t35) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1cf6); };
            c.w64(0, (t35 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_ae() { blk = 49; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            c.imul1(c.r32(11), 32);
            c.w64(0, c.r64(11));
            let r36 = c.shr(c.r32(0), 0x1fu64, 32).unwrap_or(c.r32(0));
            c.w32(0, r36);
            let r37 = c.sar(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r37);
            let r38 = c.add(c.r32(0), c.r32(2), 32);
            c.w32(0, r38);
            let r39 = c.imul2(c.r32(0), 0xau64, 32);
            c.w32(2, r39);
            let r40 = c.sub(c.r32(11), c.r32(2), 32);
            c.w32(11, r40);
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let r41 = c.add(c.r32(11), 0x57u64, 32);
            c.w32(11, r41);
            blk = 37; continue;
        }
        42 => {
            let r42 = c.add(c.r32(11), 0x30u64, 32);
            c.w32(11, r42);
            blk = 37; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r43 = c.shl(c.r32(3), 0x1au64, 32).unwrap_or(c.r32(3));
            c.w32(3, r43);
            c.w32(1, c.r64(12).wrapping_add(c.r64(12)));
            let r44 = c.r32(3) | c.r32(1);
            c.w32(3, r44);
            c.logic(r44, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008a1d37); }
            blk = 48; continue;
        }
        45 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1d3d); };
            c.w64(1, (t45 as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_a() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return RegfileExit::fault(0x70008a1d47);
        }
        47 => {
            if !c.st8(c.r64(9).wrapping_add(0xb), 0x2du64) { return RegfileExit::fault(0x70008a1d48); }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(9));
            blk = 53; continue;
        }
        49 => {
            return RegfileExit::fault(0x70008a1d52);
        }
        50 => {
            return RegfileExit::fault(0x70008a1d53);
        }
        51 => {
            c.w32(8, c.r64(11).wrapping_add(0x30));
            c.cmp(c.r32(8), 0xffu64, 32);
            if c.cc_g() { blk = 67; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(11, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008a1d6d); };
            c.w64(12, t46);
            let Some(t47) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1d74); };
            c.w32(0, t47);
            let r48 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r48);
            let Some(t49) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a1d7c); };
            c.cmp(t49, c.r32(0), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t50) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1d89); };
            c.w32(1, t50);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a1d92); };
            c.w64(8, t51);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 81; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a1d9f); };
            c.w64(1, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a1da3); };
            c.w64(9, t53);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008a1da7); }
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1dac); };
            c.w32(1, t54);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r55 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r55);
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 82; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008a1dca); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r56 = c.r64(8) & c.r64(7);
            c.w64(8, r56);
            c.logic(r56, 64);
            let Some(t57) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a1ddb); };
            c.logic(t57 & 0x4u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 10; continue;
        }
        58 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a1deb); };
            c.w32(0, t58);
            let r59 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r59);
            blk = 10; continue;
        }
        59 => {
            let Some(t60) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a1df6); };
            c.w64(5, t60);
            let r61 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r61);
            let Some(t62) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008a1dfe); };
            if !c.push64(t62) { return RegfileExit::fault(0x70008a1dfe); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x24d0)) else { return RegfileExit::fault(0x70008a1e01); };
            c.w64(3, t63);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008a1e08); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        60 => {
            let Some(t64) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1e17); };
            c.w32(1, t64);
            let Some(t65) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1e1a); };
            c.w32(8, t65);
            c.w64(11, c.r64(1));
            let r66 = c.r32(11) ^ c.r32(8);
            c.w32(11, r66);
            c.logic(r66, 32);
            let r67 = c.r32(11) & c.r32(9);
            c.w32(11, r67);
            c.logic(r67, 32);
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1e2f); };
            c.w32(11, t68);
            let r69 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r69);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t70) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a1e41); };
            c.w32(12, t70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a1e45); };
            c.cmp(t71, c.r32(12), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t72) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a1e52); };
            c.w32(12, t72);
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1e56); };
            c.cmp(t73, c.r32(1), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(12), c.r32(8), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t74) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a1e69); };
            c.w32(0, t74);
            let r75 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r75);
            blk = 10; continue;
        }
        65 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a1e76); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008a1e7a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008a1e7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008a1e82); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008a1e86); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008a1e8a); }
            return c.call(0x70007febb4, 0x70008a1e93, 0x70008a1e8e);
        }
        66 => {
            c.w64(9, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1e96); };
            c.w64(7, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a1e9a); };
            c.w64(12, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a1e9e); };
            c.w64(3, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a1ea2); };
            c.w64(8, t79);
            c.w32(1, 0x66666667u64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a1eab); };
            c.w64(11, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a1eaf); };
            c.w64(0, t81);
            blk = 36; continue;
        }
        67 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a1eb8); };
            c.w64(11, t82);
            let Some(t83) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a1ebf); };
            c.w64(12, t83);
            c.w64(0, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a1ec7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a1ecb); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a1ecf); };
            c.w64(3, t84);
            let Some(t85) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008a1ed6); };
            c.cmp(t85, c.r64(0), 64);
            if c.cc_a() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008a1ee5, 0x70008a1ee0);
        }
        69 => {
            blk = 71; continue;
        }
        70 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008a1ee7); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 71; continue;
        }
        71 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008a1ef2); };
            c.w64(1, t86);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a1ef9); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008a1efc); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1f03); }
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a1f0a); };
            c.w64(8, t87);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(8)) { return RegfileExit::fault(0x70008a1f0e); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a1f13); };
            c.w64(9, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1f17); };
            c.w64(7, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a1f1b); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a1f1f); };
            c.w64(1, t91);
            blk = 53; continue;
        }
        72 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a1f28); };
            c.w64(0, t92);
            let Some(t93) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008a1f2f); };
            c.w64(3, t93);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a1f36); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008a1f3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a1f3e); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a1f42); };
            c.w64(6, t94);
            let Some(t95) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008a1f49); };
            c.cmp(t95, c.r64(2), 64);
            if c.cc_a() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008a1f58, 0x70008a1f53);
        }
        74 => {
            blk = 76; continue;
        }
        75 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008a1f5a); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 76; continue;
        }
        76 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008a1f61); };
            c.w64(1, t96);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a1f68); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008a1f6b); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008a1f72); }
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a1f79); };
            c.w64(1, t97);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008a1f7d); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008a1f80); };
            c.w64(12, t98);
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a1f87); };
            c.w32(7, t99);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008a1f97);
        }
        78 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a1f98); };
            c.w64(11, t100);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008a1f9c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r101 = c.r64(1) & c.r64(12);
            c.w64(1, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a1fae); };
            c.logic(t102 & 0x4u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a1fb4); };
            c.w64(9, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a1fb8); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a1fbc); };
            c.w64(15, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a1fc0); };
            c.w64(1, t106);
            blk = 53; continue;
        }
        81 => {
            return RegfileExit::fault(0x70008a1fc9);
        }
        82 => {
            return RegfileExit::fault(0x70008a1fca);
        }
        83 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a1fcb); }
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x22d8)) else { return RegfileExit::fault(0x70008a1fcc); };
            c.w64(3, t107);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a1fdf, 0x70008a1fda);
        }
        84 => {
            blk = 10; continue;
        }
        85 => {
            c.w32(0, c.r32(0));
            let r108 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r108);
            c.w64(1, 0xfffffffffffc0000u64);
            let r109 = c.r64(1) & c.r64(0);
            c.w64(1, r109);
            c.logic(r109, 64);
            let Some(t110) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a1ff3); };
            c.logic(t110 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a1ffd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a1ffe); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008a200b, 0x70008a2006);
        }
        87 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a200b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008a200d); }
            blk = 80; continue;
        }
        88 => {
            c.w32(0, c.r32(0));
            let r111 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r111);
            c.w64(8, 0xfffffffffffc0000u64);
            let r112 = c.r64(8) & c.r64(0);
            c.w64(8, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a201f); };
            c.logic(t113 & 0x2u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008a202e); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008a2038, 0x70008a2033);
        }
        90 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008a2038); }
            blk = 57; continue;
        }
        91 => {
            return RegfileExit::jump(0x70008a2040);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, 0x8bu64);
            blk = 7; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a205f); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2065); };
            c.w32(7, t2);
            let r3 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r3);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x0, 0x70008a2077);
        }
        4 => {
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008a2078); };
            c.w32(1, t4);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ne() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2082); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a2086); }
            let r5 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r5);
            c.w32(1, 0x16cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2091); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2092); };
            c.w64(1, t6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x1a3)) else { return RegfileExit::fault(0x70008a2099); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a20a2); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a20a3); };
            c.w64(3, t9);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a20b4, 0x70008a20af);
        }
        6 => {
            return RegfileExit::fault(0x70008a20b4);
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a20b5); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008a20bb); };
            c.w32(7, t12);
            let r13 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r13);
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a20c1); };
            c.w32(8, t14);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_b() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008a20d0);
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a20d1); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a20d8); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a20de); };
            c.logic(t19 & 0x200000u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a20e7); };
            c.w64(11, t20);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a20ee); };
            c.w64(8, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a20f5); };
            c.w64(7, t22);
            blk = 21; continue;
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a2101); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a2108); };
            c.w64(8, t24);
            c.w64(9, c.r64(8).wrapping_add(0x4c));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a210f); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a2116); };
            c.cmp(t26, c.r64(9), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a211f); }
            let r27 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r27);
            blk = 13; continue;
        }
        13 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008a2126); };
            c.w64(9, t28);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008a212d); }
            c.w32(9, 0x22u64);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008a2137); }
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a213b); };
            c.w32(9, t29);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008a2152); }
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a2156); };
            c.w32(12, t30);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008a216a); }
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a216e); };
            c.w32(9, t31);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(9, 0x8u64);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008a2188); }
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a218c); };
            c.w32(9, t32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(9, 0x2u64);
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008a21a6); }
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a21aa); };
            c.w32(9, t33);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008a21c1); }
            c.w64(9, c.r64(8).wrapping_add(0x4b));
            c.w64(12, c.r64(8).wrapping_add(0x1b));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a21d2); };
            c.w64(15, t34);
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(12), c.r32(15)) { return RegfileExit::fault(0x70008a21d9); }
            let r35 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r35);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 20; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t36) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a21e6); };
            c.w32(9, t36);
            c.w64(2, c.r64(9).wrapping_mul(4));
            let Some(t37) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a21f3); };
            c.w64(9, t37);
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t38) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a21fa); };
            c.cmp(t38, c.r64(12), 64);
            if c.cc_be() { blk = 43; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008a2203); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 23; continue;
        }
        23 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a220e); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008a2211); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008a2215); };
            c.w64(8, t39);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008a221c); }
            let Some(t40) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a2220); };
            c.w32(8, t40);
            c.w64(11, c.r64(8));
            let r41 = c.r32(11) & 0xe0000000u64;
            c.w32(11, r41);
            c.logic(r41, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2236); };
            c.w64(1, t42);
            blk = 26; continue;
        }
        26 => {
            let r43 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r43);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70008a2241); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_ne() { blk = 26; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 35; continue;
        }
        28 => {
            let r44 = c.sub(c.r32(8), 0x20000000u64, 32);
            c.w32(8, r44);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008a2254); }
            let Some(t45) = c.ld8(c.r64(1).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008a2258); };
            c.w32(8, t45);
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008a226a); };
            c.w64(12, t46);
            blk = 30; continue;
        }
        30 => {
            let r47 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r47);
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(12)) { return RegfileExit::fault(0x70008a2275); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ne() { blk = 30; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2284); };
            c.w64(9, t48);
            blk = 33; continue;
        }
        33 => {
            let r49 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r49);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(9)) { return RegfileExit::fault(0x70008a228f); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            let r50 = c.add(c.r32(11), 0xe0000000u64, 32);
            c.w32(11, r50);
            if c.cc_e() { blk = 45; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008a22a6); }
            c.w64(0, c.r64(7));
            return c.ret(0x0, 0x70008a22ac);
        }
        36 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a22ad); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a22b1); }
            let r51 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r51);
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a22bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a22c0); }
            return c.call(0x70007febf4, 0x70008a22c9, 0x70008a22c4);
        }
        37 => {
            c.w64(8, c.r64(0));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a22cc); };
            c.w64(0, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a22d0); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a22d4); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a22db); };
            c.w64(11, t55);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a22e5); }
            blk = 13; continue;
        }
        38 => {
            return RegfileExit::fault(0x70008a22eb);
        }
        39 => {
            return RegfileExit::fault(0x70008a22ec);
        }
        40 => {
            return RegfileExit::fault(0x70008a22ed);
        }
        41 => {
            return RegfileExit::fault(0x70008a22ee);
        }
        42 => {
            return RegfileExit::fault(0x70008a22ef);
        }
        43 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a22f0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a22f4); }
            let r56 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r56);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a22fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a22fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a2302); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008a2306); }
            return c.call(0x70007febf4, 0x70008a230f, 0x70008a230a);
        }
        44 => {
            c.w64(7, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a2312); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2316); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a231a); };
            c.w64(8, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a231e); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2325); }
            blk = 23; continue;
        }
        45 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a232b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a232f); }
            let r61 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r61);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2335); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008a2336); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a233a); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008a233e); };
            c.w64(3, t62);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a2354, 0x70008a234f);
        }
        46 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a2354); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a2358); };
            c.w64(0, t64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a235f); }
            blk = 35; continue;
        }
        47 => {
            return RegfileExit::jump(0x70008a2368);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2374); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a2378); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a237e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008a2382); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xcf8)) else { return RegfileExit::fault(0x70008a2386); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x70008a2392, 0x70008a238d);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2392); };
            c.cmp(t2, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a239f); };
            c.cmp(t3, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let r4 = c.r32(2) | c.r32(1);
            c.w32(2, r4);
            c.logic(r4, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x628)) else { return RegfileExit::fault(0x70008a23b4); };
            c.w64(2, t5);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a23bb); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a23bc); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008a23bc); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a23bf); };
            c.w64(6, t7);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008a23d3, 0x70008a23ce);
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a23db); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a23e1); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a23ea); };
            c.w32(1, t11);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a23fc); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a2405); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a240e); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2417); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2420); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0x3cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a242e); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a242f); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a243b); };
            c.w64(6, t18);
            return c.call(0x700086ed74, 0x70008a2444, 0x70008a243f);
        }
        13 => {
            return RegfileExit::fault(0x70008a2444);
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2448); }
            return c.ret(0x0, 0x70008a2449);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a244a); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a244e); };
            c.w64(6, t20);
            return c.call(0x70008a2674, 0x70008a2457, 0x70008a2452);
        }
        16 => {
            blk = 14; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008a245c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2474); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a2478); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a247e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008a2482); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xcf8)) else { return RegfileExit::fault(0x70008a2486); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x70008a2492, 0x70008a248d);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2492); };
            c.cmp(t2, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a249f); };
            c.cmp(t3, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let r4 = c.r32(2) | c.r32(1);
            c.w32(2, r4);
            c.logic(r4, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x70008a24b4); };
            c.w64(2, t5);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a24bb); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a24bc); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008a24bc); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a24bf); };
            c.w64(6, t7);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008a24d3, 0x70008a24ce);
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a24db); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a24e1); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a24ea); };
            c.w32(1, t11);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a24fc); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a2505); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a250e); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2517); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2520); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0x3cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a252e); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a252f); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a253b); };
            c.w64(6, t18);
            return c.call(0x700086ed74, 0x70008a2544, 0x70008a253f);
        }
        13 => {
            return RegfileExit::fault(0x70008a2544);
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2548); }
            return c.ret(0x0, 0x70008a2549);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a254a); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a254e); };
            c.w64(6, t20);
            return c.call(0x70008a2674, 0x70008a2557, 0x70008a2552);
        }
        16 => {
            blk = 14; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008a255c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2574); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a2578); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008a257e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008a2582); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xcf8)) else { return RegfileExit::fault(0x70008a2586); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x70008a2592, 0x70008a258d);
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2592); };
            c.cmp(t2, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a259f); };
            c.cmp(t3, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let r4 = c.r32(2) | c.r32(1);
            c.w32(2, r4);
            c.logic(r4, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x70008a25b4); };
            c.w64(2, t5);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a25bb); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a25bc); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008a25bc); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a25bf); };
            c.w64(6, t7);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008a25d3, 0x70008a25ce);
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a25db); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a25e1); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a25ea); };
            c.w32(1, t11);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a25fc); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a2605); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a260e); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2617); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2620); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0x3cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a262e); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a262f); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a263b); };
            c.w64(6, t18);
            return c.call(0x700086ed74, 0x70008a2644, 0x70008a263f);
        }
        13 => {
            return RegfileExit::fault(0x70008a2644);
        }
        14 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2648); }
            return c.ret(0x0, 0x70008a2649);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a264a); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a264e); };
            c.w64(6, t20);
            return c.call(0x70008a27d4, 0x70008a2657, 0x70008a2652);
        }
        16 => {
            blk = 14; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008a265c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a2678); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a267e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a2682); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xb58)) else { return RegfileExit::fault(0x70008a2686); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x70008a2692, 0x70008a268d);
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a269a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a26a0); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a26aa); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008a26aa); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a26ad); };
            c.w64(6, t6);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008a26be, 0x70008a26b9);
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a26c2); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a26c8); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a26d1); };
            c.w32(1, t10);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a26e3); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a26ec); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a26f5); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a26fe); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2707); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2713); }
            return c.ret(0x0, 0x70008a2714);
        }
        14 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xaf0)) else { return RegfileExit::fault(0x70008a2715); };
            c.w64(3, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a271c); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a2720); };
            c.w64(6, t18);
            return c.call(0x7000870914, 0x70008a2729, 0x70008a2724);
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a272d); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a2733); };
            c.logic(t21 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a2739); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008a2739); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a2741); };
            c.w64(6, t23);
            return c.call(0x70007f2274, 0x70008a274a, 0x70008a2745);
        }
        18 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a274e); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a2754); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t27) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a275d); };
            c.w32(1, t27);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 13; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a276f); };
            c.cmp(t28, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a2778); };
            c.cmp(t29, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a2781); };
            c.cmp(t30, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a278a); };
            c.cmp(t31, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2797); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(2, 0x3cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a27a9); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a27aa); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a27b6); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008a27bf, 0x70008a27ba);
        }
        28 => {
            return RegfileExit::fault(0x70008a27bf);
        }
        29 => {
            return RegfileExit::jump(0x70008a27c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a27d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a27d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008a27d8); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008a27de); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008a27e2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xaf0)) else { return RegfileExit::fault(0x70008a27e6); };
            c.w64(3, t1);
            return c.call(0x7000870914, 0x70008a27f2, 0x70008a27ed);
        }
        1 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a27fa); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a2800); };
            c.logic(t4 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a280a); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008a280a); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a280d); };
            c.w64(6, t6);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008a281e, 0x70008a2819);
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2822); };
            c.w32(2, t7);
            let r8 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a2828); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2831); };
            c.w32(1, t10);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a2843); };
            c.cmp(t11, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a284c); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a2855); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a285e); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a2867); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2873); }
            return c.ret(0x0, 0x70008a2874);
        }
        14 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xb58)) else { return RegfileExit::fault(0x70008a2875); };
            c.w64(3, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a287c); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a2880); };
            c.w64(6, t18);
            return c.call(0x7000870914, 0x70008a2889, 0x70008a2884);
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a288d); };
            c.w32(2, t19);
            let r20 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r20);
            let Some(t21) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a2893); };
            c.logic(t21 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008a2899); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008a2899); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a28a1); };
            c.w64(6, t23);
            return c.call(0x70007f2274, 0x70008a28aa, 0x70008a28a5);
        }
        18 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t24) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a28ae); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a28b4); };
            c.cmp(t26, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t27) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a28bd); };
            c.w32(1, t27);
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_e() { blk = 13; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(1, c.r16(1));
            c.cmp(c.r32(1), 0x3fu64, 32);
            if c.cc_be() { blk = 13; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008a28cf); };
            c.cmp(t28, c.r32(2), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008a28d8); };
            c.cmp(t29, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a28e1); };
            c.cmp(t30, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a28ea); };
            c.cmp(t31, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008a28f7); };
            c.cmp(t32, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(2, 0x3cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2909); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a290a); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008a2916); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008a291f, 0x70008a291a);
        }
        28 => {
            return RegfileExit::fault(0x70008a291f);
        }
        29 => {
            return RegfileExit::jump(0x70008a2924);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2934(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2934); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a2938); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a2939); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a293a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a293f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2955); };
            c.w64(1, t2);
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a295b); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a2962); };
            c.cmp(t4, c.r32(1), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(9, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a2973); };
            c.cmp(t5, c.r32(1), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a297c); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a2983); };
            c.logic(t8 & 0x10u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a298a); };
            c.cmp(t9, c.r32(9), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2997); };
            c.cmp(t10, 0x41u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 12; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(8).wrapping_add(0x1));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a29ab); };
            c.w64(0, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a29b5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a29b6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a29bc); }
            return c.ret(0x0, 0x70008a29be);
        }
        13 => {
            let Some(t12) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008a29bf); };
            c.w64(5, t12);
            let r13 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r13);
            return RegfileExit::jump(0x700082e4f4);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008a29cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008a29d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008a29d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008a29d8); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a29dc); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a29ed, 0x70008a29e8);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a29ed); };
            c.w64(7, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a29f1); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a29f5); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a29f9); };
            c.w64(6, t18);
            blk = 1; continue;
        }
        16 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a2a02); };
            c.logic(t19 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 12; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a2a0d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 12; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 13; continue;
        }
        20 => {
            return RegfileExit::jump(0x70008a2a20);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2a34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2a34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a2a38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a2a39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a2a3a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a2a3f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2a4f); };
            c.w64(2, t2);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2a59); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2a5f); };
            c.cmp(t5, 0x82du64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a2a67); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r8 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r8);
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2a7a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2a7b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2a81); }
            return c.ret(0x0, 0x70008a2a83);
        }
        4 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2a89); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2a8a); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008a2a91); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2a9a); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a2a9b); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a2aac, 0x70008a2aa7);
        }
        5 => {
            return RegfileExit::fault(0x70008a2aac);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a2aad); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a2ab1); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a2ab5); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a2ac6, 0x70008a2ac1);
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2ac6); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2aca); };
            c.w64(0, t15);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008a2ad4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2af4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a2af8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a2af9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a2afa); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a2aff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2b13); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2b1d); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2b23); };
            c.cmp(t5, 0x82du64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a2b2b); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            let r8 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a2b38); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a2b3e); };
            c.w64(1, t10);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2b51); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2b52); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2b58); }
            return c.ret(0x0, 0x70008a2b5a);
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2b60); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2b61); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2b67); }
            return c.ret(0x0, 0x70008a2b69);
        }
        7 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2b6f); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2b70); };
            c.w64(1, t11);
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008a2b77); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2b80); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a2b81); };
            c.w64(3, t14);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a2b92, 0x70008a2b8d);
        }
        8 => {
            return RegfileExit::fault(0x70008a2b92);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a2b93); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a2b97); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a2b9b); };
            c.w64(3, t15);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a2bac, 0x70008a2ba7);
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2bac); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2bb0); };
            c.w64(0, t17);
            blk = 1; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a2bb9); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a2bc0); };
            c.w64(9, t19);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008a2bc7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008a2bcb); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a2bcf); };
            c.w64(12, t20);
            let Some(t21) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a2bd6); };
            c.cmp(t21, c.r64(11), 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a2be6, 0x70008a2be1);
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2be6); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2bea); };
            c.w64(8, t23);
            blk = 15; continue;
        }
        14 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008a2bf0); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 15; continue;
        }
        15 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 18; continue;
        }
        18 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 19; continue;
        }
        19 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a2c21); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a2c28); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a2c2b); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2c33); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2c34); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2c3a); }
            return c.ret(0x0, 0x70008a2c3c);
        }
        20 => {
            return RegfileExit::jump(0x70008a2c40);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2c54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2c54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a2c58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a2c59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a2c5a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a2c5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2c73); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2c7d); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2c83); };
            c.cmp(t5, 0x82du64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a2c8b); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            let r8 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r8);
            let Some(t9) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a2c98); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a2c9e); };
            c.w64(1, t10);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2cb1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2cb2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2cb8); }
            return c.ret(0x0, 0x70008a2cba);
        }
        6 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2cc0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2cc1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2cc7); }
            return c.ret(0x0, 0x70008a2cc9);
        }
        7 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2ccf); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2cd0); };
            c.w64(1, t11);
            let Some(t12) = c.ld32(c.r64(1).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008a2cd7); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a2ce0); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a2ce1); };
            c.w64(3, t14);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a2cf2, 0x70008a2ced);
        }
        8 => {
            return RegfileExit::fault(0x70008a2cf2);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a2cf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a2cf7); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a2cfb); };
            c.w64(3, t15);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a2d0c, 0x70008a2d07);
        }
        10 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2d0c); };
            c.w64(6, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2d10); };
            c.w64(0, t17);
            blk = 1; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a2d19); };
            c.w64(7, t18);
            let Some(t19) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a2d20); };
            c.w64(9, t19);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008a2d27); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008a2d2b); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a2d2f); };
            c.w64(12, t20);
            let Some(t21) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a2d36); };
            c.cmp(t21, c.r64(11), 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a2d46, 0x70008a2d41);
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2d46); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2d4a); };
            c.w64(8, t23);
            blk = 15; continue;
        }
        14 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008a2d50); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 15; continue;
        }
        15 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            let r24 = c.r64(10) | 0x1u64;
            c.w64(10, r24);
            c.logic(r24, 64);
            blk = 18; continue;
        }
        18 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 19; continue;
        }
        19 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a2d81); };
            c.w64(1, t25);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a2d88); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a2d8b); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2d93); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2d94); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2d9a); }
            return c.ret(0x0, 0x70008a2d9c);
        }
        20 => {
            return RegfileExit::jump(0x70008a2da0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a2db4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a2db4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a2db8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a2db9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a2dba); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a2dbf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a2dc3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2dd7); };
            c.w64(1, t2);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008a2ddf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a2de3); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a2ded); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2df3); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2e03); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2e0a); };
            c.cmp(t7, 0x82du64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2e17); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        9 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a2e36, 0x70008a2e31);
        }
        10 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a2e36); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2e3a); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2e3e); };
            c.w64(7, t11);
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 21; continue;
        }
        14 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a2e57); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 32; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 32; continue; }
            blk = 16; continue;
        }
        16 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x8000000000000000u64);
            let r12 = c.r64(2) | c.r64(10);
            c.w64(2, r12);
            c.logic(r12, 64);
            blk = 19; continue;
        }
        19 => {
            blk = 21; continue;
        }
        20 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        21 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2ebc); };
            c.w64(8, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a2ec3); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a2ed5); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a2ede); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a2ee5); };
            c.logic(t18 & 0x10u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a2eec); };
            c.cmp(t19, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t20) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a2ef9); };
            c.cmp(t20, 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a2f0b); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a2f12); };
            c.logic(t23 & 0x4u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t24) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a2f19); };
            c.w64(9, t24);
            c.w64(11, c.r64(2).wrapping_add(0x1));
            let Some(t25) = c.ld64(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a2f21); };
            c.cmp(t25, c.r64(11), 64);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a2f27); };
            c.w64(1, t26);
            let r27 = c.add(c.r64(2), c.r64(9), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld8(c.r64(1).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a2f2e); };
            c.w32(2, t28);
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(7));
            let r29 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a2f3f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a2f40); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a2f46); }
            return c.ret(0x0, 0x70008a2f48);
        }
        32 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2f4e); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a2f4f); };
            c.w64(3, t30);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a2f60, 0x70008a2f5b);
        }
        33 => {
            return RegfileExit::fault(0x70008a2f60);
        }
        34 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2f66); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2f67); };
            c.w64(2, t31);
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008a2f6e); };
            c.w32(2, t32);
            let r33 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r33);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2f77); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a2f78); };
            c.w64(3, t34);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a2f89, 0x70008a2f84);
        }
        35 => {
            return RegfileExit::fault(0x70008a2f89);
        }
        36 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2f8f); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a2f90); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008a2f97); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a2fa0); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a2fa1); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a2fb2, 0x70008a2fad);
        }
        37 => {
            return RegfileExit::fault(0x70008a2fb2);
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a2fb3); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a2fb7); };
            c.w64(3, t39);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a2fc8, 0x70008a2fc3);
        }
        39 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a2fc8); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a2fcc); };
            c.w64(0, t41);
            blk = 1; continue;
        }
        40 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2fd5); };
            c.w64(8, t42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a2fdc); };
            c.logic(t43 & 0x7ffffffeu64, 32);
            blk = 29; continue;
        }
        41 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a2fe9); };
            c.w64(8, t44);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a2ff0); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            blk = 29; continue;
        }
        42 => {
            return RegfileExit::jump(0x70008a3004);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a3014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a3014); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a3018); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a3019); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a301a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a301f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a3023); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3037); };
            c.w64(1, t2);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008a303f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a3043); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a304d); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3053); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3063); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a306a); };
            c.cmp(t7, 0x82du64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3077); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        9 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a3096, 0x70008a3091);
        }
        10 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a3096); };
            c.w64(1, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a309a); };
            c.w64(6, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a309e); };
            c.w64(7, t11);
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 21; continue;
        }
        14 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a30b7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 32; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 32; continue; }
            blk = 16; continue;
        }
        16 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x8000000000000000u64);
            let r12 = c.r64(2) | c.r64(10);
            c.w64(2, r12);
            c.logic(r12, 64);
            blk = 19; continue;
        }
        19 => {
            blk = 21; continue;
        }
        20 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        21 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a311c); };
            c.w64(8, t13);
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a3123); };
            c.cmp(t14, c.r32(8), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a3135); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a313e); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a3145); };
            c.logic(t18 & 0x10u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a314c); };
            c.cmp(t19, c.r32(8), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t20) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3159); };
            c.cmp(t20, 0x41u64, 16);
            if c.cc_e() { blk = 40; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a316b); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a3172); };
            c.logic(t23 & 0x4u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t24) = c.ld64(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a3179); };
            c.w64(9, t24);
            c.w64(11, c.r64(2).wrapping_add(0x1));
            let Some(t25) = c.ld64(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a3181); };
            c.cmp(t25, c.r64(11), 64);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a3187); };
            c.w64(1, t26);
            let r27 = c.add(c.r64(2), c.r64(9), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld8(c.r64(1).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a318e); };
            c.w32(2, (t28 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(7));
            let r29 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a319f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a31a0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a31a6); }
            return c.ret(0x0, 0x70008a31a8);
        }
        32 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a31ae); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a31af); };
            c.w64(3, t30);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a31c0, 0x70008a31bb);
        }
        33 => {
            return RegfileExit::fault(0x70008a31c0);
        }
        34 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a31c6); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a31c7); };
            c.w64(2, t31);
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008a31ce); };
            c.w32(2, t32);
            let r33 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r33);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a31d7); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a31d8); };
            c.w64(3, t34);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a31e9, 0x70008a31e4);
        }
        35 => {
            return RegfileExit::fault(0x70008a31e9);
        }
        36 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a31ef); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a31f0); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x70008a31f7); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3200); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3201); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3212, 0x70008a320d);
        }
        37 => {
            return RegfileExit::fault(0x70008a3212);
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a3213); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a3217); };
            c.w64(3, t39);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a3228, 0x70008a3223);
        }
        39 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3228); };
            c.w64(6, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a322c); };
            c.w64(0, t41);
            blk = 1; continue;
        }
        40 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3235); };
            c.w64(8, t42);
            let Some(t43) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a323c); };
            c.logic(t43 & 0x7ffffffeu64, 32);
            blk = 29; continue;
        }
        41 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3249); };
            c.w64(8, t44);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3250); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            blk = 29; continue;
        }
        42 => {
            return RegfileExit::jump(0x70008a3264);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a3274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a3274); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a3278); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a3279); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a327a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a327f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a3283); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a329b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a32a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a32a7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a32b1); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a32b7); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a32c4); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a32ca); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a32d1); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a32df); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a32e6); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a32f3); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a3312, 0x70008a330d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a3312); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3316); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a331a); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a331e); };
            c.w64(7, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3333); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(2) | c.r64(10);
            c.w64(2, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 41; continue;
        }
        24 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a339a); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(9, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a33ab); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a33b4); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a33bb); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a33c2); };
            c.cmp(t21, c.r32(9), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a33cf); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 49; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a33ec); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a33f8); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a33ff); };
            c.w64(11, t26);
            c.w64(12, c.r64(2).wrapping_add(0x2));
            let Some(t27) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a3407); };
            c.cmp(t27, c.r64(12), 64);
            if c.cc_b() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a340d); };
            c.w64(7, t28);
            let r29 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r29);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a3419); };
            c.w32(1, (t30 as u8 as i8 as i64 as u64));
            let Some(t31) = c.ld8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a341d); };
            c.w32(2, t31);
            let r32 = c.shl(c.r32(1), 0x8u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r32);
            let r33 = c.add(c.r32(2), c.r32(1), 32);
            c.w32(2, r33);
            blk = 40; continue;
        }
        39 => {
            let Some(t34) = c.ld8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a3429); };
            c.w32(1, (t34 as u8 as i8 as i64 as u64));
            let Some(t35) = c.ld8(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a342e); };
            c.w32(2, t35);
            let r36 = c.shl(c.r32(1), 0x8u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r36);
            let r37 = c.add(c.r32(2), c.r32(1), 32);
            c.w32(2, r37);
            blk = 40; continue;
        }
        40 => {
            c.w32(2, c.r16(2));
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(8));
            let r38 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a3447); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a3448); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a344e); }
            return c.ret(0x0, 0x70008a3450);
        }
        41 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3456); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a3457); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008a3463, 0x70008a345e);
        }
        42 => {
            return RegfileExit::fault(0x70008a3463);
        }
        43 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3469); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a346a); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008a3471); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a347a); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a347b); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a348c, 0x70008a3487);
        }
        44 => {
            return RegfileExit::fault(0x70008a348c);
        }
        45 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3492); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3493); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008a349a); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a34a3); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a34a4); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a34b5, 0x70008a34b0);
        }
        46 => {
            return RegfileExit::fault(0x70008a34b5);
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a34b6); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a34bc); };
            c.w64(3, t48);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a34cb, 0x70008a34c6);
        }
        48 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a34cb); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a34cf); };
            c.w64(0, t50);
            blk = 1; continue;
        }
        49 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a34d8); };
            c.logic(t51 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 33; continue;
        }
        51 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a34ea); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 34; continue;
        }
        53 => {
            return RegfileExit::jump(0x70008a3504);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a3514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a3514); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a3518); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a3519); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a351a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a351f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a3523); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 47; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a353b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a3543); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a3547); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3551); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3557); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a3564); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a356a); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a3571); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a357f); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3586); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3593); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a35b2, 0x70008a35ad);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a35b2); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a35b6); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a35ba); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a35be); };
            c.w64(7, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a35d3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(2) | c.r64(10);
            c.w64(2, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 41; continue;
        }
        24 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a363a); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(9, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a364b); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3654); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a365b); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3662); };
            c.cmp(t21, c.r32(9), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a366f); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 49; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a368c); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a3698); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a369f); };
            c.w64(11, t26);
            c.w64(12, c.r64(2).wrapping_add(0x2));
            let Some(t27) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a36a7); };
            c.cmp(t27, c.r64(12), 64);
            if c.cc_b() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a36ad); };
            c.w64(7, t28);
            let r29 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r29);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t30) = c.ld8(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a36b9); };
            c.w32(1, (t30 as u8 as i8 as i64 as u64));
            let Some(t31) = c.ld8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a36bd); };
            c.w32(2, t31);
            let r32 = c.shl(c.r32(1), 0x8u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r32);
            let r33 = c.add(c.r32(2), c.r32(1), 32);
            c.w32(2, r33);
            blk = 40; continue;
        }
        39 => {
            let Some(t34) = c.ld8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a36c9); };
            c.w32(1, (t34 as u8 as i8 as i64 as u64));
            let Some(t35) = c.ld8(c.r64(7).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008a36ce); };
            c.w32(2, t35);
            let r36 = c.shl(c.r32(1), 0x8u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r36);
            let r37 = c.add(c.r32(2), c.r32(1), 32);
            c.w32(2, r37);
            blk = 40; continue;
        }
        40 => {
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(2, c.r64(8));
            let r38 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a36e4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a36e5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a36eb); }
            return c.ret(0x0, 0x70008a36ed);
        }
        41 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a36f3); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a36f4); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008a3700, 0x70008a36fb);
        }
        42 => {
            return RegfileExit::fault(0x70008a3700);
        }
        43 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3706); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3707); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008a370e); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3717); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3718); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3729, 0x70008a3724);
        }
        44 => {
            return RegfileExit::fault(0x70008a3729);
        }
        45 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a372f); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3730); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x70008a3737); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a3740); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3741); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3752, 0x70008a374d);
        }
        46 => {
            return RegfileExit::fault(0x70008a3752);
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a3753); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a3759); };
            c.w64(3, t48);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a3768, 0x70008a3763);
        }
        48 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3768); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a376c); };
            c.w64(0, t50);
            blk = 1; continue;
        }
        49 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a3775); };
            c.logic(t51 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 33; continue;
        }
        51 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3787); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 34; continue;
        }
        53 => {
            return RegfileExit::jump(0x70008a37a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a37b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a37b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a37b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a37b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a37ba); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a37bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a37c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a37db); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a37e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a37e7); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a37f1); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a37f7); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a3804); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a380a); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a3811); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a381f); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3826); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3833); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a3852, 0x70008a384d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a3852); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3856); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a385a); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a385e); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3873); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 42; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a38da); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a38ec); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a38f5); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a38fc); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3903); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3910); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a392d); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a3939); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a3944); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x4));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a394c); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a3952); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a3959); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a395e); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a3964); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a396a); };
            c.w32(1, t33);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r34 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r34);
            let r35 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r35);
            let r36 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            let r37 = c.r32(12) | c.r32(11);
            c.w32(12, r37);
            c.logic(r37, 32);
            let r38 = c.r32(15) | c.r32(12);
            c.w32(15, r38);
            c.logic(r38, 32);
            let r39 = c.r32(15) | c.r32(1);
            c.w32(15, r39);
            c.logic(r39, 32);
            c.w64(11, c.r64(15));
            blk = 40; continue;
        }
        39 => {
            let r40 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r40);
            let r41 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            let r42 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            let r43 = c.r32(15) | c.r32(1);
            c.w32(15, r43);
            c.logic(r43, 32);
            let r44 = c.r32(12) | c.r32(15);
            c.w32(12, r44);
            c.logic(r44, 32);
            let r45 = c.r32(11) | c.r32(12);
            c.w32(11, r45);
            c.logic(r45, 32);
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(9));
            let r46 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r46);
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 54; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a39be); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a39bf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a39c5); }
            return c.ret(0x0, 0x70008a39c7);
        }
        42 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a39cd); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a39ce); };
            c.w64(3, t47);
            return c.call(0x700086ed74, 0x70008a39da, 0x70008a39d5);
        }
        43 => {
            return RegfileExit::fault(0x70008a39da);
        }
        44 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a39e0); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a39e1); };
            c.w64(1, t48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x1c3)) else { return RegfileExit::fault(0x70008a39e8); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a39f1); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a39f2); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3a03, 0x70008a39fe);
        }
        45 => {
            return RegfileExit::fault(0x70008a3a03);
        }
        46 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3a09); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3a0a); };
            c.w64(1, t52);
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(0x1c3)) else { return RegfileExit::fault(0x70008a3a11); };
            c.w32(1, t53);
            let r54 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r54);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3a1a); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3a1b); };
            c.w64(3, t55);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3a2c, 0x70008a3a27);
        }
        47 => {
            return RegfileExit::fault(0x70008a3a2c);
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a3a2d); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a3a33); };
            c.w64(3, t56);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a3a42, 0x70008a3a3d);
        }
        49 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3a42); };
            c.w64(6, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3a46); };
            c.w64(0, t58);
            blk = 1; continue;
        }
        50 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a3a4f); };
            c.logic(t59 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 33; continue;
        }
        52 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3a61); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 34; continue;
        }
        54 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a3a79); };
            c.w64(7, t60);
            let Some(t61) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a3a80); };
            c.w64(8, t61);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a3a87); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008a3a8b); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a3a8f); };
            c.w64(12, t62);
            let Some(t63) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a3a96); };
            c.cmp(t63, c.r64(9), 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a3aa6, 0x70008a3aa1);
        }
        56 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3aa6); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3aaa); };
            c.w64(1, t65);
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a3ab0); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 58; continue;
        }
        58 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3aba); };
            c.w64(7, t66);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008a3ac1); }
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a3ad0); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a3ad8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a3ad9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a3adf); }
            return c.ret(0x0, 0x70008a3ae1);
        }
        59 => {
            return RegfileExit::jump(0x70008a3ae4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a3af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a3af4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a3af8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a3af9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a3afa); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a3aff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a3b03); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 48; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3b1b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a3b23); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a3b27); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3b31); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3b37); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a3b44); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3b4a); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a3b51); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3b5f); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3b66); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3b73); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a3b92, 0x70008a3b8d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a3b92); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3b96); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a3b9a); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3b9e); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3bb3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 42; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a3c1a); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a3c2c); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3c35); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a3c3c); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3c43); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3c50); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 50; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a3c6d); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a3c79); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a3c84); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x4));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a3c8c); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a3c92); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a3c99); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a3c9e); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a3ca4); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a3caa); };
            c.w32(1, t33);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r34 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r34);
            let r35 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r35);
            let r36 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            let r37 = c.r32(12) | c.r32(11);
            c.w32(12, r37);
            c.logic(r37, 32);
            let r38 = c.r32(15) | c.r32(12);
            c.w32(15, r38);
            c.logic(r38, 32);
            let r39 = c.r32(15) | c.r32(1);
            c.w32(15, r39);
            c.logic(r39, 32);
            c.w64(11, c.r64(15));
            blk = 40; continue;
        }
        39 => {
            let r40 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r40);
            let r41 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            let r42 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            let r43 = c.r32(15) | c.r32(1);
            c.w32(15, r43);
            c.logic(r43, 32);
            let r44 = c.r32(12) | c.r32(15);
            c.w32(12, r44);
            c.logic(r44, 32);
            let r45 = c.r32(11) | c.r32(12);
            c.w32(11, r45);
            c.logic(r45, 32);
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(9));
            let r46 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r46);
            c.w64(7, c.r64(11));
            let r47 = c.add(c.r32(7), c.r32(11), 32);
            c.w32(7, r47);
            if c.cc_o() { blk = 54; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a3cfc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a3cfd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a3d03); }
            return c.ret(0x0, 0x70008a3d05);
        }
        42 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3d0b); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a3d0c); };
            c.w64(3, t48);
            return c.call(0x700086ed74, 0x70008a3d18, 0x70008a3d13);
        }
        43 => {
            return RegfileExit::fault(0x70008a3d18);
        }
        44 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3d1e); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3d1f); };
            c.w64(1, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x1c7)) else { return RegfileExit::fault(0x70008a3d26); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3d2f); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3d30); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3d41, 0x70008a3d3c);
        }
        45 => {
            return RegfileExit::fault(0x70008a3d41);
        }
        46 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3d47); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a3d48); };
            c.w64(1, t53);
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(0x1c7)) else { return RegfileExit::fault(0x70008a3d4f); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a3d58); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a3d59); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a3d6a, 0x70008a3d65);
        }
        47 => {
            return RegfileExit::fault(0x70008a3d6a);
        }
        48 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a3d6b); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a3d71); };
            c.w64(3, t57);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a3d80, 0x70008a3d7b);
        }
        49 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3d80); };
            c.w64(6, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3d84); };
            c.w64(0, t59);
            blk = 1; continue;
        }
        50 => {
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a3d8d); };
            c.logic(t60 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 33; continue;
        }
        52 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3d9f); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 34; continue;
        }
        54 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a3db7); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a3dbe); };
            c.w64(8, t62);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a3dc5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008a3dc9); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a3dcd); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a3dd4); };
            c.cmp(t64, c.r64(9), 64);
            if c.cc_a() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a3de4, 0x70008a3ddf);
        }
        56 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3de4); };
            c.w64(11, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3de8); };
            c.w64(1, t66);
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a3dee); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 58; continue;
        }
        58 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3df8); };
            c.w64(7, t67);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008a3dff); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a3e0b); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a3e13); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a3e14); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a3e1a); }
            return c.ret(0x0, 0x70008a3e1c);
        }
        59 => {
            return RegfileExit::jump(0x70008a3e20);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a3e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a3e34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a3e38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a3e39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a3e3a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a3e3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a3e43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3e5b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a3e63); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a3e67); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a3e71); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3e77); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a3e84); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3e8a); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a3e91); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3e9f); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3ea6); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a3eb3); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a3ed2, 0x70008a3ecd);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a3ed2); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a3ed6); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a3eda); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a3ede); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a3ef3); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 46; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a3f5a); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a3f6c); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a3f75); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a3f7c); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a3f83); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a3f90); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a3fad); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a3fb9); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a3fc4); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x4));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a3fcc); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 46; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a3fd6); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a3fdd); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a3fe2); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a3fe8); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a3fee); };
            c.w32(1, t33);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r34 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r34);
            let r35 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r35);
            let r36 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            let r37 = c.r32(12) | c.r32(11);
            c.w32(12, r37);
            c.logic(r37, 32);
            let r38 = c.r32(15) | c.r32(12);
            c.w32(15, r38);
            c.logic(r38, 32);
            let r39 = c.r32(15) | c.r32(1);
            c.w32(15, r39);
            c.logic(r39, 32);
            c.w64(11, c.r64(15));
            blk = 40; continue;
        }
        39 => {
            let r40 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r40);
            let r41 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r41);
            let r42 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            let r43 = c.r32(15) | c.r32(1);
            c.w32(15, r43);
            c.logic(r43, 32);
            let r44 = c.r32(12) | c.r32(15);
            c.w32(12, r44);
            c.logic(r44, 32);
            let r45 = c.r32(11) | c.r32(12);
            c.w32(11, r45);
            c.logic(r45, 32);
            blk = 40; continue;
        }
        40 => {
            c.movd_xr(0, 11);
            c.cvtss2sd(0, c.xmm_u32(0));
            c.w64(1, c.r64(9));
            let r46 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r46);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 58; continue; }
            blk = 41; continue;
        }
        41 => {
            if c.cc_ne() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, c.r64(7));
            let r47 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r47);
            if c.cc_o() { blk = 58; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a407d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a407e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4084); }
            return c.ret(0x0, 0x70008a4086);
        }
        46 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a408c); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a408d); };
            c.w64(3, t48);
            return c.call(0x700086ed74, 0x70008a4099, 0x70008a4094);
        }
        47 => {
            return RegfileExit::fault(0x70008a4099);
        }
        48 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a409f); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a40a0); };
            c.w64(1, t49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x1cb)) else { return RegfileExit::fault(0x70008a40a7); };
            c.w32(1, t50);
            let r51 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r51);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a40b0); }
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a40b1); };
            c.w64(3, t52);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a40c2, 0x70008a40bd);
        }
        49 => {
            return RegfileExit::fault(0x70008a40c2);
        }
        50 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a40c8); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a40c9); };
            c.w64(1, t53);
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(0x1cb)) else { return RegfileExit::fault(0x70008a40d0); };
            c.w32(1, t54);
            let r55 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r55);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a40d9); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a40da); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a40eb, 0x70008a40e6);
        }
        51 => {
            return RegfileExit::fault(0x70008a40eb);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a40ec); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a40f2); };
            c.w64(3, t57);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a4101, 0x70008a40fc);
        }
        53 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4101); };
            c.w64(6, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4105); };
            c.w64(0, t59);
            blk = 1; continue;
        }
        54 => {
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a410e); };
            c.logic(t60 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 33; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4120); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 34; continue;
        }
        58 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a4138); };
            c.w64(7, t61);
            let Some(t62) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a413f); };
            c.w64(8, t62);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a4146); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a414a); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a414f); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a4156); };
            c.cmp(t64, c.r64(9), 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a4165, 0x70008a4160);
        }
        60 => {
            blk = 62; continue;
        }
        61 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a4167); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a4171); };
            c.w64(1, t65);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a4178); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a417b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a4180); }
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4185); };
            c.w64(1, t66);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a418c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a418d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4193); }
            return c.ret(0x0, 0x70008a4195);
        }
        63 => {
            return RegfileExit::jump(0x70008a4198);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a41b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a41b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a41b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a41b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a41ba); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a41bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a41c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 52; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a41db); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a41e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a41e7); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a41f1); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a41f7); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a4204); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a420a); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a4211); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a421f); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4226); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 50; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4233); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a4252, 0x70008a424d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a4252); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4256); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a425a); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a425e); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4273); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 46; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a42da); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a42ec); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a42f5); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a42fc); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a4303); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4310); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 54; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a432d); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a4339); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a4344); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x8));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a434c); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 46; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a4356); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a435d); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a4362); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a4368); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a436e); };
            c.w32(0, t33);
            let Some(t34) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008a4374); };
            c.w32(3, t34);
            let Some(t35) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008a437a); };
            c.w32(2, t35);
            let Some(t36) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x6)) else { return RegfileExit::fault(0x70008a4380); };
            c.w32(6, t36);
            let Some(t37) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4386); };
            c.w32(1, t37);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r38 = c.shl(c.r32(2), 0x10u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            let r39 = c.shl(c.r32(3), 0x18u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r39);
            let r40 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r41);
            let r42 = c.shl(c.r32(6), 0x8u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r42);
            let r43 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r43);
            let r44 = c.r32(2) | c.r32(3);
            c.w32(2, r44);
            c.logic(r44, 32);
            let r45 = c.r32(12) | c.r32(11);
            c.w32(12, r45);
            c.logic(r45, 32);
            let r46 = c.r32(6) | c.r32(2);
            c.w32(6, r46);
            c.logic(r46, 32);
            let r47 = c.r32(15) | c.r32(12);
            c.w32(15, r47);
            c.logic(r47, 32);
            let r48 = c.r32(6) | c.r32(1);
            c.w32(6, r48);
            c.logic(r48, 32);
            let r49 = c.r32(15) | c.r32(0);
            c.w32(15, r49);
            c.logic(r49, 32);
            c.w64(11, c.r64(6));
            c.w64(3, c.r64(15));
            blk = 40; continue;
        }
        39 => {
            let r50 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r50);
            let r51 = c.shl(c.r32(0), 0x18u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r51);
            let r52 = c.shl(c.r32(6), 0x10u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r52);
            let r53 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r53);
            let r54 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r54);
            let r55 = c.shl(c.r32(2), 0x8u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r55);
            let r56 = c.r32(15) | c.r32(0);
            c.w32(15, r56);
            c.logic(r56, 32);
            let r57 = c.r32(6) | c.r32(1);
            c.w32(6, r57);
            c.logic(r57, 32);
            let r58 = c.r32(12) | c.r32(15);
            c.w32(12, r58);
            c.logic(r58, 32);
            let r59 = c.r32(2) | c.r32(6);
            c.w32(2, r59);
            c.logic(r59, 32);
            let r60 = c.r32(11) | c.r32(12);
            c.w32(11, r60);
            c.logic(r60, 32);
            let r61 = c.r32(3) | c.r32(2);
            c.w32(3, r61);
            c.logic(r61, 32);
            blk = 40; continue;
        }
        40 => {
            c.movd_xr(0, 11);
            c.movd_xr(15, 3);
            c.punpckldq_rr(0, 15);
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(1, cv);
            c.w64(7, c.r64(9));
            let r62 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r62);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 58; continue; }
            blk = 41; continue;
        }
        41 => {
            if c.cc_ne() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, c.r64(1));
            let r63 = c.add(c.r32(8), c.r32(1), 32);
            c.w32(8, r63);
            if c.cc_o() { blk = 58; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a443c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a443d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4443); }
            return c.ret(0x0, 0x70008a4445);
        }
        46 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a444b); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a444c); };
            c.w64(3, t64);
            return c.call(0x700086ed74, 0x70008a4458, 0x70008a4453);
        }
        47 => {
            return RegfileExit::fault(0x70008a4458);
        }
        48 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a445e); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a445f); };
            c.w64(1, t65);
            let Some(t66) = c.ld32(c.r64(1).wrapping_add(0x1cf)) else { return RegfileExit::fault(0x70008a4466); };
            c.w32(1, t66);
            let r67 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r67);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a446f); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4470); };
            c.w64(3, t68);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a4481, 0x70008a447c);
        }
        49 => {
            return RegfileExit::fault(0x70008a4481);
        }
        50 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4487); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4488); };
            c.w64(1, t69);
            let Some(t70) = c.ld32(c.r64(1).wrapping_add(0x1cf)) else { return RegfileExit::fault(0x70008a448f); };
            c.w32(1, t70);
            let r71 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r71);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4498); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4499); };
            c.w64(3, t72);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a44aa, 0x70008a44a5);
        }
        51 => {
            return RegfileExit::fault(0x70008a44aa);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a44ab); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a44b1); };
            c.w64(3, t73);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a44c0, 0x70008a44bb);
        }
        53 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a44c0); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a44c4); };
            c.w64(0, t75);
            blk = 1; continue;
        }
        54 => {
            let Some(t76) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a44cd); };
            c.logic(t76 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 33; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a44df); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 34; continue;
        }
        58 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a44f7); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008a44fe); };
            c.w64(8, t78);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008a4505); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008a4509); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a450e); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008a4515); };
            c.cmp(t80, c.r64(9), 64);
            if c.cc_a() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008a4524, 0x70008a451f);
        }
        60 => {
            blk = 62; continue;
        }
        61 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008a4526); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 62; continue;
        }
        62 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a4530); };
            c.w64(1, t81);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008a4537); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008a453a); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008a453f); }
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4544); };
            c.w64(7, t82);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a454b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a454c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4552); }
            return c.ret(0x0, 0x70008a4554);
        }
        63 => {
            return RegfileExit::jump(0x70008a4558);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a4574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a4574); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a4578); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a4579); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a457a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a457f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a4583); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a459b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008a45a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a45a7); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a45b1); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a45b7); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a45c4); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a45ca); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008a45d1); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a45df); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a45e6); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a45f3); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a4612, 0x70008a460d);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a4612); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4616); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a461a); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a461e); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4633); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 48; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a469a); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a46ac); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a46b5); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a46bc); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a46c3); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a46d0); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a46ed); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a46f9); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a4704); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x8));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a470c); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 48; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a4716); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a471d); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a4722); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a4728); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a472e); };
            c.w32(0, t33);
            let Some(t34) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008a4734); };
            c.w32(3, t34);
            let Some(t35) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008a473a); };
            c.w32(2, t35);
            let Some(t36) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x6)) else { return RegfileExit::fault(0x70008a4740); };
            c.w32(6, t36);
            let Some(t37) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4746); };
            c.w32(1, t37);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r38 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r38);
            let r39 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r39);
            let r40 = c.shl(c.r32(2), 0x10u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r40);
            let r41 = c.shl(c.r32(3), 0x18u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r41);
            let r42 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r42);
            let r43 = c.shl(c.r32(6), 0x8u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r43);
            let r44 = c.r32(12) | c.r32(11);
            c.w32(12, r44);
            c.logic(r44, 32);
            let r45 = c.r32(2) | c.r32(3);
            c.w32(2, r45);
            c.logic(r45, 32);
            let r46 = c.r32(15) | c.r32(12);
            c.w32(15, r46);
            c.logic(r46, 32);
            let r47 = c.r32(6) | c.r32(2);
            c.w32(6, r47);
            c.logic(r47, 32);
            let r48 = c.r32(15) | c.r32(0);
            c.w32(15, r48);
            c.logic(r48, 32);
            let r49 = c.r32(6) | c.r32(1);
            c.w32(6, r49);
            c.logic(r49, 32);
            c.w64(3, c.r64(15));
            c.w64(11, c.r64(6));
            blk = 40; continue;
        }
        39 => {
            let r50 = c.shl(c.r32(6), 0x10u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r50);
            let r51 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r51);
            let r52 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r52);
            let r53 = c.shl(c.r32(0), 0x18u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r53);
            let r54 = c.shl(c.r32(2), 0x8u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r54);
            let r55 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r55);
            let r56 = c.r32(6) | c.r32(1);
            c.w32(6, r56);
            c.logic(r56, 32);
            let r57 = c.r32(15) | c.r32(0);
            c.w32(15, r57);
            c.logic(r57, 32);
            let r58 = c.r32(2) | c.r32(6);
            c.w32(2, r58);
            c.logic(r58, 32);
            let r59 = c.r32(12) | c.r32(15);
            c.w32(12, r59);
            c.logic(r59, 32);
            let r60 = c.r32(3) | c.r32(2);
            c.w32(3, r60);
            c.logic(r60, 32);
            let r61 = c.r32(11) | c.r32(12);
            c.w32(11, r61);
            c.logic(r61, 32);
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(9));
            let r62 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r62);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a47b2); };
            c.w64(7, t63);
            let Some(t64) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a47b9); };
            c.w64(8, t64);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a47c0); };
            c.w64(12, t65);
            let Some(t66) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008a47c7); };
            c.cmp(t66, c.r64(9), 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008a47d1); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008a47d8); };
            c.w64(8, t67);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008a47df); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008a47e3); }
            if let Some(sr) = c.shl(c.r64(3), 0x20u64, 64) { c.w64(3, sr); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008a47ee); }
            c.w64(8, c.r64(3).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(7).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008a47f9); }
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a4803); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a4804); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a480a); }
            return c.ret(0x0, 0x70008a480c);
        }
        45 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a480d); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a4814); };
            c.w64(8, t69);
            c.w64(11, c.r64(8).wrapping_add(0x8));
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a481b); };
            c.w64(9, t70);
            let Some(t71) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a4822); };
            c.cmp(t71, c.r64(11), 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008a482b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008a4832); };
            c.w64(8, t72);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008a4839); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008a483d); }
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a484a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a484b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4851); }
            return c.ret(0x0, 0x70008a4853);
        }
        48 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4859); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a485a); };
            c.w64(3, t73);
            return c.call(0x700086ed74, 0x70008a4866, 0x70008a4861);
        }
        49 => {
            return RegfileExit::fault(0x70008a4866);
        }
        50 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a486c); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a486d); };
            c.w64(1, t74);
            let Some(t75) = c.ld32(c.r64(1).wrapping_add(0x1d3)) else { return RegfileExit::fault(0x70008a4874); };
            c.w32(1, t75);
            let r76 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r76);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a487d); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a487e); };
            c.w64(3, t77);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a488f, 0x70008a488a);
        }
        51 => {
            return RegfileExit::fault(0x70008a488f);
        }
        52 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4895); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4896); };
            c.w64(1, t78);
            let Some(t79) = c.ld32(c.r64(1).wrapping_add(0x1d3)) else { return RegfileExit::fault(0x70008a489d); };
            c.w32(1, t79);
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a48a6); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a48a7); };
            c.w64(3, t81);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a48b8, 0x70008a48b3);
        }
        53 => {
            return RegfileExit::fault(0x70008a48b8);
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a48b9); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a48bf); };
            c.w64(3, t82);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a48ce, 0x70008a48c9);
        }
        55 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a48ce); };
            c.w64(6, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a48d2); };
            c.w64(0, t84);
            blk = 1; continue;
        }
        56 => {
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a48db); };
            c.logic(t85 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 33; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a48ed); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 34; continue;
        }
        60 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a490a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008a490e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008a4912); }
            return c.call(0x70007febb4, 0x70008a491b, 0x70008a4916);
        }
        61 => {
            c.w64(7, c.r64(0));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a491e); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a4922); };
            c.w64(11, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4926); };
            c.w64(3, t88);
            blk = 44; continue;
        }
        62 => {
            c.w32(2, 0x8u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a4934); }
            return c.call(0x70007febb4, 0x70008a493d, 0x70008a4938);
        }
        63 => {
            c.w64(7, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4940); };
            c.w64(1, t89);
            blk = 47; continue;
        }
        64 => {
            return RegfileExit::jump(0x70008a494c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a4954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a4954); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a4958); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a4959); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a495a); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a495f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a4963); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 57; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a497b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a4983); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a4987); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a4991); };
            c.w64(1, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4997); };
            c.w64(1, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a49a4); };
            c.w64(7, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a49aa); };
            c.w64(7, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008a49b1); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a49bf); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a49c6); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 55; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a49d3); };
            c.cmp(t10, c.r32(1), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(1));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008a49f2, 0x70008a49ed);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a49f2); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a49f6); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a49fa); };
            c.w64(9, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a49fe); };
            c.w64(8, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 23; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            blk = 25; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4a13); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(1, cv);
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(1) | c.r64(10);
            c.w64(1, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            blk = 25; continue;
        }
        23 => {
            c.w32(0, 0x1u64);
            blk = 51; continue;
        }
        24 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        25 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a4a7a); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a4a8c); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a4a95); };
            c.w32(11, t18);
            let r19 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r19);
            let Some(t20) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a4a9c); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a4aa3); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t22) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4ab0); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 59; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 33; continue;
        }
        32 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        35 => {
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a4acd); };
            c.w32(11, t23);
            let r24 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r24);
            c.w32(0, 0x1u64);
            let Some(t25) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a4ad9); };
            c.logic(t25 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a4ae4); };
            c.w64(12, t26);
            c.w64(15, c.r64(1).wrapping_add(0x8));
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a4aec); };
            c.cmp(t27, c.r64(15), 64);
            if c.cc_b() { blk = 51; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t28) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a4af6); };
            c.w64(8, t28);
            let r29 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(8).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008a4afd); };
            c.w32(11, t30);
            let Some(t31) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008a4b02); };
            c.w32(12, t31);
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008a4b08); };
            c.w32(15, t32);
            let Some(t33) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a4b0e); };
            c.w32(3, t33);
            let Some(t34) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008a4b14); };
            c.w32(2, t34);
            let Some(t35) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008a4b1a); };
            c.w32(6, t35);
            let Some(t36) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x6)) else { return RegfileExit::fault(0x70008a4b20); };
            c.w32(0, t36);
            let Some(t37) = c.ld8(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4b26); };
            c.w32(1, t37);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r38 = c.shl(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r38);
            let r39 = c.shl(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r39);
            let r40 = c.shl(c.r32(6), 0x10u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r40);
            let r41 = c.shl(c.r32(2), 0x18u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r41);
            let r42 = c.shl(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r42);
            let r43 = c.shl(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r43);
            let r44 = c.r32(12) | c.r32(11);
            c.w32(12, r44);
            c.logic(r44, 32);
            let r45 = c.r32(6) | c.r32(2);
            c.w32(6, r45);
            c.logic(r45, 32);
            let r46 = c.r32(15) | c.r32(12);
            c.w32(15, r46);
            c.logic(r46, 32);
            let r47 = c.r32(0) | c.r32(6);
            c.w32(0, r47);
            c.logic(r47, 32);
            let r48 = c.r32(15) | c.r32(3);
            c.w32(15, r48);
            c.logic(r48, 32);
            let r49 = c.r32(0) | c.r32(1);
            c.w32(0, r49);
            c.logic(r49, 32);
            c.w64(2, c.r64(15));
            c.w64(11, c.r64(0));
            blk = 40; continue;
        }
        39 => {
            let r50 = c.shl(c.r32(0), 0x10u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r50);
            let r51 = c.shl(c.r32(1), 0x18u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r51);
            let r52 = c.shl(c.r32(15), 0x10u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r52);
            let r53 = c.shl(c.r32(3), 0x18u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r53);
            let r54 = c.shl(c.r32(6), 0x8u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r54);
            let r55 = c.shl(c.r32(12), 0x8u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r55);
            let r56 = c.r32(0) | c.r32(1);
            c.w32(0, r56);
            c.logic(r56, 32);
            let r57 = c.r32(15) | c.r32(3);
            c.w32(15, r57);
            c.logic(r57, 32);
            let r58 = c.r32(6) | c.r32(0);
            c.w32(6, r58);
            c.logic(r58, 32);
            let r59 = c.r32(12) | c.r32(15);
            c.w32(12, r59);
            c.logic(r59, 32);
            let r60 = c.r32(2) | c.r32(6);
            c.w32(2, r60);
            c.logic(r60, 32);
            let r61 = c.r32(11) | c.r32(12);
            c.w32(11, r61);
            c.logic(r61, 32);
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(9));
            let r62 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r62);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            if let Some(sr) = c.shl(c.r64(2), 0x20u64, 64) { c.w64(2, sr); }
            c.w64(7, c.r64(2).wrapping_add(c.r64(11)));
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_l() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 45; continue;
        }
        44 => {
            let r63 = c.neg(c.r64(7), 64);
            c.w64(7, r63);
            c.w32(8, 0x1u64);
            blk = 45; continue;
        }
        45 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a4bae); };
            c.w64(9, t64);
            let Some(t65) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a4bb5); };
            c.w64(11, t65);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a4bbc); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008a4bc3); };
            c.cmp(t67, c.r64(12), 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008a4bcc); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008a4bd3); };
            c.w64(11, t68);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008a4bda); }
            let r69 = c.r32(8) & 0x1u64;
            c.w32(8, r69);
            c.logic(r69, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008a4be2); }
            let r70 = c.r32(8) | 0x2u64;
            c.w32(8, r70);
            c.logic(r70, 32);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008a4bee); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(7)) { return RegfileExit::fault(0x70008a4bf2); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a4bfc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a4bfd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4c03); }
            return c.ret(0x0, 0x70008a4c05);
        }
        48 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008a4c06); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008a4c0d); };
            c.w64(8, t72);
            c.w64(11, c.r64(8).wrapping_add(0x8));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008a4c14); };
            c.w64(9, t73);
            let Some(t74) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008a4c1b); };
            c.cmp(t74, c.r64(11), 64);
            if c.cc_be() { blk = 65; continue; }
            blk = 49; continue;
        }
        49 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008a4c24); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 50; continue;
        }
        50 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008a4c2b); };
            c.w64(8, t75);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008a4c32); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008a4c36); }
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a4c43); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a4c44); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4c4a); }
            return c.ret(0x0, 0x70008a4c4c);
        }
        51 => {
            c.w32(1, 0x19au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4c52); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a4c53); };
            c.w64(3, t76);
            return c.call(0x700086ed74, 0x70008a4c5f, 0x70008a4c5a);
        }
        52 => {
            return RegfileExit::fault(0x70008a4c5f);
        }
        53 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4c65); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4c66); };
            c.w64(1, t77);
            let Some(t78) = c.ld32(c.r64(1).wrapping_add(0x1d7)) else { return RegfileExit::fault(0x70008a4c6d); };
            c.w32(1, t78);
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4c76); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4c77); };
            c.w64(3, t80);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a4c88, 0x70008a4c83);
        }
        54 => {
            return RegfileExit::fault(0x70008a4c88);
        }
        55 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4c8e); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4c8f); };
            c.w64(1, t81);
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0x1d7)) else { return RegfileExit::fault(0x70008a4c96); };
            c.w32(1, t82);
            let r83 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r83);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008a4c9f); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4ca0); };
            c.w64(3, t84);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a4cb1, 0x70008a4cac);
        }
        56 => {
            return RegfileExit::fault(0x70008a4cb1);
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a4cb2); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a4cb8); };
            c.w64(3, t85);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008a4cc7, 0x70008a4cc2);
        }
        58 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4cc7); };
            c.w64(6, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4ccb); };
            c.w64(0, t87);
            blk = 1; continue;
        }
        59 => {
            let Some(t88) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a4cd4); };
            c.logic(t88 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 33; continue;
        }
        61 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4ce6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 33; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 34; continue;
        }
        63 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a4d03); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008a4d07); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a4d0b); }
            return c.call(0x70007febb4, 0x70008a4d14, 0x70008a4d0f);
        }
        64 => {
            c.w64(9, c.r64(0));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4d17); };
            c.w64(1, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a4d1b); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4d1f); };
            c.w64(7, t91);
            blk = 47; continue;
        }
        65 => {
            c.w32(2, 0x8u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008a4d2d); }
            return c.call(0x70007febb4, 0x70008a4d36, 0x70008a4d31);
        }
        66 => {
            c.w64(7, c.r64(0));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4d39); };
            c.w64(1, t92);
            blk = 50; continue;
        }
        67 => {
            return RegfileExit::jump(0x70008a4d44);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a4d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a4d54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a4d58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a4d59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a4d5a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a4d5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a4d63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a4d7b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a4d83); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008a4d87); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a4d91); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4d97); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a4da4); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4daa); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008a4db1); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a4dbf); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4dc6); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4dd3); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a4df2, 0x70008a4ded);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a4df2); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4df6); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4dfa); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a4dfe); };
            c.w64(7, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 22; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4e17); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 40; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(2) | c.r64(10);
            c.w64(2, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            c.w64(9, c.r64(2));
            blk = 24; continue;
        }
        23 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4e82); };
            c.w64(11, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a4e89); };
            c.cmp(t17, c.r32(11), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(12, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a4e9b); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a4ea4); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            let Some(t21) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a4eab); };
            c.logic(t21 & 0x10u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a4eb2); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a4ebf); };
            c.cmp(t23, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a4ed1); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t26) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a4ee1); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t27) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a4ee8); };
            c.w64(2, t27);
            c.w64(12, c.r64(9).wrapping_add(0x1));
            let Some(t28) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a4ef0); };
            c.cmp(t28, c.r64(12), 64);
            if c.cc_b() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4efb); }
            blk = 38; continue;
        }
        37 => {
            let r29 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r29);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 38; continue;
        }
        38 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 67; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(1, c.r32(1));
            let Some(t30) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a4f1d); };
            c.w64(7, t30);
            let r31 = c.add(c.r64(2), c.r64(9), 64);
            c.w64(2, r31);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008a4f24); }
            c.w64(2, c.r64(8));
            let r32 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4f2e); };
            c.w64(0, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a4f38); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a4f39); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a4f3f); }
            return c.ret(0x0, 0x70008a4f41);
        }
        40 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a4f47); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a4f48); };
            c.w64(3, t34);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a4f59, 0x70008a4f54);
        }
        41 => {
            return RegfileExit::fault(0x70008a4f59);
        }
        42 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a4f5f); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4f60); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x1db)) else { return RegfileExit::fault(0x70008a4f67); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a4f70); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4f71); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a4f82, 0x70008a4f7d);
        }
        43 => {
            return RegfileExit::fault(0x70008a4f82);
        }
        44 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a4f88); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a4f89); };
            c.w64(2, t39);
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0x1db)) else { return RegfileExit::fault(0x70008a4f90); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a4f99); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a4f9a); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a4fab, 0x70008a4fa6);
        }
        45 => {
            return RegfileExit::fault(0x70008a4fab);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a4fac); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a4fb0); };
            c.w64(3, t43);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a4fc1, 0x70008a4fbc);
        }
        47 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a4fc1); };
            c.w64(6, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a4fc5); };
            c.w64(0, t45);
            blk = 1; continue;
        }
        48 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4fce); };
            c.w64(11, t46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a4fd5); };
            c.logic(t47 & 0x7ffffffeu64, 32);
            blk = 32; continue;
        }
        49 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a4fe2); };
            c.w64(11, t48);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a4fe9); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            blk = 32; continue;
        }
        50 => {
            c.w64(0, c.r64(1));
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a4fff); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a5005); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a5009); }
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a500d); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(1, c.r64(0));
            blk = 33; continue;
        }
        52 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a501e); };
            c.cmp(t52, 0xbau64, 16);
            if c.cc_b() { blk = 57; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a2474, 0x70008a502b, 0x70008a5026);
        }
        54 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5033); };
            c.w32(2, t53);
            let r54 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5039); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5048); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a504c); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5050); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5054); };
            c.w64(11, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5058); };
            c.w64(7, t60);
            blk = 57; continue;
        }
        57 => {
            let Some(t61) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a505c); };
            c.w32(1, t61);
            let r62 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r62);
            let Some(t63) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5062); };
            c.w32(1, t63);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a5072); }
            c.w32(0, 0x1u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a5078); };
            c.w64(3, t64);
            return c.call(0x700086ed74, 0x70008a5084, 0x70008a507f);
        }
        60 => {
            blk = 66; continue;
        }
        61 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5086); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        62 => {
            let Some(t67) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5091); };
            c.w32(1, t67);
            c.logic(c.r32(1) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r68 = c.r64(1) & 0xffffffu64;
            c.w64(1, r68);
            c.logic(r68, 64);
            let r69 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r69);
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        64 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a50ae); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a50af); };
            c.w64(3, t70);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a50c3, 0x70008a50be);
        }
        65 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a50c3); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a50c7); };
            c.w64(6, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a50cb); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a50cf); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a50d3); };
            c.w64(11, t75);
            blk = 66; continue;
        }
        66 => {
            c.w64(1, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a50da); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a50de); };
            c.w64(8, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a50e2); };
            c.w64(9, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a50e6); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a50ea); };
            c.w64(7, t80);
            blk = 33; continue;
        }
        67 => {
            let r81 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r81);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a50f7); }
            return c.call(0x70008708b4, 0x70008a5101, 0x70008a50fc);
        }
        68 => {
            let Some(t82) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a5101); };
            c.w32(1, t82);
            let r83 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r83);
            blk = 39; continue;
        }
        69 => {
            return RegfileExit::jump(0x70008a5110);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a5114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a5114); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a5118); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a5119); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a511a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a511f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a5123); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a513b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a5143); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008a5147); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a5151); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5157); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a5164); };
            c.w64(1, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a516a); };
            c.w64(1, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008a5171); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a517f); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5186); };
            c.cmp(t9, 0x82du64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5193); };
            c.cmp(t10, c.r32(2), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(0, c.r64(2));
            blk = 14; continue;
        }
        12 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a51b2, 0x70008a51ad);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a51b2); };
            c.w64(1, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a51b6); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a51ba); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a51be); };
            c.w64(7, t14);
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 22; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a51d7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 40; continue; }
            blk = 19; continue;
        }
        19 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x8000000000000000u64);
            let r15 = c.r64(2) | c.r64(10);
            c.w64(2, r15);
            c.logic(r15, 64);
            blk = 22; continue;
        }
        22 => {
            c.w64(9, c.r64(2));
            blk = 24; continue;
        }
        23 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5242); };
            c.w64(11, t16);
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a5249); };
            c.cmp(t17, c.r32(11), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(12, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 31; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a525b); };
            c.cmp(t18, c.r32(11), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5264); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            let Some(t21) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a526b); };
            c.logic(t21 & 0x10u64, 8);
            if c.cc_ne() { blk = 32; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t22) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5272); };
            c.cmp(t22, c.r32(11), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a527f); };
            c.cmp(t23, 0x41u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a5291); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t26) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a52a1); };
            c.logic(t26 & 0x4u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t27) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a52a8); };
            c.w64(2, t27);
            c.w64(12, c.r64(9).wrapping_add(0x1));
            let Some(t28) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a52b0); };
            c.cmp(t28, c.r64(12), 64);
            if c.cc_b() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a52bb); }
            blk = 38; continue;
        }
        37 => {
            let r29 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r29);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 38; continue;
        }
        38 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 67; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(1, c.r32(1));
            let Some(t30) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a52dd); };
            c.w64(7, t30);
            let r31 = c.add(c.r64(2), c.r64(9), 64);
            c.w64(2, r31);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008a52e4); }
            c.w64(2, c.r64(8));
            let r32 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r32);
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a52ee); };
            c.w64(0, t33);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a52f8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a52f9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a52ff); }
            return c.ret(0x0, 0x70008a5301);
        }
        40 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5307); }
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a5308); };
            c.w64(3, t34);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008a5319, 0x70008a5314);
        }
        41 => {
            return RegfileExit::fault(0x70008a5319);
        }
        42 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a531f); }
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5320); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x1df)) else { return RegfileExit::fault(0x70008a5327); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5330); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5331); };
            c.w64(3, t38);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5342, 0x70008a533d);
        }
        43 => {
            return RegfileExit::fault(0x70008a5342);
        }
        44 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5348); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5349); };
            c.w64(2, t39);
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0x1df)) else { return RegfileExit::fault(0x70008a5350); };
            c.w32(2, t40);
            let r41 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r41);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5359); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a535a); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a536b, 0x70008a5366);
        }
        45 => {
            return RegfileExit::fault(0x70008a536b);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a536c); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a5370); };
            c.w64(3, t43);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a5381, 0x70008a537c);
        }
        47 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5381); };
            c.w64(6, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5385); };
            c.w64(0, t45);
            blk = 1; continue;
        }
        48 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a538e); };
            c.w64(11, t46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5395); };
            c.logic(t47 & 0x7ffffffeu64, 32);
            blk = 32; continue;
        }
        49 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a53a2); };
            c.w64(11, t48);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a53a9); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            blk = 32; continue;
        }
        50 => {
            c.w64(0, c.r64(1));
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a53bf); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008a53c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a53c9); }
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a53cd); };
            c.cmp(t51, c.r32(1), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(1, c.r64(0));
            blk = 33; continue;
        }
        52 => {
            let Some(t52) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a53de); };
            c.cmp(t52, 0xbau64, 16);
            if c.cc_b() { blk = 57; continue; }
            blk = 53; continue;
        }
        53 => {
            return c.call(0x70008a2474, 0x70008a53eb, 0x70008a53e6);
        }
        54 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a53f3); };
            c.w32(2, t53);
            let r54 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a53f9); };
            c.cmp(t55, c.r32(2), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5408); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a540c); };
            c.w64(8, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5410); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5414); };
            c.w64(11, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5418); };
            c.w64(7, t60);
            blk = 57; continue;
        }
        57 => {
            let Some(t61) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a541c); };
            c.w32(1, t61);
            let r62 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r62);
            let Some(t63) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5422); };
            c.w32(1, t63);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 62; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r16(1), 0x43u64, 16);
            if c.cc_e() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a5432); }
            c.w32(0, 0x1u64);
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a5438); };
            c.w64(3, t64);
            return c.call(0x700086ed74, 0x70008a5444, 0x70008a543f);
        }
        60 => {
            blk = 66; continue;
        }
        61 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5446); };
            c.w32(1, t65);
            let r66 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r66);
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        62 => {
            let Some(t67) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5451); };
            c.w32(1, t67);
            c.logic(c.r32(1) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r68 = c.r64(1) & 0xffffffu64;
            c.w64(1, r68);
            c.logic(r68, 64);
            let r69 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r69);
            c.w64(0, c.r64(1));
            blk = 66; continue;
        }
        64 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a546e); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a546f); };
            c.w64(3, t70);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a5483, 0x70008a547e);
        }
        65 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5483); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5487); };
            c.w64(6, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a548b); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a548f); };
            c.w64(9, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5493); };
            c.w64(11, t75);
            blk = 66; continue;
        }
        66 => {
            c.w64(1, c.r64(0));
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a549a); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a549e); };
            c.w64(8, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a54a2); };
            c.w64(9, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a54a6); };
            c.w64(11, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a54aa); };
            c.w64(7, t80);
            blk = 33; continue;
        }
        67 => {
            let r81 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r81);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a54b7); }
            return c.call(0x70008708b4, 0x70008a54c1, 0x70008a54bc);
        }
        68 => {
            let Some(t82) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a54c1); };
            c.w32(1, t82);
            let r83 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r83);
            blk = 39; continue;
        }
        69 => {
            return RegfileExit::jump(0x70008a54d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a54d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a54d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a54d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a54d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a54da); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a54df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a54e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a54fb); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a5503); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a5507); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a5511); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5517); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a5524); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a552a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a5531); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a553b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5541); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a5548); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5556); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a555d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a556a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a5589, 0x70008a5584);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5589); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a558d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5591); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5595); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a5599); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a55ae); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 49; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a561b); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a562c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5635); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a563c); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5644); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5651); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a566f); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a5685); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5691); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x2));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a5699); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a56a5); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 78; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a56ca); };
            c.w64(9, t32);
            let r33 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r33);
            c.w64(11, c.r64(7));
            let r34 = c.shr(c.r32(11), 0x8u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a56de); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(7)) { return RegfileExit::fault(0x70008a56e2); }
            blk = 48; continue;
        }
        47 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008a56e9); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(11)) { return RegfileExit::fault(0x70008a56ed); }
            blk = 48; continue;
        }
        48 => {
            c.w64(2, c.r64(8));
            let r35 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a56f9); };
            c.w64(0, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a5703); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a5704); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a570a); }
            return c.ret(0x0, 0x70008a570c);
        }
        49 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5712); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a5713); };
            c.w64(3, t37);
            return c.call(0x700086ed74, 0x70008a571f, 0x70008a571a);
        }
        50 => {
            return RegfileExit::fault(0x70008a571f);
        }
        51 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5725); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5726); };
            c.w64(2, t38);
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0x1e3)) else { return RegfileExit::fault(0x70008a572d); };
            c.w32(2, t39);
            let r40 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r40);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5736); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5737); };
            c.w64(3, t41);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5748, 0x70008a5743);
        }
        52 => {
            return RegfileExit::fault(0x70008a5748);
        }
        53 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a574e); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a574f); };
            c.w64(2, t42);
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0x1e3)) else { return RegfileExit::fault(0x70008a5756); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a575f); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5760); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5771, 0x70008a576c);
        }
        54 => {
            return RegfileExit::fault(0x70008a5771);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a5772); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a5778); };
            c.w64(3, t46);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a5787, 0x70008a5782);
        }
        56 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5787); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a578b); };
            c.w64(0, t48);
            blk = 1; continue;
        }
        57 => {
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5794); };
            c.logic(t49 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 36; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a57a6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 37; continue;
        }
        61 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a57be); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a57c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a57c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a57cd); }
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a57d1); };
            c.cmp(t52, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t53) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a57de); };
            c.cmp(t53, 0xbau64, 16);
            if c.cc_b() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a57ef, 0x70008a57ea);
        }
        64 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a57f7); };
            c.w32(2, t54);
            let r55 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a57fd); };
            c.cmp(t56, c.r32(2), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(9, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a580d); };
            c.w64(6, t57);
            c.w32(0, 0x1u64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5816); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a581a); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a581e); };
            c.w64(11, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5822); };
            c.w64(12, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5826); };
            c.w64(7, t62);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        67 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a582c); };
            c.w32(15, t63);
            let r64 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r64);
            let Some(t65) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5833); };
            c.w32(15, t65);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a5846); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a5848); };
            c.w64(3, t66);
            return c.call(0x700086ed74, 0x70008a5854, 0x70008a584f);
        }
        70 => {
            blk = 77; continue;
        }
        71 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5856); };
            c.w32(9, t67);
            let r68 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r68);
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        72 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5862); };
            c.w32(15, t69);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r70 = c.r64(15) & 0xffffffu64;
            c.w64(15, r70);
            c.logic(r70, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        74 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a5883); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a5885); };
            c.w64(3, t71);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a5894, 0x70008a588f);
        }
        75 => {
            c.w64(2, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5897); };
            c.w64(6, t72);
            c.w32(0, 0x1u64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a58a0); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a58a4); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a58a8); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a58ac); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a58b0); };
            c.w64(7, t77);
            blk = 76; continue;
        }
        76 => {
            c.w64(0, c.r64(2));
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a58ba); };
            c.w64(6, t78);
            c.w32(0, 0x1u64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a58c3); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a58c7); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a58cb); };
            c.w64(11, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a58cf); };
            c.w64(12, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a58d3); };
            c.w64(7, t83);
            blk = 39; continue;
        }
        78 => {
            let r84 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r84);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a58e0); }
            return c.call(0x70008708b4, 0x70008a58ea, 0x70008a58e5);
        }
        79 => {
            let Some(t85) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a58ea); };
            c.w32(7, t85);
            let r86 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r86);
            blk = 45; continue;
        }
        80 => {
            return RegfileExit::jump(0x70008a58f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a5914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a5914); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a5918); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a5919); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a591a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a591f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a5923); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a593b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a5943); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a5947); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a5951); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5957); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a5964); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a596a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a5971); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a597b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5981); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a5988); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5996); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a599d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a59aa); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a59c9, 0x70008a59c4);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a59c9); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a59cd); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a59d1); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a59d5); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a59d9); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a59ee); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 49; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a5a5b); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a5a6c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5a75); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a5a7c); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5a84); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5a91); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a5aaf); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a5ac5); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5ad1); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x2));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a5ad9); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a5ae5); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 78; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a5b0a); };
            c.w64(9, t32);
            let r33 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r33);
            c.w64(11, c.r64(7));
            let r34 = c.shr(c.r32(11), 0x8u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a5b1e); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(7)) { return RegfileExit::fault(0x70008a5b22); }
            blk = 48; continue;
        }
        47 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008a5b29); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(11)) { return RegfileExit::fault(0x70008a5b2d); }
            blk = 48; continue;
        }
        48 => {
            c.w64(2, c.r64(8));
            let r35 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5b39); };
            c.w64(0, t36);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a5b43); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a5b44); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a5b4a); }
            return c.ret(0x0, 0x70008a5b4c);
        }
        49 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5b52); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a5b53); };
            c.w64(3, t37);
            return c.call(0x700086ed74, 0x70008a5b5f, 0x70008a5b5a);
        }
        50 => {
            return RegfileExit::fault(0x70008a5b5f);
        }
        51 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5b65); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5b66); };
            c.w64(2, t38);
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(0x1e7)) else { return RegfileExit::fault(0x70008a5b6d); };
            c.w32(2, t39);
            let r40 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r40);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5b76); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5b77); };
            c.w64(3, t41);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5b88, 0x70008a5b83);
        }
        52 => {
            return RegfileExit::fault(0x70008a5b88);
        }
        53 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5b8e); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5b8f); };
            c.w64(2, t42);
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0x1e7)) else { return RegfileExit::fault(0x70008a5b96); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5b9f); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5ba0); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5bb1, 0x70008a5bac);
        }
        54 => {
            return RegfileExit::fault(0x70008a5bb1);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a5bb2); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a5bb8); };
            c.w64(3, t46);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a5bc7, 0x70008a5bc2);
        }
        56 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5bc7); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5bcb); };
            c.w64(0, t48);
            blk = 1; continue;
        }
        57 => {
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5bd4); };
            c.logic(t49 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 36; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a5be6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 37; continue;
        }
        61 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5bfe); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a5c05); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a5c09); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a5c0d); }
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5c11); };
            c.cmp(t52, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t53) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5c1e); };
            c.cmp(t53, 0xbau64, 16);
            if c.cc_b() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a5c2f, 0x70008a5c2a);
        }
        64 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5c37); };
            c.w32(2, t54);
            let r55 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r55);
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5c3d); };
            c.cmp(t56, c.r32(2), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(9, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5c4d); };
            c.w64(6, t57);
            c.w32(0, 0x1u64);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5c56); };
            c.w64(8, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5c5a); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5c5e); };
            c.w64(11, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5c62); };
            c.w64(12, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5c66); };
            c.w64(7, t62);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        67 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5c6c); };
            c.w32(15, t63);
            let r64 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r64);
            let Some(t65) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5c73); };
            c.w32(15, t65);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a5c86); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a5c88); };
            c.w64(3, t66);
            return c.call(0x700086ed74, 0x70008a5c94, 0x70008a5c8f);
        }
        70 => {
            blk = 77; continue;
        }
        71 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5c96); };
            c.w32(9, t67);
            let r68 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r68);
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        72 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a5ca2); };
            c.w32(15, t69);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r70 = c.r64(15) & 0xffffffu64;
            c.w64(15, r70);
            c.logic(r70, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        74 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a5cc3); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a5cc5); };
            c.w64(3, t71);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a5cd4, 0x70008a5ccf);
        }
        75 => {
            c.w64(2, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5cd7); };
            c.w64(6, t72);
            c.w32(0, 0x1u64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5ce0); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5ce4); };
            c.w64(1, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5ce8); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5cec); };
            c.w64(12, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5cf0); };
            c.w64(7, t77);
            blk = 76; continue;
        }
        76 => {
            c.w64(0, c.r64(2));
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5cfa); };
            c.w64(6, t78);
            c.w32(0, 0x1u64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5d03); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5d07); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a5d0b); };
            c.w64(11, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a5d0f); };
            c.w64(12, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5d13); };
            c.w64(7, t83);
            blk = 39; continue;
        }
        78 => {
            let r84 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r84);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a5d20); }
            return c.call(0x70008708b4, 0x70008a5d2a, 0x70008a5d25);
        }
        79 => {
            let Some(t85) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a5d2a); };
            c.w32(7, t85);
            let r86 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r86);
            blk = 45; continue;
        }
        80 => {
            return RegfileExit::jump(0x70008a5d38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a5d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a5d54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a5d58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a5d59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a5d5a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a5d5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a5d63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a5d7b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a5d83); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a5d87); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a5d91); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5d97); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a5da4); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5daa); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a5db1); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a5dbb); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5dc1); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a5dc8); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5dd6); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5ddd); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5dea); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a5e09, 0x70008a5e04);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a5e09); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a5e0d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a5e11); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a5e15); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a5e19); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a5e2e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 49; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a5e9b); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a5eac); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a5eb5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a5ebc); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a5ec4); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a5ed1); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a5eef); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a5f05); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a5f11); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x4));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a5f19); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a5f29); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 78; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a5f4e); };
            c.w64(9, t32);
            let r33 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r33);
            c.w64(11, c.r64(7));
            let r34 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            c.w64(12, c.r64(7));
            let r35 = c.shr(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r35);
            c.w64(15, c.r64(7));
            let r36 = c.shr(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a5f70); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(12)) { return RegfileExit::fault(0x70008a5f74); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(15)) { return RegfileExit::fault(0x70008a5f79); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(7)) { return RegfileExit::fault(0x70008a5f7e); }
            blk = 48; continue;
        }
        47 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008a5f85); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(15)) { return RegfileExit::fault(0x70008a5f89); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(12)) { return RegfileExit::fault(0x70008a5f8e); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(11)) { return RegfileExit::fault(0x70008a5f93); }
            blk = 48; continue;
        }
        48 => {
            c.w64(2, c.r64(8));
            let r37 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a5f9f); };
            c.w64(0, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a5fa9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a5faa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a5fb0); }
            return c.ret(0x0, 0x70008a5fb2);
        }
        49 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5fb8); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a5fb9); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008a5fc5, 0x70008a5fc0);
        }
        50 => {
            return RegfileExit::fault(0x70008a5fc5);
        }
        51 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5fcb); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5fcc); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008a5fd3); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5fdc); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a5fdd); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a5fee, 0x70008a5fe9);
        }
        52 => {
            return RegfileExit::fault(0x70008a5fee);
        }
        53 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a5ff4); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a5ff5); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008a5ffc); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6005); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a6006); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a6017, 0x70008a6012);
        }
        54 => {
            return RegfileExit::fault(0x70008a6017);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a6018); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a601e); };
            c.w64(3, t48);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a602d, 0x70008a6028);
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a602d); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6031); };
            c.w64(0, t50);
            blk = 1; continue;
        }
        57 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a603a); };
            c.logic(t51 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 36; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a604c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 37; continue;
        }
        61 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6064); };
            c.w32(15, t52);
            let r53 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a606b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a606f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a6073); }
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6077); };
            c.cmp(t54, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t55) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6084); };
            c.cmp(t55, 0xbau64, 16);
            if c.cc_b() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a6095, 0x70008a6090);
        }
        64 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a609d); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a60a3); };
            c.cmp(t58, c.r32(2), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(9, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a60b3); };
            c.w64(6, t59);
            c.w32(0, 0x1u64);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a60bc); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a60c0); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a60c4); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a60c8); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a60cc); };
            c.w64(7, t64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        67 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a60d2); };
            c.w32(15, t65);
            let r66 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r66);
            let Some(t67) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a60d9); };
            c.w32(15, t67);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a60ec); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a60ee); };
            c.w64(3, t68);
            return c.call(0x700086ed74, 0x70008a60fa, 0x70008a60f5);
        }
        70 => {
            blk = 77; continue;
        }
        71 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a60fc); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        72 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a6108); };
            c.w32(15, t71);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r72 = c.r64(15) & 0xffffffu64;
            c.w64(15, r72);
            c.logic(r72, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        74 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a6129); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a612b); };
            c.w64(3, t73);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a613a, 0x70008a6135);
        }
        75 => {
            c.w64(2, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a613d); };
            c.w64(6, t74);
            c.w32(0, 0x1u64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6146); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a614a); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a614e); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6152); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6156); };
            c.w64(7, t79);
            blk = 76; continue;
        }
        76 => {
            c.w64(0, c.r64(2));
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6160); };
            c.w64(6, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6169); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a616d); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6171); };
            c.w64(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6175); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6179); };
            c.w64(7, t85);
            blk = 39; continue;
        }
        78 => {
            let r86 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r86);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a6186); }
            return c.call(0x70008708b4, 0x70008a6190, 0x70008a618b);
        }
        79 => {
            let Some(t87) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a6190); };
            c.w32(7, t87);
            let r88 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r88);
            blk = 45; continue;
        }
        80 => {
            return RegfileExit::jump(0x70008a61a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a61b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a61b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a61b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a61b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a61ba); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a61bf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a61c3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a61db); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a61e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a61e7); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a61f1); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a61f7); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a6204); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a620a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a6211); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a621b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6221); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a6228); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6236); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a623d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a624a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a6269, 0x70008a6264);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6269); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a626d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6271); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6275); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a6279); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a628e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 49; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a62fb); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a630c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6315); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a631c); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6324); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6331); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a634f); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a6365); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a6371); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x4));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a6379); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 49; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6389); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 78; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(7, c.r32(7));
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a63ae); };
            c.w64(9, t32);
            let r33 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r33);
            c.w64(11, c.r64(7));
            let r34 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            c.w64(12, c.r64(7));
            let r35 = c.shr(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r35);
            c.w64(15, c.r64(7));
            let r36 = c.shr(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a63d0); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(12)) { return RegfileExit::fault(0x70008a63d4); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(15)) { return RegfileExit::fault(0x70008a63d9); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(7)) { return RegfileExit::fault(0x70008a63de); }
            blk = 48; continue;
        }
        47 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008a63e5); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(15)) { return RegfileExit::fault(0x70008a63e9); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(12)) { return RegfileExit::fault(0x70008a63ee); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(11)) { return RegfileExit::fault(0x70008a63f3); }
            blk = 48; continue;
        }
        48 => {
            c.w64(2, c.r64(8));
            let r37 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a63ff); };
            c.w64(0, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a6409); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a640a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a6410); }
            return c.ret(0x0, 0x70008a6412);
        }
        49 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6418); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a6419); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008a6425, 0x70008a6420);
        }
        50 => {
            return RegfileExit::fault(0x70008a6425);
        }
        51 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a642b); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a642c); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x70008a6433); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a643c); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a643d); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a644e, 0x70008a6449);
        }
        52 => {
            return RegfileExit::fault(0x70008a644e);
        }
        53 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6454); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a6455); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1ef)) else { return RegfileExit::fault(0x70008a645c); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6465); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a6466); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a6477, 0x70008a6472);
        }
        54 => {
            return RegfileExit::fault(0x70008a6477);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a6478); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a647e); };
            c.w64(3, t48);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a648d, 0x70008a6488);
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a648d); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6491); };
            c.w64(0, t50);
            blk = 1; continue;
        }
        57 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a649a); };
            c.logic(t51 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 36; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a64ac); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 37; continue;
        }
        61 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a64c4); };
            c.w32(15, t52);
            let r53 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a64cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a64cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a64d3); }
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a64d7); };
            c.cmp(t54, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t55) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a64e4); };
            c.cmp(t55, 0xbau64, 16);
            if c.cc_b() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a64f5, 0x70008a64f0);
        }
        64 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a64fd); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6503); };
            c.cmp(t58, c.r32(2), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(9, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6513); };
            c.w64(6, t59);
            c.w32(0, 0x1u64);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a651c); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6520); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6524); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6528); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a652c); };
            c.w64(7, t64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        67 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6532); };
            c.w32(15, t65);
            let r66 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r66);
            let Some(t67) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6539); };
            c.w32(15, t67);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a654c); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a654e); };
            c.w64(3, t68);
            return c.call(0x700086ed74, 0x70008a655a, 0x70008a6555);
        }
        70 => {
            blk = 77; continue;
        }
        71 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a655c); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        72 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a6568); };
            c.w32(15, t71);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r72 = c.r64(15) & 0xffffffu64;
            c.w64(15, r72);
            c.logic(r72, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 76; continue;
        }
        74 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a6589); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a658b); };
            c.w64(3, t73);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a659a, 0x70008a6595);
        }
        75 => {
            c.w64(2, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a659d); };
            c.w64(6, t74);
            c.w32(0, 0x1u64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a65a6); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a65aa); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a65ae); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a65b2); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a65b6); };
            c.w64(7, t79);
            blk = 76; continue;
        }
        76 => {
            c.w64(0, c.r64(2));
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a65c0); };
            c.w64(6, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a65c9); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a65cd); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a65d1); };
            c.w64(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a65d5); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a65d9); };
            c.w64(7, t85);
            blk = 39; continue;
        }
        78 => {
            let r86 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r86);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008a65e6); }
            return c.call(0x70008708b4, 0x70008a65f0, 0x70008a65eb);
        }
        79 => {
            let Some(t87) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008a65f0); };
            c.w32(7, t87);
            let r88 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r88);
            blk = 45; continue;
        }
        80 => {
            return RegfileExit::jump(0x70008a6600);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a6614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a6614); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a6618); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a6619); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a661a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a661f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a6623); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a663b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a6643); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a6647); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a6651); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6657); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a6664); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a666a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a6671); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a667b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6681); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a6688); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6696); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a669d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a66aa); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a66c9, 0x70008a66c4);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a66c9); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a66cd); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a66d1); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a66d5); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a66d9); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a66ee); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 48; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a675b); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a676c); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6775); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a677c); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6784); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6791); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a67af); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a67c5); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a67d1); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x4));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a67d9); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a67e9); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            c.cvtsd2ss(0, c.xmm_u64(0));
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a6801); };
            c.w64(7, t32);
            let r33 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r33);
            c.movd_rx(9, 0);
            c.w64(11, c.r64(9));
            let r34 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r34);
            c.w64(12, c.r64(9));
            let r35 = c.shr(c.r32(12), 0x10u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r35);
            c.w64(15, c.r64(9));
            let r36 = c.shr(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r36);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a6828); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(12)) { return RegfileExit::fault(0x70008a682c); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(15)) { return RegfileExit::fault(0x70008a6831); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(9)) { return RegfileExit::fault(0x70008a6836); }
            blk = 47; continue;
        }
        46 => {
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(9)) { return RegfileExit::fault(0x70008a683d); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(15)) { return RegfileExit::fault(0x70008a6841); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(12)) { return RegfileExit::fault(0x70008a6846); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(11)) { return RegfileExit::fault(0x70008a684b); }
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(8));
            let r37 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6857); };
            c.w64(0, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a6861); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a6862); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a6868); }
            return c.ret(0x0, 0x70008a686a);
        }
        48 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6870); }
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a6871); };
            c.w64(3, t39);
            return c.call(0x700086ed74, 0x70008a687d, 0x70008a6878);
        }
        49 => {
            return RegfileExit::fault(0x70008a687d);
        }
        50 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6883); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a6884); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x70008a688b); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6894); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a6895); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a68a6, 0x70008a68a1);
        }
        51 => {
            return RegfileExit::fault(0x70008a68a6);
        }
        52 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a68ac); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a68ad); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1f3)) else { return RegfileExit::fault(0x70008a68b4); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a68bd); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a68be); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a68cf, 0x70008a68ca);
        }
        53 => {
            return RegfileExit::fault(0x70008a68cf);
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a68d0); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a68d6); };
            c.w64(3, t48);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a68e5, 0x70008a68e0);
        }
        55 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a68e5); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a68e9); };
            c.w64(0, t50);
            blk = 1; continue;
        }
        56 => {
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a68f2); };
            c.logic(t51 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 36; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6904); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 37; continue;
        }
        60 => {
            let Some(t52) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a691c); };
            c.w32(15, t52);
            let r53 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r53);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a6923); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a6927); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a692b); }
            let Some(t54) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a692f); };
            c.cmp(t54, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t55) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a693c); };
            c.cmp(t55, 0xbau64, 16);
            if c.cc_b() { blk = 66; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a694d, 0x70008a6948);
        }
        63 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6955); };
            c.w32(2, t56);
            let r57 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a695b); };
            c.cmp(t58, c.r32(2), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(9, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a696b); };
            c.w64(6, t59);
            c.w32(0, 0x1u64);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6974); };
            c.w64(8, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6978); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a697c); };
            c.w64(11, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6980); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6984); };
            c.w64(7, t64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        66 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a698a); };
            c.w32(15, t65);
            let r66 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r66);
            let Some(t67) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6991); };
            c.w32(15, t67);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a69a4); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a69a6); };
            c.w64(3, t68);
            return c.call(0x700086ed74, 0x70008a69b2, 0x70008a69ad);
        }
        69 => {
            blk = 76; continue;
        }
        70 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a69b4); };
            c.w32(9, t69);
            let r70 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r70);
            c.w64(2, c.r64(9));
            blk = 75; continue;
        }
        71 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a69c0); };
            c.w32(15, t71);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r72 = c.r64(15) & 0xffffffu64;
            c.w64(15, r72);
            c.logic(r72, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 75; continue;
        }
        73 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a69e1); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a69e3); };
            c.w64(3, t73);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a69f2, 0x70008a69ed);
        }
        74 => {
            c.w64(2, c.r64(0));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a69f5); };
            c.w64(6, t74);
            c.w32(0, 0x1u64);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a69fe); };
            c.w64(8, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6a02); };
            c.w64(1, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6a06); };
            c.w64(11, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6a0a); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6a0e); };
            c.w64(7, t79);
            blk = 75; continue;
        }
        75 => {
            c.w64(0, c.r64(2));
            blk = 76; continue;
        }
        76 => {
            c.w64(9, c.r64(0));
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6a18); };
            c.w64(6, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6a21); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6a25); };
            c.w64(1, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6a29); };
            c.w64(11, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6a2d); };
            c.w64(12, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6a31); };
            c.w64(7, t85);
            blk = 39; continue;
        }
        77 => {
            return RegfileExit::jump(0x70008a6a3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a6a54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a6a54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a6a58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a6a59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a6a5a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a6a5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a6a63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a6a7b); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008a6a83); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008a6a87); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a6a91); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6a97); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a6aa4); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6aaa); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008a6ab1); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a6abb); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6ac1); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a6ac8); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6ad6); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6add); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6aea); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a6b09, 0x70008a6b04);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6b09); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6b0d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6b11); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6b15); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a6b19); };
            c.w64(9, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 25; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6b2e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            c.w64(11, c.r64(2));
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 48; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a6b9b); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(12, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a6bac); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6bb5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a6bbc); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6bc4); };
            c.cmp(t24, c.r32(12), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6bd1); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 56; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a6bef); };
            c.w32(12, t26);
            let r27 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r27);
            c.w32(0, 0x1u64);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a6c05); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t29) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a6c11); };
            c.w64(2, t29);
            c.w64(15, c.r64(11).wrapping_add(0x8));
            let Some(t30) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a6c19); };
            c.cmp(t30, c.r64(15), 64);
            if c.cc_b() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6c29); }
            blk = 44; continue;
        }
        43 => {
            let r31 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 44; continue;
        }
        44 => {
            c.movq_rx(7, 0);
            if let Some(sr) = c.shr(c.r64(7), 0x20u64, 64) { c.w64(7, sr); }
            let Some(t32) = c.ld64(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a6c46); };
            c.w64(9, t32);
            c.movd_rx(12, 0);
            c.w64(15, c.r64(7));
            let r33 = c.shr(c.r32(15), 0x18u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r33);
            c.w64(0, c.r64(7));
            let r34 = c.shr(c.r32(0), 0x10u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r34);
            c.w64(3, c.r64(7));
            let r35 = c.shr(c.r32(3), 0x8u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r35);
            let r36 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r36);
            c.w64(11, c.r64(12));
            let r37 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r37);
            c.w64(6, c.r64(12));
            let r38 = c.shr(c.r32(6), 0x10u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r38);
            c.w64(8, c.r64(12));
            let r39 = c.shr(c.r32(8), 0x8u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r39);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(15)) { return RegfileExit::fault(0x70008a6c7f); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(0)) { return RegfileExit::fault(0x70008a6c83); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(3)) { return RegfileExit::fault(0x70008a6c88); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(7)) { return RegfileExit::fault(0x70008a6c8d); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(11)) { return RegfileExit::fault(0x70008a6c92); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(6)) { return RegfileExit::fault(0x70008a6c97); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(8)) { return RegfileExit::fault(0x70008a6c9c); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(12)) { return RegfileExit::fault(0x70008a6ca1); }
            blk = 47; continue;
        }
        46 => {
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)), c.r8(12)) { return RegfileExit::fault(0x70008a6ca8); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(8)) { return RegfileExit::fault(0x70008a6cac); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(6)) { return RegfileExit::fault(0x70008a6cb1); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(11)) { return RegfileExit::fault(0x70008a6cb6); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(7)) { return RegfileExit::fault(0x70008a6cbb); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(3)) { return RegfileExit::fault(0x70008a6cc0); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(0)) { return RegfileExit::fault(0x70008a6cc5); }
            if !c.st8(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(15)) { return RegfileExit::fault(0x70008a6cca); }
            blk = 47; continue;
        }
        47 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6ccf); };
            c.w64(2, t40);
            let r41 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r41);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6cd7); };
            c.w64(0, t42);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a6ce1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a6ce2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a6ce8); }
            return c.ret(0x0, 0x70008a6cea);
        }
        48 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6cf0); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a6cf1); };
            c.w64(3, t43);
            return c.call(0x700086ed74, 0x70008a6cfd, 0x70008a6cf8);
        }
        49 => {
            return RegfileExit::fault(0x70008a6cfd);
        }
        50 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6d03); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a6d04); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x1f7)) else { return RegfileExit::fault(0x70008a6d0b); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6d14); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a6d15); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a6d26, 0x70008a6d21);
        }
        51 => {
            return RegfileExit::fault(0x70008a6d26);
        }
        52 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6d2c); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a6d2d); };
            c.w64(2, t48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x1f7)) else { return RegfileExit::fault(0x70008a6d34); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a6d3d); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a6d3e); };
            c.w64(3, t51);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a6d4f, 0x70008a6d4a);
        }
        53 => {
            return RegfileExit::fault(0x70008a6d4f);
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a6d50); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a6d56); };
            c.w64(3, t52);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a6d65, 0x70008a6d60);
        }
        55 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6d65); };
            c.w64(6, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6d69); };
            c.w64(0, t54);
            blk = 1; continue;
        }
        56 => {
            let Some(t55) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a6d72); };
            c.logic(t55 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 36; continue;
        }
        58 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6d84); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 37; continue;
        }
        60 => {
            let Some(t56) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6d9c); };
            c.w32(15, t56);
            let r57 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r57);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008a6da3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008a6da7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008a6dab); }
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6daf); };
            c.cmp(t58, c.r32(15), 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t59) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6dbc); };
            c.cmp(t59, 0xbau64, 16);
            if c.cc_b() { blk = 66; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a2474, 0x70008a6dcd, 0x70008a6dc8);
        }
        63 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6dd5); };
            c.w32(2, t60);
            let r61 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a6ddb); };
            c.cmp(t62, c.r32(2), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(9, c.r64(0));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6deb); };
            c.w64(6, t63);
            c.w32(0, 0x1u64);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6df4); };
            c.w64(8, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6df8); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6dfc); };
            c.w64(11, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6e00); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6e04); };
            c.w64(7, t68);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        66 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6e0a); };
            c.w32(15, t69);
            let r70 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r70);
            let Some(t71) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6e11); };
            c.w32(15, t71);
            c.cmp(c.r16(15), 0x40u64, 16);
            if c.cc_b() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r16(15), 0x43u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a6e24); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x24b8)) else { return RegfileExit::fault(0x70008a6e26); };
            c.w64(3, t72);
            return c.call(0x700086ed74, 0x70008a6e32, 0x70008a6e2d);
        }
        69 => {
            blk = 76; continue;
        }
        70 => {
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a6e34); };
            c.w32(9, t73);
            let r74 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r74);
            c.w64(2, c.r64(9));
            blk = 75; continue;
        }
        71 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a6e40); };
            c.w32(15, t75);
            c.logic(c.r32(15) & 0xe0000002u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            if let Some(sr) = c.shr(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let r76 = c.r64(15) & 0xffffffu64;
            c.w64(15, r76);
            c.logic(r76, 64);
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(9));
            blk = 75; continue;
        }
        73 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a6e61); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x22f0)) else { return RegfileExit::fault(0x70008a6e63); };
            c.w64(3, t77);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x70008a6e72, 0x70008a6e6d);
        }
        74 => {
            c.w64(2, c.r64(0));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6e75); };
            c.w64(6, t78);
            c.w32(0, 0x1u64);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6e7e); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6e82); };
            c.w64(1, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6e86); };
            c.w64(11, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6e8a); };
            c.w64(12, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6e8e); };
            c.w64(7, t83);
            blk = 75; continue;
        }
        75 => {
            c.w64(0, c.r64(2));
            blk = 76; continue;
        }
        76 => {
            c.w64(9, c.r64(0));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6e98); };
            c.w64(6, t84);
            c.w32(0, 0x1u64);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6ea1); };
            c.w64(8, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6ea5); };
            c.w64(1, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6ea9); };
            c.w64(11, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a6ead); };
            c.w64(12, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a6eb1); };
            c.w64(7, t89);
            blk = 39; continue;
        }
        77 => {
            return RegfileExit::jump(0x70008a6ebc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008a6ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008a6ed4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008a6ed8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008a6ed9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008a6eda); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008a6edf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008a6ee3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(1, c.r64(2).wrapping_add(0x18));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a6efb); };
            c.w64(7, t2);
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008a6f03); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008a6f07); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008a6f11); };
            c.w64(2, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6f17); };
            c.w64(2, t4);
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008a6f24); };
            c.w64(9, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6f2a); };
            c.w64(9, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008a6f31); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008a6f3b); };
            c.w64(1, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6f41); };
            c.w64(1, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a6f48); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a6f56); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a6f5d); };
            c.cmp(t11, 0x82du64, 16);
            if c.cc_ne() { blk = 62; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a6f6a); };
            c.cmp(t12, c.r32(2), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(2));
            blk = 17; continue;
        }
        15 => {
            c.w64(0, c.r64(2));
            return c.call(0x7000899a14, 0x70008a6f89, 0x70008a6f84);
        }
        16 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a6f89); };
            c.w64(1, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a6f8d); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a6f91); };
            c.w64(9, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a6f95); };
            c.w64(8, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a6f99); };
            c.w64(7, t17);
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            blk = 28; continue;
        }
        20 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a6fae); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 26; continue; }
            blk = 22; continue;
        }
        22 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(2, cv);
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_s() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(2) | c.r64(10);
            c.w64(2, r18);
            c.logic(r18, 64);
            blk = 25; continue;
        }
        25 => {
            blk = 28; continue;
        }
        26 => {
            c.w32(0, 0x1u64);
            blk = 56; continue;
        }
        27 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008a7015); }
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008a7019); };
            c.cmp(t19, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(11, c.r32(1));
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008a702a); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t21) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7033); };
            c.w32(11, t21);
            let r22 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r22);
            let Some(t23) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008a703a); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008a7041); };
            c.cmp(t24, c.r32(11), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a704e); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 66; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 36; continue;
        }
        35 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(1, 0x1u64);
            blk = 38; continue;
        }
        37 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        38 => {
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008a706b); };
            c.w32(11, t26);
            let r27 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r27);
            c.w32(0, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008a7077); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008a707b); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008a7089); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a7090); };
            c.cmp(t30, 0x41u64, 16);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a7099); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008a709b); };
            c.w64(3, t31);
            return c.call(0x700086ed74, 0x70008a70a7, 0x70008a70a2);
        }
        41 => {
            c.w64(9, c.r64(0));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a70aa); };
            c.w64(6, t32);
            c.w32(0, 0x1u64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a70b3); };
            c.w64(8, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008a70b7); };
            c.w64(1, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008a70bb); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008a70bf); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008a70c3); };
            c.w64(7, t37);
            blk = 42; continue;
        }
        42 => {
            let Some(t38) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008a70c7); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t39) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008a70d2); };
            c.w64(12, t39);
            c.w64(15, c.r64(2).wrapping_add(0x8));
            let Some(t40) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008a70da); };
            c.cmp(t40, c.r64(15), 64);
            if c.cc_b() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t41) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a70e4); };
            c.w32(7, t41);
            c.logic(c.r32(7) & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(9));
            blk = 47; continue;
        }
        46 => {
            let Some(t42) = c.ld64(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008a70f8); };
            c.w64(9, t42);
            c.w64(15, c.r64(9));
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.w32(9, c.r32(9));
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 47; continue;
        }
        47 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let r43 = c.neg(c.r32(9), 32);
            c.w32(9, r43);
            blk = 51; continue;
        }
        50 => {
            c.w32(7, 0xffffffffu64);
            let r44 = c.sub(c.r32(7), c.r32(9), 32);
            c.w32(7, r44);
            c.w64(9, c.r64(7));
            blk = 51; continue;
        }
        51 => {
            let r45 = c.neg(c.r32(15), 32);
            c.w32(15, r45);
            blk = 52; continue;
        }
        52 => {
            let Some(t46) = c.ld64(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008a712e); };
            c.w64(7, t46);
            c.w64(11, c.r64(9));
            let r47 = c.shr(c.r32(11), 0x18u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r47);
            c.w64(0, c.r64(9));
            let r48 = c.shr(c.r32(0), 0x10u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r48);
            c.w64(3, c.r64(9));
            let r49 = c.shr(c.r32(3), 0x8u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r49);
            c.w64(6, c.r64(15));
            let r50 = c.shr(c.r32(6), 0x18u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r50);
            c.w64(8, c.r64(15));
            let r51 = c.shr(c.r32(8), 0x10u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008a7152); }
            let r52 = c.shr(c.r32(15), 0x8u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r52);
            let r53 = c.add(c.r64(2), c.r64(12), 64);
            c.w64(2, r53);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(11)) { return RegfileExit::fault(0x70008a7162); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(0)) { return RegfileExit::fault(0x70008a7166); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(3)) { return RegfileExit::fault(0x70008a716a); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(9)) { return RegfileExit::fault(0x70008a716e); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(6)) { return RegfileExit::fault(0x70008a7173); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(8)) { return RegfileExit::fault(0x70008a7178); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(15)) { return RegfileExit::fault(0x70008a717d); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7182); };
            c.w64(1, t54);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(1)) { return RegfileExit::fault(0x70008a7186); }
            blk = 55; continue;
        }
        54 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a718c); };
            c.w64(1, t55);
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008a7190); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x1), c.r8(15)) { return RegfileExit::fault(0x70008a7193); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x2), c.r8(8)) { return RegfileExit::fault(0x70008a7198); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x3), c.r8(6)) { return RegfileExit::fault(0x70008a719d); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x4), c.r8(9)) { return RegfileExit::fault(0x70008a71a2); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x5), c.r8(3)) { return RegfileExit::fault(0x70008a71a7); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x6), c.r8(0)) { return RegfileExit::fault(0x70008a71ab); }
            if !c.st8(c.r64(7).wrapping_add(c.r64(2)).wrapping_add(0x7), c.r8(11)) { return RegfileExit::fault(0x70008a71af); }
            blk = 55; continue;
        }
        55 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008a71b4); };
            c.w64(2, t56);
            let r57 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r57);
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008a71bc); };
            c.w64(0, t58);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008a71c6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008a71c7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008a71cd); }
            return c.ret(0x0, 0x70008a71cf);
        }
        56 => {
            c.w32(2, 0x19au64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a71d5); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008a71d6); };
            c.w64(3, t59);
            return c.call(0x700086ed74, 0x70008a71e2, 0x70008a71dd);
        }
        57 => {
            return RegfileExit::fault(0x70008a71e2);
        }
        58 => {
            c.w32(2, 0x66u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a71e8); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a71e9); };
            c.w64(2, t60);
            let Some(t61) = c.ld32(c.r64(2).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x70008a71f0); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a71f9); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a71fa); };
            c.w64(3, t63);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a720b, 0x70008a7206);
        }
        59 => {
            return RegfileExit::fault(0x70008a720b);
        }
        60 => {
            c.w32(2, 0x28u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a7211); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008a7212); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a7214); };
            c.w64(3, t64);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a7225, 0x70008a7220);
        }
        61 => {
            return RegfileExit::fault(0x70008a7225);
        }
        62 => {
            c.w32(2, 0x80u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a722b); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008a722c); };
            c.w64(2, t65);
            let Some(t66) = c.ld32(c.r64(2).wrapping_add(0x1fb)) else { return RegfileExit::fault(0x70008a7233); };
            c.w32(2, t66);
            let r67 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r67);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008a723c); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008a723d); };
            c.w64(3, t68);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008a724e, 0x70008a7249);
        }
        63 => {
            return RegfileExit::fault(0x70008a724e);
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008a724f); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008a7255); };
            c.w64(3, t69);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008a7264, 0x70008a725f);
        }
        65 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008a7264); };
            c.w64(6, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008a7268); };
            c.w64(0, t71);
            blk = 1; continue;
        }
        66 => {
            let Some(t72) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008a7271); };
            c.logic(t72 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 36; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008a7283); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 36; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 37; continue;
        }
        70 => {
            return RegfileExit::jump(0x70008a729c);
        }
            _ => unreachable!(),
        }
    }
}
